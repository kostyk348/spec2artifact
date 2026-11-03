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
        vec!["bundle", "code", "dataset", "list", "prose", "records", "report", "table"]
    );
    // same spec -> same artifact across library uses
    let s = template("records");
    assert_eq!(render::render(&s).unwrap(), render::render(&s).unwrap());
}

// ------------------------------------------------------------------ bundle --

fn records_sum(spec: &Value) -> f64 {
    let rows: Vec<Value> = render::render(spec)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .collect();
    rows.iter().map(|r| r["price"].as_f64().unwrap()).sum()
}

#[test]
fn bundle_resolves_refs_against_exact_data() {
    let b = template("bundle");
    let orders = b["parts"]["orders"].clone();
    let expected_sum = records_sum(&orders);
    let artifact = render::render(&b).unwrap();

    let count_line = artifact
        .lines()
        .find(|l| l.starts_with("- Orders analyzed: "))
        .expect("count ref resolved");
    assert_eq!(count_line, "- Orders analyzed: 120.");

    let sum_line = artifact
        .lines()
        .find(|l| l.starts_with("- Total revenue: "))
        .expect("sum ref resolved");
    let sum_str: String = sum_line
        .trim_start_matches("- Total revenue: ")
        .trim_end_matches('.')
        .to_string();
    let rendered_sum: f64 = sum_str.parse().expect("numeric sum in the bundle");
    assert!(
        (rendered_sum - expected_sum).abs() < 1e-6,
        "bundle cites {rendered_sum}, data sums to {expected_sum}"
    );
}

#[test]
fn bundle_is_deterministic() {
    let b = template("bundle");
    assert_eq!(render::render(&b).unwrap(), render::render(&b).unwrap());
}

#[test]
fn bundle_reference_cycle_is_rejected() {
    let spec = json!({
        "kind": "bundle",
        "parts": {
            "a": {"kind": "table", "title": "A", "columns": ["x"], "rows": [["{ref:part=b,stat=count}"]]},
            "b": {"kind": "table", "title": "B", "columns": ["x"], "rows": [["{ref:part=a,stat=count}"]]}
        }
    });
    let errs = spec::validate(&spec);
    assert!(
        errs.iter().any(|e| e.contains("reference cycle")),
        "{errs:?}"
    );
    assert!(render::render(&spec)
        .unwrap_err()
        .contains("reference cycle"));
}

#[test]
fn bundle_unknown_part_and_bad_stats_are_rejected() {
    let unknown = json!({
        "kind": "bundle",
        "parts": {
            "a": {"kind": "report", "title": "A",
                  "sections": [{"heading": "h", "bullets": ["{ref:part=nope,stat=count}"]}]}
        }
    });
    assert!(spec::validate(&unknown)
        .iter()
        .any(|e| e.contains("unknown part")));

    let bad_stat = json!({
        "kind": "bundle",
        "parts": {
            "a": {"kind": "report", "title": "A",
                  "sections": [{"heading": "h", "bullets": ["{ref:part=b,stat=median,field=x}"]}]},
            "b": {"kind": "records", "n": 2, "seed": 1}
        }
    });
    assert!(spec::validate(&bad_stat).iter().any(|e| e.contains("stat")));

    let self_ref = json!({
        "kind": "bundle",
        "parts": {
            "a": {"kind": "report", "title": "A",
                  "sections": [{"heading": "h", "bullets": ["{ref:part=a,stat=count}"]}]}
        }
    });
    assert!(spec::validate(&self_ref)
        .iter()
        .any(|e| e.contains("self-reference")));
}

#[test]
fn bundle_part_without_seed_is_seeded_from_bundle() {
    // two bundles differing only in the bundle seed must render different data
    let mut b1 = template("bundle");
    b1["seed"] = json!(1);
    b1["parts"]["orders"]
        .as_object_mut()
        .unwrap()
        .remove("seed");
    let mut b2 = template("bundle");
    b2["seed"] = json!(2);
    b2["parts"]["orders"]
        .as_object_mut()
        .unwrap()
        .remove("seed");
    assert_ne!(render::render(&b1).unwrap(), render::render(&b2).unwrap());
}

