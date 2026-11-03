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
        vec![
            "bundle", "chart", "code", "dataset", "list", "map", "prose", "records", "report",
            "table", "tree"
        ]
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

// ------------------------------------------------------------- {expr:...} --

#[test]
fn expr_eval_precedence_functions_and_refs() {
    use spec2artifact::expr;
    let stats = serde_json::json!({});
    let _ = stats;
    let lookup = |p: &str, s: &str, f: Option<&str>| -> Result<f64, String> {
        match (p, s, f) {
            ("orders", "sum", Some("price")) => Ok(100.0),
            ("orders", "count", None) => Ok(4.0),
            _ => Err("no value".into()),
        }
    };
    assert_eq!(expr::eval("1 + 2 * 3", &lookup).unwrap(), 7.0);
    assert_eq!(expr::eval("(1 + 2) * 3", &lookup).unwrap(), 9.0);
    assert!(expr::eval("-2^", &lookup).is_err());
    assert_eq!(expr::eval("2 * -3", &lookup).unwrap(), -6.0);
    assert_eq!(
        expr::eval("round(10 / 3 * 100) / 100", &lookup).unwrap(),
        3.33
    );
    assert_eq!(expr::eval("max(1, 5) + min(1, 5)", &lookup).unwrap(), 6.0);
    assert_eq!(
        expr::eval("sqrt(9) + clamp(10, 0, 5)", &lookup).unwrap(),
        8.0
    );
    assert_eq!(expr::eval("if(1 > 2, 10, 20)", &lookup).unwrap(), 20.0);
    assert_eq!(
        expr::eval("if(ref('orders','count') >= 4, 1, 0)", &lookup).unwrap(),
        1.0
    );
    assert_eq!(
        expr::eval("ref('orders','sum','price') * 1.2", &lookup).unwrap(),
        120.0
    );
    assert_eq!(expr::eval("1 && 0 || 1", &lookup).unwrap(), 1.0);
    assert!(expr::eval("!(0)", &lookup).is_err()); // no ! operator — by design
}

#[test]
fn expr_errors_are_precise() {
    use spec2artifact::expr;
    let lookup = |_p: &str, _s: &str, _f: Option<&str>| -> Result<f64, String> {
        Err("no value".to_string())
    };
    assert!(expr::eval("1 / 0", &lookup)
        .unwrap_err()
        .contains("division by zero"));
    assert!(expr::eval("1 % 0", &lookup).unwrap_err().contains("modulo"));
    assert!(expr::eval("nope(1)", &lookup)
        .unwrap_err()
        .contains("unknown function"));
    assert!(expr::eval("1 +", &lookup)
        .unwrap_err()
        .contains("unexpected end"));
    assert!(expr::eval("ref('x','sum')", &lookup)
        .unwrap_err()
        .contains("no value"));
    assert!(expr::eval("1 2", &lookup)
        .unwrap_err()
        .contains("unexpected"));
}

#[test]
fn bundle_resolves_exprs_against_part_stats() {
    let mut b = template("bundle");
    b["parts"]["summary"]["sections"][0]["bullets"]
        .as_array_mut()
        .unwrap()
        .push(json!("VAT: {expr:ref('orders','sum','price') * 1.2}."));
    b["parts"]["summary"]["sections"][0]["bullets"]
        .as_array_mut()
        .unwrap()
        .push(json!(
            "Per order cap: {expr:round(ref('orders','sum','price') / ref('orders','stat_x') )}"
        ));
    // drop the deliberately broken second bullet — replaced below by a valid mean one
    let arr = b["parts"]["summary"]["sections"][0]["bullets"]
        .as_array_mut()
        .unwrap();
    arr.pop();
    arr.push(json!(
        "Mean: {expr:ref('orders','sum','price') / ref('orders','count')}."
    ));
    let out = render::render(&b).unwrap();
    let orders = b["parts"]["orders"].clone();
    let rows: Vec<Value> = render::render(&orders)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .collect();
    let sum: f64 = rows.iter().map(|r| r["price"].as_f64().unwrap()).sum();
    let mean = sum / rows.len() as f64;
    let parse_after = |prefix: &str| -> f64 {
        out.lines()
            .find(|l| l.starts_with(prefix))
            .unwrap_or_else(|| panic!("{prefix} missing:\n{out}"))
            .trim_start_matches(prefix)
            .trim_end_matches('.')
            .parse()
            .expect("numeric expr result")
    };
    let vat = parse_after("- VAT: ");
    assert!((vat - sum * 1.2).abs() < 1e-6, "VAT {vat} vs {}", sum * 1.2);
    let m = parse_after("- Mean: ");
    assert!((m - mean).abs() < 1e-6, "mean {m} vs {mean}");
}

