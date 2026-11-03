//! Rollups: group another part's rows and aggregate — the data surface for
//! `chart` and for `table.from`. Deterministic: labels sorted ascending,
//! ties broken by label.

use serde_json::{Map, Value};

/// Exact statistics + the generated rows of one rendered part. Lives here so
/// chart/table/expr can all read the same data surface.
#[derive(Debug, Default, Clone)]
pub struct PartStats {
    pub count: u64,
    pub nums: std::collections::BTreeMap<String, f64>, // sum/min/max/mean per field
    pub top: std::collections::BTreeMap<String, String>,
    pub uniques: std::collections::BTreeMap<String, u64>,
    pub rows: Vec<Map<String, Value>>, // empty for table/code/... parts
}

impl PartStats {
    /// Numeric surface for `expr.ref(part, stat[, field])`: count + sum/min/max/mean.
    pub fn num_map(&self) -> std::collections::BTreeMap<String, f64> {
        let mut m = self.nums.clone();
        m.insert("count".to_string(), self.count as f64);
        m
    }
}

/// One aggregation over grouped rows.
#[derive(Debug, Clone, PartialEq)]
pub enum Agg {
    Count,
    Sum,
    Min,
    Max,
    Mean,
    Unique,
    Top,
}

/// Parse `count` or `stat:field` (sum/min/max/mean/unique/top need a field).
pub fn parse_agg(s: &str) -> Result<(Agg, Option<String>), String> {
    let (name, field) = match s.split_once(':') {
        Some((a, f)) => (a, Some(f.to_string())),
        None => (s, None),
    };
    let agg = match name {
        "count" => Agg::Count,
        "sum" => Agg::Sum,
        "min" => Agg::Min,
        "max" => Agg::Max,
        "mean" => Agg::Mean,
        "unique" => Agg::Unique,
        "top" => Agg::Top,
        other => {
            return Err(format!(
                "unknown stat {other:?}; have count|sum|min|max|mean|unique|top"
            ))
        }
    };
    let needs_field = agg != Agg::Count;
    if needs_field && field.is_none() {
        return Err(format!("stat {name:?} needs field=NAME"));
    }
    if !needs_field && field.is_some() {
        return Err("stat count takes no field".to_string());
    }
    Ok((agg, field))
}

/// Format an agg spec for error messages.
pub fn agg_label(agg: &Agg, field: &Option<String>) -> String {
    match (agg, field) {
        (Agg::Count, _) => "count".into(),
        (a, Some(f)) => format!("{a:?}:{f}"),
        (a, None) => format!("{a:?}"),
    }
}

/// Labels + aggregate values (parallel vectors, labels ascending).
#[derive(Debug, Clone)]
pub struct Grouping {
    pub labels: Vec<String>,
    pub values: Vec<f64>,
}

/// Group `rows` by `by` and aggregate `agg` over `field` per group.
pub fn group_rows(
    rows: &[Map<String, Value>],
    by: &str,
    agg: &Agg,
    field: &Option<String>,
) -> Result<Grouping, String> {
    if rows.is_empty() {
        return Err("no rows to group (the data part has n=0)".into());
    }
    let mut groups: std::collections::BTreeMap<String, Vec<f64>> =
        std::collections::BTreeMap::new();
    for r in rows {
        let label = match r.get(by) {
            Some(Value::String(s)) => s.clone(),
            Some(other) => other.to_string(),
            None => return Err(format!("field {by:?} missing in a row of the data part")),
        };
        let v = match agg {
            Agg::Count => 1.0,
            _ => {
                let f = field.as_deref().unwrap_or_default();
                r.get(f)
                    .and_then(|v| v.as_f64())
                    .ok_or_else(|| format!("field {f:?} is missing or not numeric"))?
            }
        };
        groups.entry(label).or_default().push(v);
    }
    let mut labels: Vec<String> = Vec::with_capacity(groups.len());
    let mut values: Vec<f64> = Vec::with_capacity(groups.len());
    for (label, vals) in groups {
        let v = match agg {
            Agg::Count => vals.len() as f64,
            Agg::Sum => vals.iter().sum(),
            Agg::Min => vals.iter().cloned().fold(f64::INFINITY, f64::min),
            Agg::Max => vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            Agg::Mean => vals.iter().sum::<f64>() / vals.len() as f64,
            Agg::Unique => vals
                .iter()
                .map(|v| v.to_bits())
                .collect::<std::collections::BTreeSet<u64>>()
                .len() as f64,
            Agg::Top => 0.0, // not numeric — handled by the caller (table top:name)
        };
        labels.push(label);
        values.push(v);
    }
    Ok(Grouping { labels, values })
}

/// `top` values per group as strings (for table cells): most frequent value.
pub fn top_value_per_group(
    rows: &[Map<String, Value>],
    by: &str,
    field: &str,
) -> Result<std::collections::BTreeMap<String, String>, String> {
    let mut groups: std::collections::BTreeMap<String, std::collections::BTreeMap<String, u64>> =
        std::collections::BTreeMap::new();
    for r in rows {
        let label = row_label(r, by)?;
        let v = match r.get(field) {
            Some(Value::String(s)) => s.clone(),
            Some(other) => other.to_string(),
            None => return Err(format!("field {field:?} missing in a row of the data part")),
        };
        *groups.entry(label).or_default().entry(v).or_insert(0) += 1;
    }
    let mut out = std::collections::BTreeMap::new();
    for (label, counts) in groups {
        let mut best: Option<(&String, u64)> = None;
        for (k, n) in &counts {
            if best.map_or(true, |(bk, bn)| *n > bn || (*n == bn && k < bk)) {
                best = Some((k, *n));
            }
        }
        out.insert(label, best.map(|(k, _)| k.clone()).unwrap_or_default());
    }
    Ok(out)
}

fn row_label(r: &Map<String, Value>, by: &str) -> Result<String, String> {
    match r.get(by) {
        Some(Value::String(s)) => Ok(s.clone()),
        Some(other) => Ok(other.to_string()),
        None => Err(format!("field {by:?} missing in a row of the data part")),
    }
}

/// `top` aggregation is textual — a separate pass (most frequent value per group).
pub fn top_per_group(
    rows: &[Map<String, Value>],
    by: &str,
    field: &str,
) -> Result<Grouping, String> {
    if rows.is_empty() {
        return Err("no rows to group (the data part has n=0)".into());
    }
    let mut groups: std::collections::BTreeMap<String, std::collections::BTreeMap<String, u64>> =
        std::collections::BTreeMap::new();
    for r in rows {
        let label = match r.get(by) {
            Some(Value::String(s)) => s.clone(),
            Some(other) => other.to_string(),
            None => return Err(format!("field {by:?} missing in a row of the data part")),
        };
        let v = match r.get(field) {
            Some(Value::String(s)) => s.clone(),
            Some(other) => other.to_string(),
            None => return Err(format!("field {field:?} missing in a row of the data part")),
        };
        *groups.entry(label).or_default().entry(v).or_insert(0) += 1;
    }
    let mut g = Grouping {
        labels: Vec::new(),
        values: Vec::new(),
    };
    for (label, counts) in groups {
        let mut best: Option<(&String, u64)> = None;
        for (k, n) in &counts {
            if best.map_or(true, |(bk, bn)| *n > bn || (*n == bn && k < bk)) {
                best = Some((k, *n));
            }
        }
        g.labels.push(label);
        g.values.push(best.map_or(0.0, |(_, n)| n as f64));
    }
    Ok(g)
}