#[test]
fn bundle_plain_format_and_data_stats() {
    let spec = json!({
        "kind": "bundle", "format": "plain",
        "parts": {
            "data": {"kind": "dataset", "n": 40, "seed": 5, "alphabet": ["a", "b"],
                     "length": 6, "rule": {"type": "mod", "k": 3}},
            "note": {"kind": "list", "title": "checks",
                     "items": ["count is {ref:part=data,stat=count}",
                               "top target is {ref:part=data,stat=top,field=target}",
                               "distinct targets {ref:part=data,stat=unique,field=target}"]}
        }
    });
    let out = render::render(&spec).unwrap();
    assert!(out.contains("count is 40"));
    assert!(out.contains("distinct targets 3"));
    assert!(out.contains("---"), "plain format separates parts");
}

// ------------------------------------------------- records: new distributions --

#[test]
fn records_sum_to_is_exact() {
    let mut s = template("records");
    s["n"] = json!(10);
    s["sum_to"] = json!(1000);
    s["price_dist"] = json!("normal");
    s["price_mean"] = json!(100);
    s["price_sd"] = json!(30);
    s["format"] = json!("jsonl");
    let sum: f64 = records_sum(&s);
    assert!(
        (sum - 1000.0).abs() < 1e-9,
        "sum is exactly sum_to, got {sum}"
    );
}

#[test]
fn records_normal_stays_plausible() {
    let mut s = template("records");
    s["n"] = json!(500);
    s["price_dist"] = json!("normal");
    s["price_mean"] = json!(100);
    s["price_sd"] = json!(5);
    s["format"] = json!("jsonl");
    let rows: Vec<Value> = render::render(&s)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .collect();
    let vals: Vec<f64> = rows.iter().map(|r| r["price"].as_f64().unwrap()).collect();
    assert!(vals.iter().all(|v| (0.0..=200.0).contains(v)));
    let mean: f64 = vals.iter().sum::<f64>() / vals.len() as f64;
    assert!((mean - 100.0).abs() < 10.0, "mean near 100, got {mean}");
}

#[test]
fn records_lognormal_honors_the_mean() {
    let mut s = template("records");
    s["n"] = json!(800);
    s["price_dist"] = json!("lognormal");
    s["price_mean"] = json!(42);
    s["price_sd"] = json!(0.9);
    s["format"] = json!("jsonl");
    let rows: Vec<Value> = render::render(&s)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .collect();
    let vals: Vec<f64> = rows.iter().map(|r| r["price"].as_f64().unwrap()).collect();
    assert!(vals.iter().all(|v| *v > 0.0));
    let mean: f64 = vals.iter().sum::<f64>() / vals.len() as f64;
    assert!(
        (mean - 42.0).abs() < 6.0,
        "lognormal mean near 42, got {mean}"
    );
}

#[test]
fn records_weights_shift_the_mix() {
    let mut s = template("records");
    s["n"] = json!(400);
    s["categories"] = json!(["tools", "toys"]);
    s["pools"] = json!({"tools": ["Hammer"], "toys": ["Kite"]});
    s["category_weights"] = json!({"tools": 9, "toys": 1});
    s["format"] = json!("jsonl");
    let rows: Vec<Value> = render::render(&s)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .collect();
    let tools = rows.iter().filter(|r| r["category"] == "tools").count();
    assert!(tools > 300, "9:1 weights must dominate, got {tools}/400");
}

#[test]
fn records_new_options_are_validated() {
    assert!(spec::validate(&json!({
        "kind": "records", "n": 5, "price_dist": "normal"
    }))
    .iter()
    .any(|e| e.contains("price_mean")));
    assert!(!spec::validate(&json!({
        "kind": "records", "n": 5, "price_dist": "weibull"
    }))
    .is_empty());
    assert!(!spec::validate(&json!({
        "kind": "records", "n": 5, "sum_to": 0
    }))
    .is_empty());
    assert!(spec::validate(&json!({
        "kind": "records", "n": 5, "sum_to": 100, "price_dist": "normal",
        "price_mean": 10, "price_sd": 3
    }))
    .is_empty());
}
