//! Ported Python test-suite: variant specs validate, renderers are exact and deterministic.

use serde_json::{json, Value};
use spec2artifact::{render, spec};

fn template(kind: &str) -> Value {
    spec::spec_template(kind).expect("known kind")
}

#[test]
fn kinds_and_templates() {
    for k in spec::kinds() {
        let t = template(k);
        assert_eq!(t["kind"], json!(k));
        assert!(
            spec::validate(&t).is_empty(),
            "template for {k} must validate: {:?}",
            spec::validate(&t)
        );
    }
}

#[test]
fn records_deterministic_and_schema() {
    let mut s = template("records");
    s["n"] = json!(50);
    let a = render::render(&s).unwrap();
    let b = render::render(&s).unwrap();
    assert_eq!(a, b, "deterministic (seeded)");
    let rows: Vec<Value> = a
        .lines()
        .map(|l| serde_json::from_str(l).expect("jsonl line"))
        .collect();
    assert_eq!(rows.len(), 50);
    for r in &rows {
        let keys: Vec<&str> = r.as_object().unwrap().keys().map(String::as_str).collect();
        assert_eq!(keys, vec!["id", "name", "category", "price"]);
        assert!(s["categories"].as_array().unwrap().contains(&r["category"]));
        assert!(r["price"].is_number());
    }
}

#[test]
fn records_format_variants() {
    let mut s = template("records");
    s["n"] = json!(5);
    s["format"] = json!("jsonl");
    assert_eq!(render::render(&s).unwrap().matches('\n').count(), 4);
    s["format"] = json!("csv");
    assert!(render::render(&s).unwrap().starts_with("id,name"));
    s["format"] = json!("json");
    assert!(render::render(&s).unwrap().trim_start().starts_with('['));
}

#[test]
fn report_and_list() {
    let mut r = template("report");
    r["title"] = json!("T");
    r["sections"] = json!([{"heading": "H", "bullets": ["a", "b"]}]);
    let out = render::render(&r).unwrap();
    assert!(out.contains("# T") && out.contains("## H") && out.contains("- a"));

    let mut lst = template("list");
    lst["title"] = json!("L");
    lst["items"] = json!(["x", "y"]);
    lst["checked"] = json!(["x"]);
    let out = render::render(&lst).unwrap();
    assert!(out.contains("- [x] x") && out.contains("- [ ] y"));
}

#[test]
fn table_and_code() {
    let mut t = template("table");
    t["columns"] = json!(["id", "name"]);
    t["rows"] = json!([["1", "a"]]);
    assert!(render::render(&t).unwrap().contains("| id | name |"));

    let mut c = template("code");
    c["module"] = json!("m");
    c["functions"] = json!([{"name": "f", "args": ["a"], "doc": "d"}]);
    let out = render::render(&c).unwrap();
    assert!(out.contains("def f(a):") && out.contains("\"\"\"d\"\"\""));
}

#[test]
fn table_generated_rows_from_pools() {
    let mut t = template("table");
    t["columns"] = json!(["id", "name", "category"]);
    t["n"] = json!(10);
    t["seed"] = json!(1);
    t["pools"] = json!({"name": ["A", "B"], "category": ["x", "y"]});
    let out = render::render(&t).unwrap();
    let lines: Vec<&str> = out.lines().collect();
    // heading + blank + header + separator + 1 explicit (from the skeleton) + 10 generated
    assert_eq!(lines.len(), 15, "got: {out}");
    assert!(lines[4].starts_with("| <r1c1> | "));
    assert!(lines[5].starts_with("| 1 | "));
    assert!(lines[14].starts_with("| 10 | "));
}

#[test]
fn dataset_all_rules_and_determinism() {
    let mut d = template("dataset");
    d["n"] = json!(40);
    d["rule"] = json!({"type": "mod", "k": 3});
    d["alphabet"] = json!(["a", "b"]);
    let out = render::render(&d).unwrap();
    let rows: Vec<Value> = out
        .lines()
        .map(|l| serde_json::from_str(l).expect("jsonl line"))
        .collect();
    let alpha: Vec<String> = d["alphabet"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert_eq!(rows.len(), 40);
    for r in &rows {
        let seq = r["sequence"].as_str().unwrap();
        let sum: usize = seq
            .chars()
            .map(|c| alpha.iter().position(|s| *s == c.to_string()).unwrap())
            .sum();
        assert_eq!(sum % 3, r["target"].as_u64().unwrap() as usize);
    }
    assert_eq!(render::render(&d).unwrap(), out, "deterministic");

    // parity / contains / count rules produce 0..k labels
    for (rule, k) in [
        (json!({"type": "parity"}), 2u64),
        (json!({"type": "contains", "sym": "a"}), 2u64),
        (json!({"type": "count", "k": 3}), 3u64),
    ] {
        d["rule"] = rule;
        let rows: Vec<Value> = render::render(&d)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert!(rows.iter().all(|r| r["target"].as_u64().unwrap() < k));
    }
}

#[test]
fn prose_renders_the_plan() {
    let mut p = template("prose");
    p["title"] = json!("T");
    p["tone"] = json!("formal");
    p["paragraphs"] =
        json!([{"topic": "memory", "points": ["it is a hash chain", "it is append-only"]}]);
    let out = render::render(&p).unwrap();
    assert!(out.contains("# T") && out.contains("memory") && out.contains("hash chain"));
    assert!(spec::validate(&p).is_empty());
    assert!(!spec::validate(&json!({"kind": "prose"})).is_empty());
}

#[test]
fn validation_rejects_bad() {
    assert!(!spec::validate(&json!({"kind": "nope"})).is_empty());
    assert!(!spec::validate(&json!({"kind": "records", "n": -1})).is_empty());
    assert!(
        !spec::validate(&json!({"kind": "records", "n": 1, "categories": ["x"], "pools": {}}))
            .is_empty()
    );
    assert!(
        !spec::validate(
            &json!({"kind": "records", "n": 1, "categories": ["x"], "pools": {"x": []}})
        )
        .is_empty(),
        "empty pool must be rejected"
    );
    assert!(!spec::validate(&json!({"kind": "report"})).is_empty());
    assert!(!spec::validate(
        &json!({"kind": "dataset", "rule": {"type": "bogus"}, "alphabet": ["a"], "length": 2})
    )
    .is_empty());
    assert!(
        !spec::validate(&json!([1, 2])).is_empty(),
        "non-object spec"
    );
}

#[test]
fn library_entry_points_match_python_surface() {
    assert_eq!(
        spec::kinds(),
        vec!["code", "dataset", "list", "prose", "records", "report", "table"]
    );
    // same spec -> same artifact across library uses
    let s = template("records");
    assert_eq!(render::render(&s).unwrap(), render::render(&s).unwrap());
}
