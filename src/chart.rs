//! `chart` — deterministic Unicode charts rendered from data.
//!
//! Sources: inline `"data": [[label, value], ...]`, or — inside a bundle —
//! `"of": {"part": NAME, "by": FIELD, "stat": AGG, "field": FIELD}` computed
//! over another part's generated rows. Charts therefore cite the same data as
//! the prose: a bundle's numbers and its pictures cannot disagree.

use crate::rollup::{self, Agg, Grouping, PartStats};
use serde_json::Value;
use std::collections::BTreeMap;

/// What a chart may read from the bundle: each data part's stats and rows.
pub type Ctx = BTreeMap<String, PartStats>;

/// Render a chart. `ctx` supplies other bundle parts' rows.
pub fn render_chart(spec: &Value, ctx: Option<&Ctx>) -> Result<String, String> {
    let kind = spec.get("chart").and_then(|v| v.as_str()).unwrap_or("bars");
    let grouping = grouping(spec, ctx)?;
    let width = usize::try_from(spec.get("width").and_then(|v| v.as_u64()).unwrap_or(40))
        .unwrap_or(40)
        .clamp(8, 200);
    let body = match kind {
        "bars" => bars(&grouping, width),
        "line" => {
            let height = usize::try_from(spec.get("height").and_then(|v| v.as_u64()).unwrap_or(12))
                .unwrap_or(12)
                .clamp(3, 60);
            line(&grouping, width, height)
        }
        "histogram" => {
            let buckets =
                usize::try_from(spec.get("buckets").and_then(|v| v.as_u64()).unwrap_or(10))
                    .unwrap_or(10)
                    .clamp(2, 60);
            histogram(&grouping, buckets, width)
        }
        "sparkline" => sparkline(&grouping),
        other => {
            return Err(format!(
                "chart must be bars|line|histogram|sparkline, got {other:?}"
            ))
        }
    };
    let mut out: Vec<String> = Vec::new();
    match spec.get("title").and_then(|v| v.as_str()) {
        Some(t) if !t.is_empty() => {
            out.push(t.to_string());
            out.push(String::new());
        }
        _ => {}
    }
    out.extend(body);
    Ok(out.join("\n"))
}

/// Resolve the chart's data source to a grouping.
fn grouping(spec: &Value, ctx: Option<&Ctx>) -> Result<Grouping, String> {
    if let Some(data) = spec.get("data").and_then(|v| v.as_array()) {
        if data.is_empty() {
            return Err("chart.data must not be empty".into());
        }
        let mut labels = Vec::with_capacity(data.len());
        let mut values = Vec::with_capacity(data.len());
        for row in data {
            let pair = row
                .as_array()
                .filter(|a| a.len() == 2)
                .ok_or_else(|| "chart.data rows must be [label, value] pairs".to_string())?;
            labels.push(crate::render::scalar_text(&pair[0]));
            values.push(
                pair[1]
                    .as_f64()
                    .ok_or_else(|| "chart.data values must be numbers".to_string())?,
            );
        }
        return Ok(Grouping { labels, values });
    }
    let of = spec.get("of").and_then(|v| v.as_object()).ok_or_else(|| {
        "chart needs \"data\": [[label, value], ...] or \"of\": {part, by, stat, field}".to_string()
    })?;
    let part = of
        .get("part")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "chart.of.part must name a data part".to_string())?;
    let rows = ctx
        .and_then(|c| c.get(part))
        .map(|st| st.rows.as_slice())
        .ok_or_else(|| {
            format!(
                "chart.of.part={part:?} needs a bundle context with that part's data (standalone charts must use \"data\")"
            )
        })?;
    let chart_kind = spec.get("chart").and_then(|v| v.as_str()).unwrap_or("bars");
    if chart_kind == "histogram" || chart_kind == "sparkline" || chart_kind == "line" {
        // raw sequence over one numeric field (no grouping)
        let field = of
            .get("field")
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("chart {chart_kind:?} needs of.field"))?;
        let mut values = Vec::new();
        for r in rows {
            values.push(
                r.get(field)
                    .and_then(|v| v.as_f64())
                    .ok_or_else(|| format!("field {field:?} is missing or not numeric"))?,
            );
        }
        if values.is_empty() {
            return Err("no rows to chart (the data part has n=0)".into());
        }
        return Ok(Grouping {
            labels: (0..values.len()).map(|i| i.to_string()).collect(),
            values,
        });
    }
    let by = of
        .get("by")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "chart.of.by must name the grouping field".to_string())?;
    let stat = of.get("stat").and_then(|v| v.as_str()).unwrap_or("count");
    // of carries stat and field as separate keys — join them for parse_agg
    let combined = of
        .get("field")
        .and_then(|v| v.as_str())
        .map(|f| format!("{stat}:{f}"))
        .unwrap_or_else(|| stat.to_string());
    let (agg, agg_field) = rollup::parse_agg(&combined)?;
    if agg == Agg::Top {
        return rollup::top_per_group(rows, by, agg_field.as_deref().unwrap_or_default());
    }
    rollup::group_rows(rows, by, &agg, &agg_field)
}