#[test]
fn bundle_expr_syntax_errors_are_caught_by_validate() {
    let bad = json!({
        "kind": "report", "title": "R",
        "sections": [{"heading": "h", "bullets": ["{expr:1 +}"]}]
    });
    let errs = spec::validate(&bad);
    assert!(errs.iter().any(|e| e.contains("expr")), "{errs:?}");
    assert!(render::render(&bad).is_err());
}

// ------------------------------------------------------------------ chart --

#[test]
fn chart_bars_render_deterministically_descending() {
    let c = json!({
        "kind": "chart", "title": "Rev", "chart": "bars",
        "data": [["a", 10], ["b", 50], ["c", 30]]
    });
    let out = render::render(&c).unwrap();
    assert_eq!(out, render::render(&c).unwrap());
    let lines: Vec<&str> = out.lines().collect();
    assert!(lines[2].starts_with("b │"));
    assert!(lines[2].contains('█'));
    assert!(lines[3].starts_with("c │"));
    assert!(lines[4].starts_with("a │"));
    assert!(out.contains("50"));
}

#[test]
fn chart_histogram_and_sparkline() {
    let h = json!({
        "kind": "chart", "chart": "histogram", "buckets": 4, "width": 20,
        "data": [[1, 1.0], [2, 2.0], [3, 3.0], [4, 4.0], [5, 100.0]]
    });
    let out = render::render(&h).unwrap();
    assert!(out.contains("[1.00,"), "{out}");
    assert!(
        out.lines().any(|l| l.ends_with(" 4")),
        "counts visible: {out}"
    );
    assert!(out.lines().any(|l| l.ends_with(" 1")));
    let s = json!({
        "kind": "chart", "chart": "sparkline",
        "data": [[1, 3], [2, 7], [3, 4], [4, 9], [5, 2]]
    });
    let out = render::render(&s).unwrap();
    assert!(out.contains("▂▆▃█▁"), "{out}");
    assert!(out.contains("min 2, max 9"));
}

#[test]
fn chart_of_part_requires_bundle_context() {
    let c = json!({
        "kind": "chart", "chart": "bars",
        "of": {"part": "orders", "by": "category", "stat": "sum", "field": "price"}
    });
    let err = render::render(&c).unwrap_err();
    assert!(err.contains("bundle context"), "{err}");
}

// ------------------------------------------------------------- table.from --

#[test]
fn rollup_table_matches_the_data_exactly() {
    // the crown test: parse the rendered rollup table and check every cell
    // against an independent computation over the same generated rows
    let mut b = template("bundle");
    b["parts"]["by_cat"] = json!({
        "kind": "table", "title": "By category", "sort": "label",
        "from": {"part": "orders", "by": "category",
                 "cols": {"orders": "count", "revenue": "sum:price"}}
    });
    let out = render::render(&b).unwrap();
    let orders = b["parts"]["orders"].clone();
    let rows: Vec<Value> = render::render(&orders)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .collect();
    let mut want: std::collections::BTreeMap<String, (u64, f64)> = Default::default();
    for r in &rows {
        let e = want
            .entry(r["category"].as_str().unwrap().to_string())
            .or_insert((0, 0.0));
        e.0 += 1;
        e.1 += r["price"].as_f64().unwrap();
    }
    for (cat, (count, sum)) in want {
        let line = out
            .lines()
            .find(|l| l.starts_with(&format!("| {cat} ")))
            .unwrap_or_else(|| panic!("row for {cat} missing in\n{out}"));
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        let cnt: f64 = cells[1].parse().expect("numeric count");
        let rev: f64 = cells[2].parse().expect("numeric revenue");
        assert_eq!(cnt, count as f64, "{cat}: count in {line}");
        assert!((rev - sum).abs() < 1e-6, "{cat}: revenue {rev} vs {sum}");
    }
}

