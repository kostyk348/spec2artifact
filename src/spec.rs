//! spec2artifact — variant specs, one per task KIND.
//!
//! The LLM fills a small SPEC for the task kind; the algebra renders the artifact.
//! The spec shape is VARIANT: each kind declares its own skeleton plus a description
//! of what the LLM is asked to fill.

use serde_json::{json, Value};

/// Sorted list of known kind names.
pub fn kinds() -> Vec<&'static str> {
    let mut names: Vec<&'static str> = kind_descriptions().iter().map(|(k, _)| *k).collect();
    names.sort_unstable();
    names
}

/// `(kind, description)` for every task kind, in declaration order.
pub fn kind_descriptions() -> &'static [(&'static str, &'static str)] {
    &[
        (
            "bundle",
            "multi-part artifact: named parts of any kind + {ref:part,stat,field} \
             cross-references — the numbers in prose are computed from the data, \
             so the artifact is self-consistent by construction",
        ),
        (
            "chart",
            "deterministic Unicode chart (bars/histogram/sparkline) from inline data              or, inside a bundle, from another part's rows",
        ),
        (
            "code",
            "module with imports + function signatures/docstrings",
        ),
        (
            "dataset",
            "n (sequence, target) pairs for a known rule (parity/mod/contains)",
        ),
        ("list", "a ticked list"),
        (
            "map",
            "seeded procedural map for game artifacts: terrain (value-noise biomes),              dungeon (rooms + corridors) or maze (perfect, single component)",
        ),
        (
            "prose",
            "readable prose from a plan (topic + points per paragraph); template NLG",
        ),
        (
            "records",
            "n records with fields drawn from per-category pools",
        ),
        ("report", "title + sections with bullets"),
        (
            "tree",
            "Unicode box-drawing tree from nested labels: project layouts, \
             org charts, tech trees",
        ),
        ("table", "columns + rows (explicit or generated from pools)"),
    ]
}

/// Description of a kind, for `kinds` listings and MCP manifests.
pub fn kind_description(kind: &str) -> Option<&'static str> {
    kind_descriptions()
        .iter()
        .find(|(k, _)| *k == kind)
        .map(|(_, d)| *d)
}

