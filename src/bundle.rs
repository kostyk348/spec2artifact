//! `bundle` — multi-part artifacts with cross-part references.
//!
//! A bundle spec is a map of named parts, each a full spec of any other kind.
//! Part strings may embed `{ref:part=NAME,stat=STAT[,field=FIELD]}` placeholders;
//! the renderer topologically orders the parts by their references, renders the
//! dependencies first, computes exact statistics over the generated DATA (never
//! over text), and substitutes the numbers. The result is **self-consistent by
//! construction**: prose and tables can never disagree with the dataset they
//! describe, because the numbers in the prose are computed from that dataset.

use crate::render::{dataset_rows, records_rows};
use crate::rng::Rng;
use crate::rollup::PartStats;
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;

/// One `{ref:...}` placeholder.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Ref {
    pub part: String,
    pub stat: String,
    pub field: Option<String>,
}

impl PartStats {
    fn get(&self, r: &Ref) -> Result<String, String> {
        match r.stat.as_str() {
            "count" => Ok(self.count.to_string()),
            "unique" => {
                let f = field(r)?;
                self.uniques
                    .get(f)
                    .map(|v| v.to_string())
                    .ok_or_else(|| format!("no stats for field {f:?}"))
            }
            other @ ("sum" | "min" | "max" | "mean") => {
                let f = field(r)?;
                self.nums
                    .get(&format!("{other}:{f}"))
                    .map(|v| fmt_num(*v))
                    .ok_or_else(|| format!("no stats for field {f:?}"))
            }
            "top" => {
                let f = field(r)?;
                self.top
                    .get(f)
                    .cloned()
                    .ok_or_else(|| format!("no stats for field {f:?}"))
            }
            other => Err(format!(
                "unknown stat {other:?}; have count|sum|min|max|mean|top|unique"
            )),
        }
    }
}

fn field(r: &Ref) -> Result<&str, String> {
    r.field
        .as_deref()
        .ok_or_else(|| format!("stat {:?} needs field=NAME", r.stat))
}

/// Format a stat number: integral values as integers, else up to 6 decimals trimmed.
pub(crate) fn fmt_num(x: f64) -> String {
    if x.is_finite() && x.fract() == 0.0 && x.abs() < 1e15 {
        return format!("{}", x as i64);
    }
    let s = format!("{x:.6}");
    let t = s.trim_end_matches('0').trim_end_matches('.');
    t.to_string()
}

/// Parse the first `{ref:...}` token in `s`; `None` if there is none.
/// Errors on a malformed or unclosed token.
pub(crate) fn first_ref(s: &str) -> Result<Option<(Ref, usize, usize)>, String> {
    let Some(start) = s.find("{ref:") else {
        return Ok(None);
    };
    let Some(end_rel) = s[start..].find('}') else {
        return Err("unclosed {{ref:...}} token (missing '}')".into());
    };
    let end = start + end_rel;
    let body = &s[start + "{ref:".len()..end];
    let mut part = None;
    let mut stat = None;
    let mut field = None;
    for kv in body.split(',') {
        let Some((k, v)) = kv.split_once('=') else {
            return Err(format!("bad {{ref:...}} fragment {kv:?} (want key=value)"));
        };
        match k.trim() {
            "part" => part = Some(v.trim().to_string()),
            "stat" => stat = Some(v.trim().to_string()),
            "field" => field = Some(v.trim().to_string()),
            other => return Err(format!("unknown ref key {other:?}; have part|stat|field")),
        }
    }
    Ok(Some((
        Ref {
            part: part.ok_or("ref missing part=NAME")?,
            stat: stat.ok_or("ref missing stat=STAT")?,
            field,
        },
        start,
        end,
    )))
}

enum Token {
    Ref(Ref, usize, usize),
    Expr(String, usize, usize),
}

/// The next `{ref:...}` or `{expr:...}` token in `s`.
fn next_token(s: &str) -> Result<Option<Token>, String> {
    let rpos = s.find("{ref:");
    let epos = s.find("{expr:");
    match (rpos, epos) {
        (None, None) => Ok(None),
        (r, e)
            if match (r, e) {
                (Some(rp), Some(ep)) => rp < ep,
                (Some(_), None) => true,
                _ => false,
            } =>
        {
            let (t, start, end) = first_ref(s)?.expect("position already found");
            Ok(Some(Token::Ref(t, start, end)))
        }
        _ => {
            let start = epos.expect("position already found");
            let Some(end_rel) = s[start..].find('}') else {
                return Err("unclosed {{expr:...}} token (missing '}')".into());
            };
            let end = start + end_rel;
            let body = s[start + "{expr:".len()..end].to_string();
            if body.contains('{') {
                return Err("nested braces are not allowed inside {{expr:...}}".into());
            }
            Ok(Some(Token::Expr(body, start, end)))
        }
    }
}

