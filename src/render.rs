//! spec2artifact — deterministic renderers, one per spec kind.
//!
//! Pure functions: spec -> artifact text. No network, no LLM, no randomness
//! beyond the spec's `seed`.

use crate::rng::Rng;
use serde_json::{json, Map, Value};

/// Render the artifact for a spec.
///
/// # Errors
/// Unknown kind, or a spec structurally broken beyond what [`crate::spec::validate`]
/// catches (e.g. missing required keys).
pub fn render(spec: &Value) -> Result<String, String> {
    render_with_ctx(spec, None)
}

/// Render with an optional bundle data context (chart `of` / table `from`
/// read other parts' rows through it). Standalone rendering passes `None`.
pub(crate) fn render_with_ctx(
    spec: &Value,
    ctx: Option<&crate::chart::Ctx>,
) -> Result<String, String> {
    // {expr:...} works in every artifact; {ref:...} only inside bundles
    let is_bundle = spec.get("kind").and_then(|v| v.as_str()) == Some("bundle");
    let resolved_spec;
    let spec = if is_bundle {
        spec
    } else {
        resolved_spec = crate::bundle::resolve_exprs_only(spec, None)?;
        &resolved_spec
    };
    let kind = spec
        .get("kind")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "spec has no string 'kind'".to_string())?;
    match kind {
        "records" => records(spec),
        "report" => Ok(report(spec)),
        "table" => table(spec, ctx),
        "code" => Ok(code(spec)),
        "list" => Ok(list(spec)),
        "dataset" => dataset(spec),
        "prose" => Ok(prose(spec)),
        "chart" => crate::chart::render_chart(spec, ctx),
        "bundle" => crate::bundle::render_bundle(spec),
        other => Err(format!(
            "no renderer for kind {other:?}; have {}",
            kinds().join(", ")
        )),
    }
}

/// Sorted list of renderable kinds (== spec kinds).
pub fn kinds() -> Vec<&'static str> {
    crate::spec::kinds()
}

// ------------------------------------------------------------------ helpers --

fn str_field<'a>(spec: &'a Value, key: &str, default: &'a str) -> String {
    spec.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or(default)
        .to_string()
}

fn truthy_title(spec: &Value) -> Option<String> {
    spec.get("title")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
}

fn u64_field(spec: &Value, key: &str) -> Result<u64, String> {
    spec.get(key)
        .and_then(|v| v.as_u64())
        .ok_or_else(|| format!("{key} must be a non-negative integer"))
}

/// Round an f64 to `dec` decimals through its shortest decimal form
/// (the same visible result as Python's `round(x, dec)` for these magnitudes).
fn round_to(x: f64, dec: usize) -> f64 {
    format!("{x:.dec$}").parse().unwrap_or(x)
}

/// One JSON object per line, keys in insertion order.
fn json_line(map: &Map<String, Value>) -> String {
    Value::Object(map.clone()).to_string()
}

/// rows -> jsonl / csv / indented json, per the spec's `format`.
fn emit_rows(
    rows: &[Map<String, Value>],
    fields: &[String],
    spec: &Value,
) -> Result<String, String> {
    match spec
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("jsonl")
    {
        "json" => {
            let arr: Vec<Value> = rows.iter().map(|r| Value::Object(r.clone())).collect();
            Ok(serde_json::to_string_pretty(&arr).expect("serialization of rows cannot fail"))
        }
        "csv" => {
            let mut out = fields.join(",");
            for r in rows {
                let cells: Vec<String> = fields
                    .iter()
                    .map(|f| match r.get(f) {
                        Some(Value::String(s)) => s.clone(),
                        Some(other) => other.to_string(),
                        None => String::new(),
                    })
                    .collect();
                out.push('\n');
                out.push_str(&cells.join(","));
            }
            Ok(out)
        }
        _ => Ok(rows.iter().map(json_line).collect::<Vec<_>>().join("\n")),
    }
}

// ------------------------------------------------------------------ records --

/// Numeric distributions for the records price field (and `sum_to` splitting).
mod dist {
    use super::Rng;