/// The VARIANT spec skeleton the LLM should fill for this task kind.
///
/// # Errors
/// Unknown kind name.
pub fn spec_template(kind: &str) -> Result<Value, String> {
    match kind {
        "bundle" => Ok(json!({
            "kind": "bundle", "seed": 42, "title": "<title>", "format": "markdown",
            "parts": {
                "orders": {
                    "kind": "records", "n": 120, "seed": 7, "format": "jsonl",
                    "fields": ["id", "name", "category", "price"],
                    "categories": ["tools", "toys"], "pools": {"tools": ["Hammer"], "toys": ["Kite"]},
                    "price_range": [5, 80], "decimals": 2
                },
                "summary": {
                    "kind": "report", "title": "Order summary",
                    "sections": [{"heading": "Totals", "bullets": [
                        "Orders analyzed: {ref:part=orders,stat=count}.",
                        "Total revenue: {ref:part=orders,stat=sum,field=price}.",
                        "Mean order: {ref:part=orders,stat=mean,field=price}.",
                        "Best seller: {ref:part=orders,stat=top,field=category}."
                    ]}]
                }
            }
        })),
        "chart" => Ok(json!({
            "kind": "chart", "title": "<title>", "chart": "bars", "width": 40,
            "data": [["<label>", 42], ["<label>", 17]]
        })),
        "code" => Ok(json!({
            "kind": "code", "language": "python", "module": "<name>",
            "imports": ["import os"],
            "functions": [{"name": "<f>", "args": ["a", "b"], "doc": "<doc>"}],
            "header": ""
        })),
        "dataset" => Ok(json!({
            "kind": "dataset", "n": 200, "seed": 42,
            "alphabet": ["a", "b", "c"], "length": 12,
            "rule": {"type": "mod", "k": 3}, "format": "jsonl"
        })),
        "list" => Ok(json!({
            "kind": "list", "title": "<title>",
            "items": ["<i1>", "<i2>"], "checked": []
        })),
        "map" => Ok(json!({
            "kind": "map", "title": "<region name>", "variant": "terrain",
            "seed": 0, "w": 40, "h": 16,
            "biomes": ["<sea>", "<beach>", "<plains>", "<forest>", "<mountains>"]
        })),
        "prose" => Ok(json!({
            "kind": "prose", "title": "<title>", "tone": "neutral", "seed": 42,
            "paragraphs": [{"topic": "<topic>", "points": ["<claim 1>", "<claim 2>"]}],
            "footer": ""
        })),
        "records" => Ok(json!({
            "kind": "records", "n": 60, "seed": 42, "format": "jsonl",
            "fields": ["id", "name", "category", "price"],
            "categories": ["tools", "toys", "books", "food"],
            "pools": {
                "tools": ["Hammer", "Wrench"], "toys": ["Robot", "Kite"],
                "books": ["Atlas", "Novel"], "food": ["Bread", "Honey"]
            },
            "price_range": [1, 999], "decimals": 2
        })),
        "report" => Ok(json!({
            "kind": "report", "title": "<title>",
            "sections": [{"heading": "<h>", "bullets": ["<b1>", "<b2>"]}],
            "footer": ""
        })),
        "tree" => Ok(json!({
            "kind": "tree", "title": "<tree title>", "root": "<root label>/",
            "nodes": [{"label": "<dir>/", "nodes": ["<leaf>", "<leaf>"]}, "<leaf>"]
        })),
        "table" => Ok(json!({
            "kind": "table", "title": "<title>",
            "columns": ["<c1>", "<c2>"],
            "rows": [["<r1c1>", "<r1c2>"]], "n": 0
        })),
        other => Err(format!(
            "unknown kind {other:?}; known: {}",
            kinds().join(", ")
        )),
    }
}

// ------------------------------------------------------------- ref cycles --