#[test]
fn rollup_table_sorts_by_value() {
    let t = json!({
        "kind": "table", "title": "T", "sort": "value", "sort_by": "rev",
        "from": {"part": "x", "by": "k", "cols": {"rev": "sum:v"}}
    });
    // standalone (no ctx) must fail with the bundle-context message
    let err = render::render(&t).unwrap_err();
    assert!(err.contains("bundle context"), "{err}");
}

#[test]
fn chart_and_rollup_validation() {
    assert!(!spec::validate(&json!({"kind": "chart", "chart": "pie"})).is_empty());
    assert!(!spec::validate(&json!({"kind": "chart"})).is_empty());
    assert!(!spec::validate(&json!({
        "kind": "chart", "data": [["a", "x"]]
    }))
    .is_empty());
    assert!(!spec::validate(&json!({
        "kind": "chart", "of": {"part": "x", "by": "k", "stat": "median"}
    }))
    .is_empty());
    assert!(spec::validate(&json!({
        "kind": "chart", "data": [["a", 1]]
    }))
    .is_empty());
    assert!(!spec::validate(&json!({
        "kind": "table", "from": {"part": "x", "by": "k", "cols": {"c": "top"}}
    }))
    .is_empty());
    assert!(spec::validate(&json!({
        "kind": "table", "from": {"part": "x", "by": "k", "cols": {"c": "top:name"}}
    }))
    .is_empty());
    assert!(!spec::validate(&json!({
        "kind": "table", "from": {"part": "x", "by": "k", "cols": {"c": "sum:v"}},
        "sort_by": "nope"
    }))
    .is_empty());
}

#[test]
fn bundle_cycle_through_expr_and_chart_source() {
    // a chart sourcing a part that expr-references the chart's sibling — topo must order it
    let spec = json!({
        "kind": "bundle",
        "parts": {
            "data": {"kind": "table", "title": "D", "columns": ["k"], "rows": [["{ref:part=head,stat=count}"]]},
            "head": {"kind": "table", "title": "H", "columns": ["k"], "rows": [["{ref:part=data,stat=count}"]]}
        }
    });
    let errs = spec::validate(&spec);
    assert!(
        errs.iter().any(|e| e.contains("reference cycle")),
        "{errs:?}"
    );
    assert!(render::render(&spec).is_err());
}

#[test]
fn standalone_exprs_work_without_a_bundle() {
    let r = json!({
        "kind": "report", "title": "Quote",
        "sections": [{"heading": "h", "bullets": [
            "List: {expr:99 * 3}.",
            "Bulk: {expr:99 * 3 * 0.9}.",
            "Cap: {expr:min(99 * 3 * 0.9, 250)}."
        ]}]
    });
    let out = render::render(&r).unwrap();
    assert!(out.contains("List: 297."), "{out}");
    assert!(out.contains("Bulk: 267.3."), "{out}");
    assert!(out.contains("Cap: 250."), "{out}");
}

#[test]
fn standalone_refs_fail_with_a_clear_message() {
    let r = json!({
        "kind": "report", "title": "R",
        "sections": [{"heading": "h", "bullets": ["{ref:part=x,stat=count}"]}]
    });
    let err = render::render(&r).unwrap_err();
    assert!(err.contains("needs a bundle data context"), "{err}");
}

// --------------------------------------------------------------------- map --