    /// One standard-normal draw (Box-Muller, cos form; consumes 2 uniforms).
    pub fn normal(rng: &mut Rng, mean: f64, sd: f64) -> f64 {
        let u1 = rng.random().max(1e-12);
        let u2 = rng.random();
        mean + sd * (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
    }

    /// Lognormal with the spec's `price_mean` as the true mean: mu = ln(mean) - sd^2/2,
    /// so E[X] = exp(mu + sd^2/2) = mean.
    pub fn lognormal(rng: &mut Rng, mean: f64, sd: f64) -> f64 {
        let mu = mean.max(1e-9).ln() - sd * sd / 2.0;
        normal(rng, mu, sd).exp().max(1e-9)
    }
}

/// Which numeric distribution the records price field follows.
enum PriceDist {
    Uniform(f64, f64),
    Normal(f64, f64),
    LogNormal(f64, f64),
}

impl PriceDist {
    fn from_spec(spec: &Value) -> Result<PriceDist, String> {
        let name = str_field(spec, "price_dist", "uniform");
        match name.as_str() {
            "uniform" => {
                let (lo, hi) = price_range(spec)?;
                Ok(PriceDist::Uniform(lo, hi))
            }
            "normal" => {
                let (m, s) = mean_sd(spec)?;
                Ok(PriceDist::Normal(m, s))
            }
            "lognormal" => {
                let (m, s) = mean_sd(spec)?;
                Ok(PriceDist::LogNormal(m, s))
            }
            other => Err(format!(
                "price_dist must be uniform|normal|lognormal, got {other:?}"
            )),
        }
    }