/// The numeric surface expressions may read: part -> stat[:field] -> value.
fn expr_lookup(
    stats: &BTreeMap<String, PartStats>,
) -> impl Fn(&str, &str, Option<&str>) -> Result<f64, String> + '_ {
    move |part, stat, field| {
        let key = match field {
            Some(f) => format!("{stat}:{f}"),
            None => stat.to_string(),
        };
        stats
            .get(part)
            .map(|st| st.num_map())
            .and_then(|m| m.get(&key).copied())
            .ok_or_else(|| format!("ref({part:?}, {stat:?}) has no value"))
    }
}

/// Resolve ONLY {expr:...} tokens (standalone rendering: refs need a context).
/// `stats` is None outside a bundle — any ref(...) call then errors.
pub(crate) fn resolve_exprs_only(
    spec: &Value,
    stats: Option<&BTreeMap<String, PartStats>>,
) -> Result<Value, String> {
    match spec {
        Value::String(s) => {
            let mut out = String::new();
            let mut rest = s.as_str();
            while let Some(tok) = next_token(rest)? {
                let (body, start, end) = match tok {
                    Token::Expr(b, st, en) => (b, st, en),
                    Token::Ref(r, _, _) => {
                        return Err(format!(
                            "{{ref:part={},...}} needs a bundle data context (standalone artifacts have no data to reference)",
                            r.part
                        ));
                    }
                };
                let lookup = |p: &str, stat: &str, field: Option<&str>| -> Result<f64, String> {
                    let key = match field {
                        Some(f) => format!("{stat}:{f}"),
                        None => stat.to_string(),
                    };
                    stats
                        .and_then(|m| m.get(p).map(|st| st.num_map()))
                        .and_then(|m| m.get(&key).copied())
                        .ok_or_else(|| {
                            format!(
                                "ref({p:?}, {stat:?}) has no value (standalone artifacts have no bundle data)"
                            )
                        })
                };
                let v = crate::expr::eval(&body, &lookup)
                    .map_err(|e| format!("expr {{{body}}}: {e}"))?;
                out.push_str(&rest[..start]);
                out.push_str(&crate::expr::fmt_num(v));
                rest = &rest[end + 1..];
            }
            out.push_str(rest);
            Ok(Value::String(out))
        }
        Value::Array(a) => {
            let mut out = Vec::with_capacity(a.len());
            for x in a {
                out.push(resolve_exprs_only(x, stats)?);
            }
            Ok(Value::Array(out))
        }
        Value::Object(o) => {
            let mut out = Map::new();
            for (k, x) in o {
                out.insert(k.clone(), resolve_exprs_only(x, stats)?);
            }
            Ok(Value::Object(out))
        }
        other => Ok(other.clone()),
    }
}

/// Every ref in every string of the spec (recursively).
pub(crate) fn spec_refs(spec: &Value) -> Result<Vec<Ref>, String> {
    let mut out = Vec::new();
    walk(spec, &mut out)?;
    Ok(out)
}