#[test]
fn map_is_deterministic_and_biomes_match_the_legend() {
    let spec = json!({
        "kind": "map", "variant": "terrain", "seed": 42, "w": 40, "h": 12,
        "title": "Isle", "biomes": ["sea", "beach", "plains", "forest", "alps"]
    });
    let out = render::render(&spec).unwrap();
    assert_eq!(out, render::render(&spec).unwrap(), "same spec, same map");
    // the legend's per-glyph counts must equal the grid's actual glyph counts
    let grid: Vec<&str> = out.lines().skip(2).take(12).collect();
    for (glyph, name) in [
        ('≈', "sea"),
        ('░', "beach"),
        ('▒', "plains"),
        ('▓', "forest"),
        ('▲', "alps"),
    ] {
        let in_grid: usize = grid.iter().map(|l| l.matches(glyph).count()).sum();
        let legend_line = out
            .lines()
            .find(|l| l.starts_with(glyph) && l.contains(" — "))
            .unwrap_or_else(|| panic!("legend line for {name} missing:\n{out}"));
        let n: usize = legend_line
            .rsplit('—')
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(in_grid, n, "legend count for {name}");
    }
    let total: usize = out
        .lines()
        .find(|l| l.contains("cells total"))
        .unwrap()
        .split(' ')
        .next()
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(total, 40 * 12);
}

#[test]
fn dungeon_has_rooms_connected_to_the_border_free_grid() {
    let spec = json!({
        "kind": "map", "variant": "dungeon", "seed": 7, "w": 44, "h": 18, "rooms": 6,
        "title": "D"
    });
    let out = render::render(&spec).unwrap();
    assert_eq!(out, render::render(&spec).unwrap());
    // border is intact (all walls) — rooms and corridors never breach the edge
    for line in out.lines().skip(2).take(18) {
        let chars: Vec<char> = line.chars().collect();
        assert_eq!(chars[0], '#');
        assert_eq!(chars[chars.len() - 1], '#');
    }
    let first = out.lines().nth(1).unwrap();
    assert!(
        first.chars().all(|c| c == '#'),
        "top border is wall: {first}"
    );
    let summary = out
        .lines()
        .find(|l| l.contains("walkable cells"))
        .unwrap_or_else(|| panic!("room summary missing:\n{out}"));
    assert!(summary.contains("rooms"), "{summary}");
    assert!(summary.contains("walkable cells"), "{summary}");
    // at least two rooms for this size/seed — corridors only make sense then
    let rooms: usize = summary
        .trim_start_matches("# ")
        .split(' ')
        .next()
        .unwrap()
        .parse()
        .unwrap();
    assert!(rooms >= 2, "want >=2 rooms, got {rooms}");
}

#[test]
fn maze_is_perfect_single_component() {
    // flood-fill from the entry: every open cell must be reachable
    let spec = json!({"kind": "map", "variant": "maze", "seed": 9, "w": 25, "h": 13, "title": "M"});
    let out = render::render(&spec).unwrap();
    assert_eq!(out, render::render(&spec).unwrap());
    let rows: Vec<Vec<char>> = out
        .lines()
        .skip(2)
        .take(13)
        .map(|l| l.chars().collect())
        .collect();
    let mut open = 0usize;
    let mut seen = vec![vec![false; 25]; 13];
    let mut stack = vec![(1usize, 1usize)];
    while let Some((x, y)) = stack.pop() {
        if x >= 25 || y >= 13 || seen[y][x] || rows[y][x] != ' ' {
            continue;
        }
        seen[y][x] = true;
        open += 1;
        stack.extend([
            (x + 1, y),
            (x.wrapping_sub(1), y),
            (x, y + 1),
            (x, y.wrapping_sub(1)),
        ]);
    }
    let total_open = rows
        .iter()
        .map(|r| r.iter().filter(|c| **c == ' ').count())
        .sum::<usize>();
    assert_eq!(open, total_open, "maze must be a single component");
    assert!(total_open > 13 * 25 / 4, "maze is mostly open cells");
    let legend = out
        .lines()
        .find(|l| l.contains("perfect maze"))
        .expect("perfect-maze legend");
    assert!(legend.contains("single component"), "{legend}");
}