    /// One raw (non-negative) draw from the distribution.
    fn draw(&self, rng: &mut Rng) -> f64 {
        match *self {
            PriceDist::Uniform(lo, hi) if hi > lo => rng.uniform(lo, hi),
            PriceDist::Uniform(_, hi) => hi,
            PriceDist::Normal(m, s) => dist::normal(rng, m, s).max(0.0),
            PriceDist::LogNormal(m, s) => dist::lognormal(rng, m, s),
        }
    }
}

fn price_range(spec: &Value) -> Result<(f64, f64), String> {
    spec.get("price_range")
        .and_then(|v| v.as_array())
        .and_then(|a| Some((a.first()?.as_f64()?, a.get(1)?.as_f64()?)))
        .filter(|(lo, hi)| lo <= hi)
        .ok_or_else(|| "price_range must be [lo, hi] with lo <= hi".to_string())
}

fn mean_sd(spec: &Value) -> Result<(f64, f64), String> {
    let m = spec
        .get("price_mean")
        .and_then(|v| v.as_f64())
        .filter(|m| m.is_finite() && *m > 0.0)
        .ok_or_else(|| "price_dist normal|lognormal needs price_mean > 0".to_string())?;
    let s = spec
        .get("price_sd")
        .and_then(|v| v.as_f64())
        .filter(|s| s.is_finite() && *s > 0.0)
        .ok_or_else(|| "price_dist normal|lognormal needs price_sd > 0".to_string())?;
    Ok((m, s))
}

/// n non-negative rounded values whose sum is EXACTLY `total` (in integer cents:
/// draw a shape from the distribution, scale to the target, distribute the
/// rounding residual one cent at a time over the first records).
fn split_sum_to(dist: &PriceDist, n: u64, total: u64, dec: usize, rng: &mut Rng) -> Vec<f64> {
    let scale = 10_f64.powi(dec as i32);
    let target_cents = (total as f64 * scale).round() as i128;
    let raws: Vec<f64> = (0..n).map(|_| dist.draw(rng).max(1e-9)).collect();
    let raw_sum: f64 = raws.iter().sum();
    let mut cents: Vec<i128> = raws
        .iter()
        .map(|v| (v * target_cents as f64 / raw_sum).round() as i128)
        .collect();
    let mut residual = target_cents - cents.iter().sum::<i128>();
    let mut i = 0usize;
    let mut guard = 0u64;
    while residual != 0 && guard <= n + 2 {
        guard += 1;
        let k = i % cents.len();
        if residual > 0 {
            cents[k] += 1;
            residual -= 1;
        } else if cents[k] > 0 {
            cents[k] -= 1;
            residual += 1;
        }
        i += 1;
    }
    cents.iter().map(|c| *c as f64 / scale).collect()
}

/// Weighted categorical pick over `cats` (weights default 1 when absent).
fn weighted_category(
    cats: &[String],
    weights: Option<&Map<String, Value>>,
    rng: &mut Rng,
) -> usize {
    let Some(w) = weights else {
        return 0; // caller falls back to round-robin
    };
    let total: u64 = cats
        .iter()
        .map(|c| w.get(c).and_then(|v| v.as_u64()).unwrap_or(1))
        .sum();
    let mut pick = rng.below(total as usize) as u64;
    for (i, c) in cats.iter().enumerate() {
        let cw = w.get(c).and_then(|v| v.as_u64()).unwrap_or(1);
        if pick < cw {
            return i;
        }
        pick -= cw;
    }
    cats.len().saturating_sub(1)
}

/// The records rows (all built-in fields, canonical order) — exposed so bundles
/// can compute stats over the data instead of re-parsing rendered text.
pub(crate) fn records_rows(spec: &Value) -> Result<Vec<Map<String, Value>>, String> {
    let n = u64_field(spec, "n")?;
    let mut rng = Rng::from_spec(spec);
    let cats: Vec<String> = match spec.get("categories").and_then(|v| v.as_array()) {
        Some(a) if !a.is_empty() => a
            .iter()
            .map(|c| c.as_str().unwrap_or_default().to_string())
            .collect(),
        _ => vec!["default".into()],
    };
    let weights = spec.get("category_weights").and_then(|v| v.as_object());
    let pools: Map<String, Value> = match spec.get("pools").and_then(|v| v.as_object()) {
        Some(p) => p.clone(),
        None => {
            let names = spec
                .get("names")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_else(|| json!(["item"]).as_array().unwrap().clone());
            let mut map = Map::new();
            map.insert("default".into(), Value::Array(names));
            map
        }
    };
    let dist = PriceDist::from_spec(spec)?;
    let sum_to = spec.get("sum_to").and_then(|v| v.as_u64());
    let dec =
        usize::try_from(spec.get("decimals").and_then(|v| v.as_u64()).unwrap_or(2)).unwrap_or(2);

    let prices: Vec<f64> = match sum_to {
        Some(total) if n > 0 => {
            if total == 0 {
                return Err("sum_to must be a positive number".into());
            }
            split_sum_to(&dist, n, total, dec, &mut rng)
        }
        _ => (0..n).map(|_| round_to(dist.draw(&mut rng), dec)).collect(),
    };

    let mut rows: Vec<Map<String, Value>> = Vec::with_capacity(n as usize);
    for (idx, price) in prices.into_iter().enumerate() {
        let i = idx as u64 + 1;
        let c = if weights.is_some() {
            let k = weighted_category(&cats, weights, &mut rng);
            &cats[k]
        } else {
            &cats[((i - 1) % cats.len() as u64) as usize]
        };
        let pool: Vec<Value> = pools
            .get(c)
            .and_then(|p| p.as_array())
            .cloned()
            .ok_or_else(|| format!("pool missing for category {c:?}"))?;
        if pool.is_empty() {
            return Err(format!("pool for category {c:?} is empty"));
        }
        let name = rng.choice(&pool).as_str().unwrap_or_default().to_string();
        let mut row = Map::new();
        row.insert("id".into(), json!(i));
        row.insert("name".into(), json!(name));
        row.insert("category".into(), json!(c));
        row.insert("price".into(), json!(price));
        rows.push(row);
    }
    Ok(rows)
}

fn records(spec: &Value) -> Result<String, String> {
    let rows = records_rows(spec)?;
    let fields: Vec<String> = match spec.get("fields").and_then(|v| v.as_array()) {
        Some(a) => a
            .iter()
            .map(|f| f.as_str().unwrap_or_default().to_string())
            .collect(),
        None => ["id", "name", "category", "price"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    };
    // keep only requested fields, in the spec's order
    let rows: Vec<Map<String, Value>> = rows
        .into_iter()
        .map(|row| {
            fields
                .iter()
                .filter_map(|f| row.get(f).map(|v| (f.clone(), v.clone())))
                .collect::<Map<String, Value>>()
        })
        .collect();
    emit_rows(&rows, &fields, spec)
}

// ------------------------------------------------------------------- report --

fn report(spec: &Value) -> String {
    let mut out = vec![
        format!("# {}", str_field(spec, "title", "Report")),
        String::new(),
    ];
    if let Some(sections) = spec.get("sections").and_then(|v| v.as_array()) {
        for s in sections {
            out.push(format!(
                "## {}",
                s.get("heading").and_then(|h| h.as_str()).unwrap_or("")
            ));
            if let Some(bullets) = s.get("bullets").and_then(|b| b.as_array()) {
                for b in bullets {
                    out.push(format!("- {}", scalar_text(b)));
                }
            }
            out.push(String::new());
        }
    }
    if let Some(footer) = truthy_title_of_footer(spec) {
        out.push("---".into());
        out.push(footer);
    }
    join_trimmed(out)
}

fn truthy_title_of_footer(spec: &Value) -> Option<String> {
    spec.get("footer")
        .and_then(|f| f.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
}

/// Stringify a scalar the way an f-string would.
pub(crate) fn scalar_text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

fn join_trimmed(mut lines: Vec<String>) -> String {
    while matches!(lines.last(), Some(l) if l.trim().is_empty()) {
        lines.pop();
    }
    let mut s = lines.join("\n");
    s.push('\n');
    s
}

// -------------------------------------------------------------------- table --

fn table(spec: &Value, ctx: Option<&crate::chart::Ctx>) -> Result<String, String> {
    if spec.get("from").is_some() {
        return table_from_data(spec, ctx);
    }
    let cols: Vec<String> = spec
        .get("columns")
        .and_then(|v| v.as_array())
        .ok_or("table.columns must be a list")?
        .iter()
        .map(scalar_text)
        .collect();
    let mut rows: Vec<Vec<String>> = spec
        .get("rows")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().map(row_cells).collect())
        .unwrap_or_default();
    let n = spec.get("n").and_then(|v| v.as_u64()).unwrap_or(0);
    let pools = spec.get("pools").and_then(|v| v.as_object());
    if n > 0 {
        let pools = pools.ok_or("table.n > 0 requires 'pools'")?;
        let mut rng = Rng::from_spec(spec);
        for i in 1..=n {
            let mut r = vec![i.to_string()];
            for c in &cols[1..] {
                let pool = pools.get(c).and_then(|p| p.as_array());
                let cell = match pool {
                    Some(p) if !p.is_empty() => scalar_text(rng.choice(p)),
                    _ => "-".to_string(),
                };
                r.push(cell);
            }
            rows.push(r);
        }
    }
    let mut out = vec![format!("| {} |", cols.join(" | "))];
    let sep: Vec<&str> = cols.iter().map(|_| "---").collect();
    out.push(format!("|{}|", sep.join("|")));
    for r in &rows {
        out.push(format!("| {} |", r.join(" | ")));
    }
    let head = truthy_title(spec)
        .map(|t| format!("# {t}\n\n"))
        .unwrap_or_default();
    Ok(format!("{}{}\n", head, out.join("\n")))
}

/// A rollup table computed from another part's grouped rows: `table.from`.
/// Cells are exact aggregates of the data the bundle generated.
fn table_from_data(spec: &Value, ctx: Option<&crate::chart::Ctx>) -> Result<String, String> {
    use crate::rollup::{self, Agg};
    let from = spec
        .get("from")
        .and_then(|v| v.as_object())
        .ok_or("table.from must be an object {part, by, cols}")?;
    let part = from
        .get("part")
        .and_then(|v| v.as_str())
        .ok_or("table.from.part must name a data part")?;
    let rows = ctx
        .and_then(|c| c.get(part))
        .map(|st| st.rows.as_slice())
        .ok_or_else(|| {
            format!(
                "table.from.part={part:?} needs a bundle context with that part's data (standalone tables must define rows explicitly)"
            )
        })?;
    let by = from
        .get("by")
        .and_then(|v| v.as_str())
        .ok_or("table.from.by must name the grouping field")?;
    let cols = from
        .get("cols")
        .and_then(|v| v.as_object())
        .ok_or("table.from.cols must be an object of NAME -> stat[:field]")?;
    if cols.is_empty() {
        return Err("table.from.cols must not be empty".into());
    }
    let mut names: Vec<&String> = cols.keys().collect();
    names.sort();
    let mut aggs = Vec::new();
    for n in &names {
        aggs.push(rollup::parse_agg(
            cols.get(*n).and_then(|v| v.as_str()).unwrap_or(""),
        )?);
    }

    // one grouping per column (labels are identical across aggs: same rows, same by)
    let mut labels: Option<Vec<String>> = None;
    let mut columns: Vec<Vec<String>> = Vec::new();
    for (name, (agg, field)) in names.iter().zip(aggs.iter()) {
        match agg {
            Agg::Top => {
                let f = field.as_deref().unwrap_or_default();
                let m = rollup::top_value_per_group(rows, by, f)?;
                let ls: Vec<String> = m.keys().cloned().collect();
                let vals: Vec<String> = m.values().cloned().collect();
                labels = Some(ls);
                columns.push(vals);
            }
            _ => {
                let g = rollup::group_rows(rows, by, agg, field)?;
                labels = Some(g.labels.clone());
                columns.push(g.values.iter().map(|v| crate::expr::fmt_num(*v)).collect());
            }
        }
        let _ = name;
    }
    let labels = labels.ok_or("no groups produced")?;
    let n = labels.len();

    // Row order: "sort": "label" (default, ascending) or "value" (descending
    // by the column named in "sort_by", else by the first numeric column —
    // column names are the sorted keys of from.cols). Strings sort last.
    let mut order: Vec<usize> = (0..n).collect();
    match spec.get("sort").and_then(|v| v.as_str()).unwrap_or("label") {
        "value" => {
            let named = spec
                .get("sort_by")
                .and_then(|v| v.as_str())
                .and_then(|sb| names.iter().position(|n| n.as_str() == sb));
            let col_i = match named {
                Some(i) => i,
                None => columns
                    .iter()
                    .position(|c| c.iter().all(|v| v.parse::<f64>().is_ok()))
                    .unwrap_or(0),
            };
            let col = &columns[col_i];
            order.sort_by(|&a, &b| {
                let va = col[a].parse::<f64>().unwrap_or(f64::NEG_INFINITY);
                let vb = col[b].parse::<f64>().unwrap_or(f64::NEG_INFINITY);
                vb.partial_cmp(&va)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| labels[a].cmp(&labels[b]))
            });
        }
        _ => order.sort_by(|&a, &b| labels[a].cmp(&labels[b])),
    }

    let mut out = Vec::new();
    if let Some(t) = truthy_title(spec) {
        out.push(format!("# {t}"));
        out.push(String::new());
    }
    let head: Vec<String> = std::iter::once(by.to_string())
        .chain(names.iter().map(|s| s.to_string()))
        .collect();
    out.push(format!("| {} |", head.join(" | ")));
    let sep: Vec<&str> = head.iter().map(|_| "---").collect();
    out.push(format!("|{}|", sep.join("|")));
    for i in order {
        let mut cells = vec![labels[i].clone()];
        for col in &columns {
            cells.push(col[i].clone());
        }
        out.push(format!("| {} |", cells.join(" | ")));
    }
    out.push(String::new());
    Ok(out.join("\n"))
}

fn row_cells(r: &Value) -> Vec<String> {
    match r {
        Value::Array(a) => a.iter().map(scalar_text).collect(),
        other => vec![scalar_text(other)],
    }
}

// --------------------------------------------------------------------- code --

fn code(spec: &Value) -> String {
    let lang = str_field(spec, "language", "python");
    let mut out: Vec<String> = Vec::new();
    if let Some(header) = truthy_title_of_header(spec) {
        if lang == "python" {
            out.extend([format!("# {header}"), String::new()]);
        } else {
            out.extend([format!("// {header}"), String::new()]);
        }
    }
    let imports: Vec<String> = spec
        .get("imports")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().map(scalar_text).collect())
        .unwrap_or_default();
    out.extend(imports.iter().cloned());
    if !imports.is_empty() {
        out.push(String::new());
    }
    if let Some(functions) = spec.get("functions").and_then(|v| v.as_array()) {
        for f in functions {
            let name = f.get("name").and_then(|n| n.as_str()).unwrap_or("");
            let args: Vec<String> = f
                .get("args")
                .and_then(|v| v.as_array())
                .map(|a| a.iter().map(scalar_text).collect())
                .unwrap_or_default();
            let args = args.join(", ");
            let doc = f.get("doc").and_then(|d| d.as_str()).unwrap_or("");
            match lang.as_str() {
                "python" => {
                    out.extend([
                        format!("def {name}({args}):"),
                        format!("    \"\"\"{doc}\"\"\""),
                        "    raise NotImplementedError".into(),
                        String::new(),
                    ]);
                }
                "c" | "cpp" => {
                    let a = if args.is_empty() { "void" } else { &args };
                    out.extend([
                        format!("/* {doc} */"),
                        format!("int {name}({a}) {{"),
                        "    return 0;".into(),
                        "}".into(),
                        String::new(),
                    ]);
                }
                "rust" => {
                    out.extend([
                        format!("/// {doc}"),
                        format!("pub fn {name}({args}) {{"),
                        "    todo!()".into(),
                        "}".into(),
                        String::new(),
                    ]);
                }
                _ => {
                    out.extend([format!("{name}({args})  // {doc}"), String::new()]);
                }
            }
        }
    }
    join_trimmed(out)
}

