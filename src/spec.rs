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
            "code",
            "module with imports + function signatures/docstrings",
        ),
        (
            "dataset",
            "n (sequence, target) pairs for a known rule (parity/mod/contains)",
        ),
        ("list", "a ticked list"),
        (
            "prose",
            "readable prose from a plan (topic + points per paragraph); template NLG",
        ),
        (
            "records",
            "n records with fields drawn from per-category pools",
        ),
        ("report", "title + sections with bullets"),
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
        "records" => {
            match int_at(spec, "n") {
                Some(n) if n >= 0 => {}
                _ => errs.push("records.n must be a non-negative int".into()),
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
        "table" => {
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
        other => {
            return vec![format!(
                "kind must be one of {}, got {other:?}",
                kinds().join(", ")
            )];
        }
    }
    errs
}