/// Find a reference cycle among bundle parts (DFS over `{ref:...}` edges).
/// Returns the cycle path, e.g. ["a", "b", "a"].
fn find_cycle(parts: &serde_json::Map<String, Value>) -> Option<Vec<String>> {
    fn dfs(
        node: &str,
        parts: &serde_json::Map<String, Value>,
        state: &mut std::collections::HashMap<String, u8>,
        stack: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        state.insert(node.to_string(), 1); // in progress
        stack.push(node.to_string());
        if let Some(p) = parts.get(node) {
            if let Ok(refs) = crate::bundle::spec_refs(p) {
                for r in refs {
                    if !parts.contains_key(&r.part) {
                        continue;
                    }
                    match state.get(&r.part) {
                        Some(1) => {
                            let pos = stack.iter().position(|n| n == &r.part).unwrap_or(0);
                            let mut cyc: Vec<String> = stack[pos..].to_vec();
                            cyc.push(r.part.clone());
                            return Some(cyc);
                        }
                        None | Some(0) => {
                            if let Some(c) = dfs(&r.part, parts, state, stack) {
                                return Some(c);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        stack.pop();
        state.insert(node.to_string(), 2); // done
        None
    }
    let mut state: std::collections::HashMap<String, u8> = std::collections::HashMap::new();
    let mut stack: Vec<String> = Vec::new();
    let mut names: Vec<&String> = parts.keys().collect();
    names.sort();
    for n in names {
        if !state.contains_key(n) {
            if let Some(c) = dfs(n, parts, &mut state, &mut stack) {
                return Some(c);
            }
        }
    }
    None
}

// --------------------------------------------------------------- validation --

fn is_list(v: Option<&Value>) -> bool {
    v.is_some_and(|v| v.is_array())
}

fn int_at(spec: &Value, key: &str) -> Option<i64> {
    spec.get(key).and_then(|v| v.as_i64())
}

/// Cheap structural validation of a spec (not of the artifact). Empty vec = valid.
pub fn validate(spec: &Value) -> Vec<String> {
    let mut errs = Vec::new();
    let Value::Object(_) = spec else {
        return vec!["spec must be an object".into()];
    };
    let Some(kind) = spec.get("kind").and_then(|v| v.as_str()) else {
        return vec![format!(
            "kind must be one of {}, got {:?}",
            kinds().join(", "),
            spec.get("kind")
        )];
    };
    match kind {
        "bundle" => {
            match spec.get("parts").and_then(|v| v.as_object()) {
                None => errs.push("bundle.parts must be an object of name -> spec".into()),
                Some(parts) if parts.is_empty() => {
                    errs.push("bundle.parts must not be empty".into())
                }
                Some(parts) => {
                    for (name, p) in parts {
                        if !p.is_object() {
                            errs.push(format!("part {name:?} must be an object"));
                            continue;
                        }
                        if p.get("kind").and_then(|v| v.as_str()) == Some("bundle") {
                            errs.push(format!("part {name:?}: bundles cannot nest bundles"));
                        }
                        for e in validate(p) {
                            errs.push(format!("part {name:?}: {e}"));
                        }
                        match crate::bundle::spec_refs(p) {
                            Err(e) => errs.push(format!("part {name:?}: {e}")),
                            Ok(refs) => {
                                for r in refs {
                                    if r.part == *name {
                                        errs.push(format!("part {name:?}: self-reference {r:?}"));
                                        continue;
                                    }
                                    match parts.get(&r.part) {
                                        None => errs.push(format!(
                                            "part {name:?} references unknown part {:?}",
                                            r.part
                                        )),
                                        Some(target) => {
                                            let tk = target
                                                .get("kind")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("");
                                            let ok = match r.stat.as_str() {
                                                "count" => {
                                                    matches!(tk, "records" | "dataset" | "table")
                                                }
                                                "sum" | "min" | "max" | "mean" | "top"
                                                | "unique" => {
                                                    matches!(tk, "records" | "dataset")
                                                        && r.field.is_some()
                                                }
                                                _ => false,
                                            };
                                            if !ok {
                                                errs.push(format!(
                                                    "part {name:?}: stat {:?} not available on part {:?} (kind {tk:?})",
                                                    r.stat, r.part
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // reference cycles (surfaces the same error the renderer would give)
                    if errs.is_empty() {
                        if let Some(cycle) = find_cycle(parts) {
                            errs.push(format!("reference cycle: {}", cycle.join(" -> ")));
                        }
                    }
                }
            }
            match spec.get("format").and_then(|v| v.as_str()) {
                None | Some("markdown") | Some("plain") => {}
                Some(other) => errs.push(format!(
                    "bundle.format must be markdown|plain, got {other:?}"
                )),
            }
        }
        "records" => {
            match int_at(spec, "n") {
                Some(n) if n >= 0 => {}
                _ => errs.push("records.n must be a non-negative int".into()),
            }
            if spec.get("sum_to").is_some() {
                match spec.get("sum_to").and_then(|v| v.as_u64()) {
                    Some(t) if t > 0 => {
                        if int_at(spec, "n") == Some(0) {
                            errs.push("sum_to needs records.n > 0".into());
                        }
                    }
                    _ => errs.push("sum_to must be a positive int".into()),
                }
            }
            match spec.get("price_dist").and_then(|v| v.as_str()) {
                None | Some("uniform") => {}
                Some("normal") | Some("lognormal") => {
                    let ok = spec
                        .get("price_mean")
                        .and_then(|v| v.as_f64())
                        .is_some_and(|m| m > 0.0)
                        && spec
                            .get("price_sd")
                            .and_then(|v| v.as_f64())
                            .is_some_and(|s| s > 0.0);
                    if !ok {
                        errs.push(
                            "price_dist normal|lognormal needs price_mean > 0 and price_sd > 0"
                                .into(),
                        );
                    }
                }
                Some(other) => errs.push(format!(
                    "price_dist must be uniform|normal|lognormal, got {other:?}"
                )),
            }
            if let Some(w) = spec.get("category_weights").and_then(|v| v.as_object()) {
                let cats: Vec<&str> = spec
                    .get("categories")
                    .and_then(|v| v.as_array())
                    .map(|a| a.iter().filter_map(|c| c.as_str()).collect())
                    .unwrap_or_default();
                let mut total = 0u64;
                for (k, v) in w {
                    match v.as_u64() {
                        Some(x) => total += x,
                        None => {
                            errs.push(format!("category_weights.{k} must be a non-negative int"))
                        }
                    }
                    if !cats.is_empty() && !cats.contains(&k.as_str()) {
                        errs.push(format!("category_weights.{k}: unknown category"));
                    }
                }
                if w.is_empty() || total == 0 {
                    errs.push("category_weights must have a positive total".into());
                }
            }
            if let (Some(cats), Some(pools)) = (
                spec.get("categories").and_then(|v| v.as_array()),
                spec.get("pools").and_then(|v| v.as_object()),
            ) {
                for c in cats {
                    let Some(name) = c.as_str() else {
                        errs.push(format!("category {c} must be a string"));
                        continue;
                    };
                    match pools.get(name).and_then(|p| p.as_array()) {
                        None => errs.push(format!("pool missing for category {name:?}")),
                        Some(pool) if pool.is_empty() => {
                            errs.push(format!("pool for category {name:?} is empty"))
                        }
                        Some(_) => {}
                    }
                }
            }
            if let Some(pr) = spec.get("price_range") {
                let ok = pr
                    .as_array()
                    .and_then(|a| {
                        let (lo, hi) = (a.first()?, a.get(1)?);
                        Some(a.len() == 2 && lo.as_f64()? <= hi.as_f64()?)
                    })
                    .unwrap_or(false);
                if !ok {
                    errs.push("price_range must be [lo, hi] with lo <= hi".into());
                }
            }
        }
        "report" => {
            if !is_list(spec.get("sections")) {
                errs.push("report.sections must be a list".into());
            }
        }
        "table" if spec.get("from").is_none() => {
            if !is_list(spec.get("columns")) {
                errs.push("table.columns must be a list".into());
            }
        }
        "code" => {
            if !is_list(spec.get("functions")) {
                errs.push("code.functions must be a list".into());
            }
        }
        "list" => {
            if !is_list(spec.get("items")) {
                errs.push("list.items must be a list".into());
            }
        }
        "dataset" => {
            let rtype = spec
                .get("rule")
                .and_then(|r| r.get("type"))
                .and_then(|t| t.as_str())
                .unwrap_or("");
            if !matches!(rtype, "mod" | "parity" | "contains" | "count") {
                errs.push("dataset.rule.type must be one of mod|parity|contains|count".into());
            }
            match spec.get("alphabet").and_then(|a| a.as_array()) {
                Some(a) if !a.is_empty() => {}
                _ => errs.push("dataset.alphabet must be a non-empty list".into()),
            }
            match int_at(spec, "length") {
                Some(n) if n >= 1 => {}
                _ => errs.push("dataset.length must be a positive int".into()),
            }
        }
        "prose" => {
            let tone = spec
                .get("tone")
                .and_then(|t| t.as_str())
                .unwrap_or("neutral");
            if !matches!(tone, "neutral" | "formal" | "casual") {
                errs.push("prose.tone must be neutral|formal|casual".into());
            }
            match spec.get("paragraphs").and_then(|p| p.as_array()) {
                None => errs.push("prose.paragraphs must be a list".into()),
                Some(paras) => {
                    for p in paras {
                        if !p.get("points").is_some_and(|v| v.is_array()) {
                            errs.push("prose paragraph needs a points list".into());
                        }
                    }
                }
            }
        }
        "chart" => {
            match spec.get("chart").and_then(|v| v.as_str()) {
                None | Some("bars") | Some("line") | Some("histogram") | Some("sparkline") => {}
                Some(other) => errs.push(format!(
                    "chart must be bars|histogram|sparkline, got {other:?}"
                )),
            }
            let has_data = spec
                .get("data")
                .and_then(|v| v.as_array())
                .map(|a| {
                    !a.is_empty()
                        && a.iter().all(|row| {
                            row.as_array()
                                .is_some_and(|p| p.len() == 2 && p[1].as_f64().is_some())
                        })
                })
                .unwrap_or(false);
            if let Some(data) = spec.get("data").and_then(|v| v.as_array()) {
                if !has_data {
                    errs.push("chart.data must be non-empty [label, number] pairs".into());
                }
                if spec.get("chart").and_then(|v| v.as_str()) == Some("line") {
                    for row in data {
                        // line accepts [x, y] numbers; string labels become the index
                        if let Some(p) = row.as_array() {
                            if p.len() == 2 && !p[0].is_string() && p[0].as_f64().is_none() {
                                errs.push("chart line.data rows must be [number, number]".into());
                            }
                        }
                    }
                }
            }
            if let Some(of) = spec.get("of").and_then(|v| v.as_object()) {
                if of.get("part").and_then(|v| v.as_str()).is_none() {
                    errs.push("chart.of.part must name a data part".into());
                }
                let ck = spec.get("chart").and_then(|v| v.as_str()).unwrap_or("bars");
                if ck == "bars" && of.get("by").and_then(|v| v.as_str()).is_none() {
                    errs.push("chart.of.by must name the grouping field (bars)".into());
                }
                if (ck == "histogram" || ck == "sparkline" || ck == "line")
                    && of.get("by").is_some()
                {
                    errs.push(format!(
                        "chart {ck:?} is a sequence — use of.field, not of.by"
                    ));
                }
                if let Some(stat) = of.get("stat").and_then(|v| v.as_str()) {
                    // of carries stat and field as separate keys — join them
                    let combined = of
                        .get("field")
                        .and_then(|v| v.as_str())
                        .map(|f| format!("{stat}:{f}"))
                        .unwrap_or_else(|| stat.to_string());
                    if let Err(e) = crate::rollup::parse_agg(&combined) {
                        errs.push(format!("chart.of: {e}"));
                    }
                }
                if (ck == "histogram" || ck == "sparkline")
                    && of.get("field").and_then(|v| v.as_str()).is_none()
                {
                    errs.push(format!("chart {ck:?} needs of.field"));
                }
            }
            if !has_data && spec.get("of").is_none() {
                errs.push(
                    "chart needs \"data\": [[label, value], ...] or \"of\": {part, by, stat, field}"
                        .into(),
                );
            }
            if spec
                .get("width")
                .is_some_and(|v| !v.is_u64() || v.as_u64() == Some(0))
            {
                errs.push("chart.width must be a positive int".into());
            }
            if spec
                .get("buckets")
                .is_some_and(|v| !v.is_u64() || v.as_u64() == Some(0))
            {
                errs.push("chart.buckets must be a positive int".into());
            }
        }
        "table" => {
            let from = spec.get("from").and_then(|v| v.as_object());
            if let Some(from) = from.as_ref() {
                if from.get("part").and_then(|v| v.as_str()).is_none() {
                    errs.push("table.from.part must name a data part".into());
                }
                if from.get("by").and_then(|v| v.as_str()).is_none() {
                    errs.push("table.from.by must name the grouping field".into());
                }
                match from.get("cols").and_then(|v| v.as_object()) {
                    None => errs
                        .push("table.from.cols must be an object of NAME -> stat[:field]".into()),
                    Some(cols) if cols.is_empty() => {
                        errs.push("table.from.cols must not be empty".into())
                    }
                    Some(cols) => {
                        for (name, v) in cols {
                            match v.as_str() {
                                None => {
                                    errs.push(format!("table.from.cols.{name} must be a string"))
                                }
                                Some(sv) => {
                                    if let Err(e) = crate::rollup::parse_agg(sv) {
                                        errs.push(format!("table.from.cols.{name}: {e}"));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match spec.get("sort").and_then(|v| v.as_str()) {
                None | Some("label") | Some("value") => {}
                Some(other) => errs.push(format!("table.sort must be label|value, got {other:?}")),
            }
            if let Some(sb) = spec.get("sort_by").and_then(|v| v.as_str()) {
                match from
                    .as_ref()
                    .and_then(|f| f.get("cols").and_then(|v| v.as_object()))
                {
                    Some(cols) if cols.contains_key(sb) => {}
                    Some(cols) => errs.push(format!(
                        "table.sort_by {sb:?} is not a column; have: {}",
                        cols.keys().cloned().collect::<Vec<_>>().join(", ")
                    )),
                    None => {}
                }
            }
        }
        "map" => {
            match spec.get("variant").and_then(|v| v.as_str()) {
                None | Some("terrain") | Some("dungeon") | Some("maze") => {}
                Some(other) => errs.push(format!(
                    "map.variant must be terrain|dungeon|maze, got {other:?}"
                )),
            }
            for k in ["w", "h", "rooms"] {
                if spec.get(k).is_some_and(|v| !v.is_null() && !v.is_u64()) {
                    errs.push(format!("map.{k} must be a positive int"));
                }
            }
            if let Some(b) = spec.get("biomes").and_then(|v| v.as_array()) {
                if b.len() != 5 || b.iter().any(|x| !x.is_string()) {
                    errs.push("map.biomes must list exactly 5 string names".into());
                }
            }
        }
        "tree" => {
            if spec
                .get("root")
                .is_some_and(|v| !v.is_null() && !v.is_string())
            {
                errs.push("tree.root must be a string".into());
            }
            fn check_nodes(v: &Value, errs: &mut Vec<String>, depth: usize) {
                if depth > 16 {
                    errs.push("tree.nodes nested deeper than 16 levels".into());
                    return;
                }
                match v {
                    Value::Array(a) => {
                        for x in a {
                            match x {
                                Value::String(_) => {}
                                Value::Object(o) => {
                                    if !o.get("label").is_some_and(|l| l.is_string()) {
                                        errs.push(
                                            "every tree node needs a string \"label\"".into(),
                                        );
                                    }
                                    if let Some(ch) = o.get("nodes") {
                                        check_nodes(ch, errs, depth + 1);
                                    }
                                }
                                _ => errs.push(
                                    "tree nodes must be strings or objects with a label".into(),
                                ),
                            }
                        }
                    }
                    _ => errs.push("tree.nodes must be a list".into()),
                }
            }
            match spec.get("nodes") {
                None => errs.push("tree needs \"nodes\"".into()),
                Some(nodes) => check_nodes(nodes, &mut errs, 0),
            }
        }
        other => {
            return vec![format!(
                "kind must be one of {}, got {other:?}",
                kinds().join(", ")
            )];
        }
    }
    // {expr:...} syntax is checked for every kind (bundle parts get this via recursion)
    if kind != "bundle" {
        if let Some(e) = check_exprs(spec) {
            errs.push(e);
        }
    }
    errs
}

/// Syntax-check every {expr:...} token (refs resolve to 0 during the check).
fn check_exprs(spec: &Value) -> Option<String> {
    fn walk(v: &Value, errs: &mut Vec<String>) {
        match v {
            Value::String(s) => {
                let mut rest = s.as_str();
                // reuse the bundle token scanner without importing private items:
                // find {expr:...} spans directly
                while let Some(start) = rest.find("{expr:") {
                    let Some(end_rel) = rest[start..].find('}') else {
                        errs.push("unclosed {{expr:...}} token (missing '}')".into());
                        break;
                    };
                    let end = start + end_rel;
                    let body = &rest[start + "{expr:".len()..end];
                    if body.contains('{') {
                        errs.push("nested braces are not allowed inside {{expr:...}}".into());
                    } else if let Err(e) = crate::expr::eval(body, &|_p, _s, _f| Ok(1.0)) {
                        errs.push(format!("expr {{{body}}}: {e}"));
                    }
                    rest = &rest[end + 1..];
                }
            }
            Value::Array(a) => a.iter().for_each(|x| walk(x, errs)),
            Value::Object(o) => o.values().for_each(|x| walk(x, errs)),
            _ => {}
        }
    }
    let mut errs = Vec::new();
    walk(spec, &mut errs);
    errs.into_iter().next()
}