fn truthy_title_of_header(spec: &Value) -> Option<String> {
    spec.get("header")
        .and_then(|h| h.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
}

// --------------------------------------------------------------------- list --

fn list(spec: &Value) -> String {
    let done: Vec<String> = spec
        .get("checked")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().map(scalar_text).collect())
        .unwrap_or_default();
    let mut out: Vec<String> = Vec::new();
    if let Some(title) = truthy_title(spec) {
        out.extend([format!("# {title}"), String::new()]);
    }
    for item in spec
        .get("items")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().map(scalar_text).collect::<Vec<_>>())
        .unwrap_or_default()
    {
        let mark = if done.contains(&item) { "x" } else { " " };
        out.push(format!("- [{mark}] {item}"));
    }
    let mut s = out.join("\n");
    s.push('\n');
    s
}

// ------------------------------------------------------------------ dataset --

/// The dataset rows (id/sequence/target) — exposed for bundle stats.
pub(crate) fn dataset_rows(spec: &Value) -> Result<Vec<Map<String, Value>>, String> {
    let n = u64_field(spec, "n")?;
    let mut rng = Rng::from_spec(spec);
    let alpha: Vec<String> = spec
        .get("alphabet")
        .and_then(|v| v.as_array())
        .ok_or("dataset.alphabet must be a non-empty list")?
        .iter()
        .map(scalar_text)
        .collect();
    if alpha.is_empty() {
        return Err("dataset.alphabet must be a non-empty list".into());
    }
    let length = usize::try_from(spec.get("length").and_then(|v| v.as_i64()).unwrap_or(0))
        .map_err(|_| "dataset.length must be a positive int".to_string())?;
    if length < 1 {
        return Err("dataset.length must be a positive int".into());
    }
    let rule = spec.get("rule").cloned().unwrap_or_else(|| json!({}));
    let rtype = rule.get("type").and_then(|t| t.as_str()).unwrap_or("mod");
    let k = rule.get("k").and_then(|v| v.as_u64()).unwrap_or(2).max(1);
    let sym = rule
        .get("sym")
        .and_then(|s| s.as_str())
        .unwrap_or(&alpha[0])
        .to_string();

    let idx = |c: &str| alpha.iter().position(|a| a == c).unwrap_or(0);
    let mut rows: Vec<Map<String, Value>> = Vec::with_capacity(n as usize);
    for i in 1..=n {
        let seq: Vec<String> = (0..length).map(|_| rng.choice(&alpha).clone()).collect();
        let sum: u64 = seq.iter().map(|c| idx(c) as u64).sum();
        let y: u64 = match rtype {
            "parity" => sum % 2,
            "mod" => sum % k,
            "contains" => u64::from(seq.iter().any(|c| c == &sym)),
            _ => seq.iter().filter(|c| **c == alpha[0]).count() as u64 % k,
        };
        let mut row = Map::new();
        row.insert("id".into(), json!(i));
        row.insert("sequence".into(), json!(seq.join("")));
        row.insert("target".into(), json!(y));
        rows.push(row);
    }
    Ok(rows)
}