/// Unicode line chart: Bresenham segments ('·') with points ('●').
/// X comes from numeric labels when every label parses, else 1..n.
fn line(g: &Grouping, width: usize, height: usize) -> Vec<String> {
    let xs: Vec<f64> = if g.labels.iter().all(|l| l.parse::<f64>().is_ok()) {
        g.labels.iter().map(|l| l.parse::<f64>().unwrap()).collect()
    } else {
        (1..=g.values.len()).map(|i| i as f64).collect()
    };
    let lo = g.values.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = g.values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let xlo = xs.iter().cloned().fold(f64::INFINITY, f64::min);
    let xhi = xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let map = |x: f64, y: f64| -> (usize, usize) {
        let col = if xhi == xlo {
            0
        } else {
            ((x - xlo) / (xhi - xlo) * (width as f64 - 1.0)).round() as usize
        };
        let row = if hi == lo {
            height / 2
        } else {
            ((hi - y) / (hi - lo) * (height as f64 - 1.0)).round() as usize
        };
        (col.min(width - 1), row.min(height - 1))
    };
    let mut canvas = vec![vec![' '; width]; height];
    for pair in xs
        .iter()
        .zip(g.values.iter())
        .collect::<Vec<_>>()
        .windows(2)
    {
        let (x0, y0) = map(*pair[0].0, *pair[0].1);
        let (x1, y1) = map(*pair[1].0, *pair[1].1);
        // Bresenham
        let (mut cx, mut cy) = (x0 as isize, y0 as isize);
        let (dx, dy) = ((x1 as isize - cx).abs(), -(y1 as isize - cy).abs());
        let (sx, sy) = (if x0 < x1 { 1 } else { -1 }, if y0 < y1 { 1 } else { -1 });
        let mut err = dx + dy;
        loop {
            if (cx, cy) != (x1 as isize, y1 as isize) {
                canvas[cy as usize][cx as usize] = '·';
            }
            if cx == x1 as isize && cy == y1 as isize {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                cx += sx;
            }
            if e2 <= dx {
                err += dx;
                cy += sy;
            }
        }
    }
    for (x, y) in xs.iter().zip(g.values.iter()) {
        let (col, row) = map(*x, *y);
        canvas[row][col] = '●';
    }
    let mut out: Vec<String> = canvas.iter().map(|r| r.iter().collect()).collect();
    out.push(format!(
        "  (min {lo}, max {hi}, n {}; x: {xlo} → {xhi})",
        g.values.len()
    ));
    out.push(String::new());
    out
}

fn bars(g: &Grouping, width: usize) -> Vec<String> {
    // descending by value, ties by label — the analyst's order
    let mut idx: Vec<usize> = (0..g.labels.len()).collect();
    idx.sort_by(|&a, &b| {
        g.values[b]
            .partial_cmp(&g.values[a])
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| g.labels[a].cmp(&g.labels[b]))
    });
    let max = g
        .values
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max)
        .max(1e-9);
    let label_w = g
        .labels
        .iter()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(0)
        .min(24);
    let mut out = Vec::new();
    for i in idx {
        let label = truncate_pad(&g.labels[i], label_w);
        let fill = ((g.values[i] / max) * width as f64).round() as usize;
        out.push(format!(
            "{label} │{}{} {}",
            "█".repeat(fill),
            "░".repeat(width - fill),
            crate::expr::fmt_num(g.values[i])
        ));
    }
    out
}

fn histogram(g: &Grouping, buckets: usize, width: usize) -> Vec<String> {
    let lo = g.values.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = g.values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    if lo == hi {
        return vec![format!(
            "[{}, {}] █ (all {} values equal)",
            lo,
            hi,
            g.values.len()
        )];
    }
    let step = (hi - lo) / buckets as f64;
    let mut counts = vec![0u64; buckets];
    for v in &g.values {
        let mut b = ((v - lo) / step) as usize;
        if b >= buckets {
            b = buckets - 1;
        }
        counts[b] += 1;
    }
    let max = *counts.iter().max().unwrap_or(&1) as f64;
    let lab_w = format!("{hi:.2}").len().max(format!("{lo:.2}").len());
    let mut out = Vec::new();
    for (i, c) in counts.iter().enumerate() {
        let a = lo + step * i as f64;
        let b = if i == buckets - 1 {
            hi
        } else {
            lo + step * (i + 1) as f64
        };
        let fill = ((*c as f64 / max) * width as f64).round() as usize;
        out.push(format!(
            "{:>w$}│{}{} {}",
            format!("[{a:.2},{b:.2})"),
            "█".repeat(fill),
            "░".repeat(width - fill),
            c,
            w = lab_w + 5
        ));
    }
    out
}

fn sparkline(g: &Grouping) -> Vec<String> {
    const LEVELS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let lo = g.values.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = g.values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let line: String = if lo == hi {
        "▄".repeat(g.values.len())
    } else {
        g.values
            .iter()
            .map(|v| {
                let t = (v - lo) / (hi - lo);
                LEVELS[(t * 7.0).round() as usize % 8]
            })
            .collect()
    };
    vec![format!(
        "{line}  (min {lo}, max {hi}, n {})",
        g.values.len()
    )]
}

fn truncate_pad(s: &str, w: usize) -> String {
    let n = s.chars().count();
    if n >= w {
        s.chars().take(w).collect()
    } else {
        format!("{s}{}", " ".repeat(w - n))
    }
}