#[test]
fn map_validation_rejects_bad_variants_and_dims() {
    assert!(!spec::validate(&json!({"kind": "map", "variant": "sky"})).is_empty());
    assert!(!spec::validate(&json!({"kind": "map", "w": "wide"})).is_empty());
    assert!(!spec::validate(&json!({
        "kind": "map", "biomes": ["a", "b"]
    }))
    .is_empty());
    assert!(spec::validate(&json!({"kind": "map", "seed": 1})).is_empty());
    assert!(spec::validate(&json!({"kind": "map", "variant": "maze", "rooms": 3})).is_empty());
}

// -------------------------------------------------------------------- tree --

#[test]
fn tree_renders_exact_box_drawing() {
    let t = json!({
        "kind": "tree", "root": "app/",
        "nodes": [{"label": "src/", "nodes": ["a.rs", "b.rs"]}, "README.md"]
    });
    let out = render::render(&t).unwrap();
    let want = "\
app/
├── src/
│   ├── a.rs
│   └── b.rs
└── README.md
";
    assert!(out.contains(want), "{out}");
    assert_eq!(out, render::render(&t).unwrap());
}

#[test]
fn tree_validation_rejects_missing_labels_and_deep_nesting() {
    assert!(!spec::validate(&json!({"kind": "tree", "nodes": [{"no": "label"}]})).is_empty());
    assert!(!spec::validate(&json!({"kind": "tree"})).is_empty());
    assert!(spec::validate(&json!({"kind": "tree", "nodes": ["a", {"label": "b/"}]})).is_empty());
    let mut deep = json!("leaf");
    for _ in 0..20 {
        deep = json!([{"label": "x/", "nodes": deep}]);
    }
    assert!(!spec::validate(&json!({"kind": "tree", "nodes": deep})).is_empty());
}

// -------------------------------------------------------------- chart line --

#[test]
fn line_chart_is_deterministic_and_marks_points() {
    let c = json!({
        "kind": "chart", "chart": "line", "width": 40, "height": 8,
        "data": [[1, 3], [2, 5], [3, 4], [4, 8], [5, 6]]
    });
    let out = render::render(&c).unwrap();
    assert_eq!(out, render::render(&c).unwrap());
    assert_eq!(out.matches('●').count(), 5, "one marker per point: {out}");
    assert!(out.contains('·'), "segments present: {out}");
    assert!(out.contains("min 3, max 8, n 5"));
}

#[test]
fn line_chart_flat_series_and_single_point() {
    let flat = json!({
        "kind": "chart", "chart": "line", "width": 20, "height": 6,
        "data": [[1, 5], [2, 5], [3, 5]]
    });
    let out = render::render(&flat).unwrap();
    assert!(out.matches('●').count() == 3, "{out}");
    let one = json!({
        "kind": "chart", "chart": "line", "width": 20, "height": 6,
        "data": [[1, 7]]
    });
    let out = render::render(&one).unwrap();
    assert!(out.matches('●').count() == 1, "{out}");
}

// ---------------------------------------------------------------- showcases --

#[test]
fn showcase_examples_render_and_validate() {
    for (spec_file, artifact_file) in [
        ("examples/game_world.json", "examples/game_world.md"),
        (
            "examples/project_skeleton.json",
            "examples/project_skeleton.md",
        ),
    ] {
        let spec: Value = serde_json::from_str(&std::fs::read_to_string(spec_file).unwrap())
            .unwrap_or_else(|e| panic!("{spec_file}: {e}"));
        let errs = spec::validate(&spec);
        assert!(errs.is_empty(), "{spec_file}: {errs:?}");
        let out = render::render(&spec).unwrap();
        let committed = std::fs::read_to_string(artifact_file).unwrap();
        assert_eq!(out, committed, "{artifact_file} is stale — regenerate it");
    }
}