fn dataset(spec: &Value) -> Result<String, String> {
    let rows = dataset_rows(spec)?;
    let fields = vec![
        "id".to_string(),
        "sequence".to_string(),
        "target".to_string(),
    ];
    emit_rows(&rows, &fields, spec)
}

// -------------------------------------------------------------------- prose --

/// Free prose assembled from a plan. Honest scope: this is TEMPLATE NLG — it renders
/// the spec's claims into readable sentences with tone-specific connectives. It never
/// invents a fact; the content is exactly what the spec supplies.
fn prose(spec: &Value) -> String {
    let tone_name = str_field(spec, "tone", "neutral");
    let tone = tone_pack(&tone_name).unwrap_or_else(|| tone_pack("neutral").unwrap());
    let mut rng = Rng::from_spec(spec);
    let mut out: Vec<String> = Vec::new();
    if let Some(title) = truthy_title(spec) {
        out.extend([format!("# {title}"), String::new()]);
    }
    if let Some(paragraphs) = spec.get("paragraphs").and_then(|v| v.as_array()) {
        for p in paragraphs {
            let topic = p.get("topic").and_then(|t| t.as_str()).unwrap_or("it");
            let mut sents: Vec<String> =
                vec![tone.lead[rng.below(tone.lead.len())].replace("{t}", topic)];
            let points: Vec<String> = p
                .get("points")
                .and_then(|v| v.as_array())
                .map(|a| a.iter().map(scalar_text).collect())
                .unwrap_or_default();
            for (i, pt) in points.iter().enumerate() {
                let pre = if i > 0 {
                    format!("{} ", tone.join[rng.below(tone.join.len())])
                } else {
                    String::new()
                };
                sents.push(format!("{}{}.", pre, pt.trim_end_matches('.')));
            }
            if rng.random() < 0.6 {
                sents.push(tone.close[rng.below(tone.close.len())].to_string());
            }
            out.extend([sents.join(" "), String::new()]);
        }
    }
    if let Some(footer) = truthy_title_of_footer(spec) {
        out.extend(["---".into(), footer]);
    }
    join_trimmed(out)
}

struct Tone {
    lead: &'static [&'static str],
    join: &'static [&'static str],
    close: &'static [&'static str],
}

fn tone_pack(name: &str) -> Option<Tone> {
    match name {
        "neutral" => Some(Tone {
            lead: &[
                "{t} is worth a moment.",
                "On {t}, a few things stand out.",
                "Consider {t}.",
            ],
            join: &["Also,", "Moreover,", "In addition,", "Beyond that,"],
            close: &[
                "That is the gist.",
                "This is the core of it.",
                "So it stands.",
            ],
        }),
        "formal" => Some(Tone {
            lead: &[
                "The matter of {t} warrants attention.",
                "With respect to {t}, several observations apply.",
            ],
            join: &["Furthermore,", "In addition,", "Consequently,"],
            close: &["This concludes the point.", "Such is the position."],
        }),
        "casual" => Some(Tone {
            lead: &[
                "So, {t} -- here's the thing.",
                "About {t}:",
                "Let's talk {t}.",
            ],
            join: &["Plus,", "And,", "Also,"],
            close: &["Anyway, that's it.", "That's the deal."],
        }),
        _ => None,
    }
}
