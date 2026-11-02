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
    let kind = spec
        .get("kind")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "spec has no string 'kind'".to_string())?;
    match kind {
        "records" => records(spec),
        "report" => Ok(report(spec)),
        "table" => table(spec),
        "code" => Ok(code(spec)),
        "list" => Ok(list(spec)),
        "dataset" => dataset(spec),
        "prose" => Ok(prose(spec)),
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

fn records(spec: &Value) -> Result<String, String> {
    let n = u64_field(spec, "n")?;
    let mut rng = Rng::from_spec(spec);
    let cats: Vec<String> = match spec.get("categories").and_then(|v| v.as_array()) {
        Some(a) if !a.is_empty() => a
            .iter()
            .map(|c| c.as_str().unwrap_or_default().to_string())
            .collect(),
        _ => vec!["default".into()],
    };
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
    let (lo, hi) = spec
        .get("price_range")
        .and_then(|v| v.as_array())
        .and_then(|a| Some((a.first()?.as_f64()?, a.get(1)?.as_f64()?)))
        .unwrap_or((0.0, 0.0));
    let dec =
        usize::try_from(spec.get("decimals").and_then(|v| v.as_u64()).unwrap_or(2)).unwrap_or(2);
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

    let mut rows: Vec<Map<String, Value>> = Vec::with_capacity(n as usize);
    for i in 1..=n {
        let c = &cats[((i - 1) % cats.len() as u64) as usize];
        let pool: Vec<Value> = pools
            .get(c)
            .and_then(|p| p.as_array())
            .cloned()
            .ok_or_else(|| format!("pool missing for category {c:?}"))?;
        if pool.is_empty() {
            return Err(format!("pool for category {c:?} is empty"));
        }
        let name = rng.choice(&pool).as_str().unwrap_or_default().to_string();
        let price = round_to(rng.uniform(lo, hi), dec);
        let mut row = Map::new();
        row.insert("id".into(), json!(i));
        row.insert("name".into(), json!(name));
        row.insert("category".into(), json!(c));
        row.insert("price".into(), json!(price));
        rows.push(row);
    }
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
fn scalar_text(v: &Value) -> String {
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

fn table(spec: &Value) -> Result<String, String> {
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

fn dataset(spec: &Value) -> Result<String, String> {
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