fn walk(v: &Value, out: &mut Vec<Ref>) -> Result<(), String> {
    match v {
        Value::String(s) => {
            let mut rest = s.as_str();
            while let Some((r, _, end)) = first_ref(rest)? {
                out.push(r);
                rest = &rest[end + 1..];
            }
            Ok(())
        }
        Value::Array(a) => {
            for x in a {
                walk(x, out)?;
            }
            Ok(())
        }
        Value::Object(o) => {
            for x in o.values() {
                walk(x, out)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

/// Replace every `{ref:...}` in every string of the spec (in place, deep clone).
pub(crate) fn resolve(
    spec: &Value,
    stats: &std::collections::BTreeMap<String, PartStats>,
) -> Result<Value, String> {
    match spec {
        Value::String(s) => {
            let mut out = String::new();
            let mut rest = s.as_str();
            while let Some(tok) = next_token(rest)? {
                match tok {
                    Token::Ref(r, start, end) => {
                        let Some(st) = stats.get(&r.part) else {
                            return Err(format!(
                                "ref to part {:?} has no stats (bug in topo order)",
                                r.part
                            ));
                        };
                        out.push_str(&rest[..start]);
                        out.push_str(&st.get(&r)?);
                        rest = &rest[end + 1..];
                    }
                    Token::Expr(body, start, end) => {
                        let lookup = expr_lookup(stats);
                        let v = crate::expr::eval(&body, &lookup)
                            .map_err(|e| format!("expr {{{body}}}: {e}"))?;
                        out.push_str(&rest[..start]);
                        out.push_str(&crate::expr::fmt_num(v));
                        rest = &rest[end + 1..];
                    }
                }
            }
            out.push_str(rest);
            Ok(Value::String(out))
        }
        Value::Array(a) => {
            let mut out = Vec::with_capacity(a.len());
            for x in a {
                out.push(resolve(x, stats)?);
            }
            Ok(Value::Array(out))
        }
        Value::Object(o) => {
            let mut out = Map::new();
            for (k, x) in o {
                out.insert(k.clone(), resolve(x, stats)?);
            }
            Ok(Value::Object(out))
        }
        other => Ok(other.clone()),
    }
}

/// Stats for one rendered part, computed from its structured rows.
pub(crate) fn part_stats(kind: &str, spec: &Value) -> Result<PartStats, String> {
    let rows: Vec<Map<String, Value>> = match kind {
        "records" => records_rows(spec)?,
        "dataset" => dataset_rows(spec)?,
        "table" => {
            let explicit = spec
                .get("rows")
                .and_then(|v| v.as_array())
                .map(|a| a.len() as u64)
                .unwrap_or(0);
            return Ok(PartStats {
                count: explicit.max(spec.get("n").and_then(|v| v.as_u64()).unwrap_or(0)),
                ..PartStats::default()
            });
        }
        "chart" => {
            return Err("chart exposes no stats; reference a records/dataset/table part".into())
        }
        other => {
            return Err(format!(
                "kind {other:?} exposes no stats; reference a records/dataset/table part"
            ))
        }
    };
    let mut st = PartStats {
        count: rows.len() as u64,
        rows: rows.clone(),
        ..PartStats::default()
    };
    if rows.is_empty() {
        return Ok(st);
    }
    let fields: Vec<String> = rows[0]
        .keys()
        .filter(|k| k.as_str() != "id")
        .cloned()
        .collect();
    for f in fields {
        let vals: Vec<f64> = rows
            .iter()
            .filter_map(|r| r.get(&f).and_then(|v| v.as_f64()))
            .collect();
        if vals.len() == rows.len() && !vals.is_empty() {
            let sum: f64 = vals.iter().sum();
            st.nums.insert(format!("sum:{f}"), sum);
            st.nums.insert(
                format!("min:{f}"),
                vals.iter().cloned().fold(f64::INFINITY, f64::min),
            );
            st.nums.insert(
                format!("max:{f}"),
                vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            );
            st.nums.insert(format!("mean:{f}"), sum / vals.len() as f64);
        }
        let mut counts: BTreeMap<String, u64> = BTreeMap::new();
        for r in &rows {
            let key = match r.get(&f) {
                Some(Value::String(s)) => s.clone(),
                Some(other) => other.to_string(),
                None => continue,
            };
            *counts.entry(key).or_insert(0) += 1;
        }
        if let Some((best, n)) = counts.iter().next() {
            let mut best = (best.clone(), *n);
            for (k, v) in &counts {
                if *v > best.1 || (*v == best.1 && k.as_str() < best.0.as_str()) {
                    best = (k.clone(), *v);
                }
            }
            st.top.insert(f.clone(), best.0);
            st.uniques.insert(f, counts.len() as u64);
        }
    }
    Ok(st)
}

/// Render a bundle: topo order the parts by refs, render dependencies first,
/// resolve placeholders against exact data stats.
pub(crate) fn render_bundle(spec: &Value) -> Result<String, String> {
    let parts = spec
        .get("parts")
        .and_then(|v| v.as_object())
        .ok_or("bundle.parts must be an object of name -> spec")?;
    if parts.is_empty() {
        return Err("bundle.parts must not be empty".into());
    }
    for (name, p) in parts {
        if p.get("kind").and_then(|v| v.as_str()) == Some("bundle") {
            return Err(format!("part {name:?}: bundles cannot nest bundles"));
        }
        if p.get("kind").and_then(|v| v.as_str()).is_none() {
            return Err(format!("part {name:?} has no kind"));
        }
    }
    // seed inheritance: a part without its own seed derives one deterministically
    let bundle_seed = spec.get("seed").and_then(|v| v.as_u64()).unwrap_or(0);
    let parts: Map<String, Value> = parts
        .iter()
        .map(|(name, p)| {
            if p.get("seed").is_some() {
                return Ok((name.clone(), p.clone()));
            }
            let mut p = p.clone();
            p["seed"] = json!(derive_seed(bundle_seed, name));
            Ok((name.clone(), p))
        })
        .collect::<Result<_, String>>()?;

    let order = topo_order(&parts)?;
    let mut stats: BTreeMap<String, PartStats> = BTreeMap::new();
    let mut bodies: Vec<(String, String, String)> = Vec::new(); // (name, kind, rendered)
    for name in &order {
        let p = &parts[name];
        // stats/rows must exist BEFORE any part that reads this one (refs,
        // exprs, charts, rollup tables) is rendered; compute them from the
        // (possibly resolved) spec, then render with the current data context
        let resolved = resolve(p, &stats)?;
        let kind = resolved
            .get("kind")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        if supplies(&parts, name) {
            stats.insert(name.clone(), part_stats(&kind, &resolved)?);
        }
        let body = crate::render::render_with_ctx(&resolved, Some(&stats))?;
        bodies.push((name.clone(), kind, body));
    }
    emit(spec, &bodies)
}

/// Does any other part read `name` — via `{ref:...}`, inside an `{expr:...}`
/// body's `ref('name', ...)`, or as a chart/table data source (`of`/`from`)?
fn supplies(parts: &Map<String, Value>, name: &str) -> bool {
    parts.iter().any(|(other, p)| {
        other != name
            && all_deps(p)
                .map(|ds| ds.contains(&name.to_string()))
                .unwrap_or(false)
    })
}

/// Every part name another part's strings reference: {ref:...} targets, parts
/// named in ref('...') inside {expr:...} bodies, and of/from.part data sources.
pub(crate) fn all_deps(spec: &Value) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = spec_refs(spec)?.into_iter().map(|r| r.part).collect();
    out.extend(expr_deps(spec)?);
    out.extend(data_deps(spec));
    Ok(out)
}

/// Parts named in ref('part', ...) calls inside {expr:...} strings.
fn expr_deps(spec: &Value) -> Result<Vec<String>, String> {
    fn scan_str(s: &str, out: &mut Vec<String>) -> Result<(), String> {
        let mut rest = s;
        while let Some(tok) = next_token(rest)? {
            if let Token::Expr(body, _, end) = tok {
                let b = body.as_bytes();
                let mut i = 0;
                while i + 4 <= b.len() {
                    if &b[i..i + 4] == b"ref(" {
                        let mut j = i + 4;
                        while j < b.len() && b[j].is_ascii_whitespace() {
                            j += 1;
                        }
                        if j < b.len() && (b[j] == b'\'' || b[j] == b'"') {
                            let q = b[j];
                            let mut k = j + 1;
                            while k < b.len() && b[k] != q {
                                k += 1;
                            }
                            if k < b.len() {
                                out.push(String::from_utf8_lossy(&b[j + 1..k]).into_owned());
                            }
                        }
                        i = j;
                    } else {
                        i += 1;
                    }
                }
                rest = &rest[end + 1..];
            } else {
                break;
            }
        }
        Ok(())
    }
    fn walk(v: &Value, out: &mut Vec<String>) -> Result<(), String> {
        match v {
            Value::String(s) => scan_str(s, out),
            Value::Array(a) => {
                for x in a {
                    walk(x, out)?;
                }
                Ok(())
            }
            Value::Object(o) => {
                for x in o.values() {
                    walk(x, out)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
    let mut out = Vec::new();
    walk(spec, &mut out)?;
    Ok(out)
}

/// `of`/`from` data sources (chart.of.part, table.from.part) at any depth.
fn data_deps(spec: &Value) -> Vec<String> {
    fn walk(v: &Value, out: &mut Vec<String>) {
        match v {
            Value::Object(o) => {
                for key in ["of", "from"] {
                    if let Some(src) = o.get(key).and_then(|x| x.as_object()) {
                        if let Some(p) = src.get("part").and_then(|x| x.as_str()) {
                            out.push(p.to_string());
                        }
                    }
                }
                for x in o.values() {
                    walk(x, out);
                }
            }
            Value::Array(a) => {
                for x in a {
                    walk(x, out);
                }
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    walk(spec, &mut out);
    out
}

/// Deterministic seed for a part that doesn't carry its own (SplitMix64 over
/// the FNV-1a of the name, mixed with the bundle seed).
fn derive_seed(bundle_seed: u64, name: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in name.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x1000_0000_01b3);
    }
    let mut r = Rng::new(bundle_seed ^ h);
    r.next_u64()
}

fn topo_order(parts: &Map<String, Value>) -> Result<Vec<String>, String> {
    let mut deps: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (name, p) in parts {
        let mut ds: Vec<String> = all_deps(p)?;
        for d in &ds {
            if d == name {
                return Err(format!("reference cycle: {name} -> {name}"));
            }
            if !parts.contains_key(d) {
                return Err(format!(
                    "part {name:?} references unknown part {d:?}; have: {}",
                    parts.keys().cloned().collect::<Vec<_>>().join(", ")
                ));
            }
        }
        ds.sort();
        ds.dedup();
        deps.insert(name.clone(), ds);
    }
    let mut order = Vec::new();
    let mut ready: Vec<String> = deps
        .iter()
        .filter(|(_, ds)| ds.is_empty())
        .map(|(n, _)| n.clone())
        .collect();
    ready.sort();
    let mut remaining: BTreeMap<String, Vec<String>> = deps.clone();
    while !ready.is_empty() {
        let name = ready.remove(0);
        remaining.remove(&name);
        order.push(name.clone());
        // bump dependents
        let mut newly: Vec<String> = Vec::new();
        for (n, ds) in remaining.iter_mut() {
            ds.retain(|d| d != &name);
            if ds.is_empty() {
                newly.push(n.clone());
            }
        }
        for n in newly {
            remaining.remove(&n);
            let pos = ready.binary_search(&n).unwrap_or_else(|i| i);
            ready.insert(pos, n);
        }
    }
    if !remaining.is_empty() {
        let mut names: Vec<&String> = remaining.keys().collect();
        names.sort();
        let chain: Vec<String> = names.iter().map(|n| n.to_string()).collect();
        return Err(format!("reference cycle: {}", chain.join(" -> ")));
    }
    Ok(order)
}

fn emit(spec: &Value, bodies: &[(String, String, String)]) -> Result<String, String> {
    let fmt = spec
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("markdown");
    match fmt {
        "plain" => {
            let mut out = Vec::new();
            for (_, _, body) in bodies {
                out.push(body.clone());
            }
            Ok(out.join("\n\n---\n\n"))
        }
        _ => {
            let mut out: Vec<String> = Vec::new();
            if let Some(t) = spec.get("title").and_then(|v| v.as_str()) {
                if !t.is_empty() {
                    out.push(format!("# {t}"));
                    out.push(String::new());
                }
            }
            for (name, kind, body) in bodies {
                out.push(format!("## {name}"));
                out.push(String::new());
                let tag = spec
                    .get("parts")
                    .and_then(|p| p.get(name))
                    .and_then(|p| fence_lang_for(kind, p));
                match tag {
                    Some(tag) => {
                        out.push(format!("```{tag}"));
                        out.push(body.clone());
                        out.push("```".into());
                    }
                    None => out.push(body.clone()),
                }
                out.push(String::new());
            }
            Ok(join_trimmed_b(out))
        }
    }
}

/// Code-fence language for a part (data formats and code; markdown kinds are inline).
fn fence_lang_for(kind: &str, p: &Value) -> Option<String> {
    match kind {
        "records" | "dataset" => Some(
            p.get("format")
                .and_then(|v| v.as_str())
                .unwrap_or("jsonl")
                .to_string(),
        ),
        "code" => Some(
            p.get("language")
                .and_then(|v| v.as_str())
                .unwrap_or("python")
                .to_string(),
        ),
        _ => None,
    }
}

fn join_trimmed_b(mut lines: Vec<String>) -> String {
    while matches!(lines.last(), Some(l) if l.trim().is_empty()) {
        lines.pop();
    }
    let mut s = lines.join("\n");
    s.push('\n');
    s
}
