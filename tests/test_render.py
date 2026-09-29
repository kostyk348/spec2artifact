"""Tests for spec2artifact: variant specs validate, renderers are exact and deterministic."""
import json

from spec2artifact import render, spec as S


def test_kinds_and_templates():
    for k in S.kinds():
        t = S.spec_template(k)
        assert t["kind"] == k
        assert S.validate(t) == [], (k, S.validate(t))


def test_records_deterministic_and_schema():
    s = S.spec_template("records"); s["n"] = 50
    a, b = render.render(s), render.render(s)
    assert a == b                                  # deterministic (seeded)
    rows = [json.loads(l) for l in a.splitlines()]
    assert len(rows) == 50
    assert all(set(r) == {"id", "name", "category", "price"} for r in rows)
    assert all(r["category"] in s["categories"] for r in rows)
    assert all(isinstance(r["price"], float) for r in rows)


def test_records_format_variants():
    s = S.spec_template("records"); s["n"] = 5
    for fmt, chk in (("jsonl", lambda t: t.count("\n") == 4),
                     ("csv", lambda t: t.startswith("id,name")),
                     ("json", lambda t: t.lstrip().startswith("["))):
        s["format"] = fmt
        assert chk(render.render(s)), fmt


def test_report_and_list():
    r = S.spec_template("report")
    r["title"] = "T"; r["sections"] = [{"heading": "H", "bullets": ["a", "b"]}]
    out = render.render(r)
    assert "# T" in out and "## H" in out and "- a" in out
    lst = S.spec_template("list"); lst["title"] = "L"; lst["items"] = ["x", "y"]; lst["checked"] = ["x"]
    assert "- [x] x" in render.render(lst) and "- [ ] y" in render.render(lst)


def test_table_and_code():
    t = S.spec_template("table")
    t["columns"] = ["id", "name"]; t["rows"] = [["1", "a"]]
    assert "| id | name |" in render.render(t)
    c = S.spec_template("code")
    c["module"] = "m"; c["functions"] = [{"name": "f", "args": ["a"], "doc": "d"}]
    out = render.render(c)
    assert "def f(a):" in out and '"""d"""' in out


def test_dataset_and_prose():
    d = S.spec_template("dataset")
    d["n"] = 40; d["rule"] = {"type": "mod", "k": 3}; d["alphabet"] = ["a", "b"]
    rows = [json.loads(l) for l in render.render(d).splitlines()]
    al = d["alphabet"]
    assert len(rows) == 40
    assert all(sum(al.index(c) for c in r["sequence"]) % 3 == r["target"] for r in rows)
    assert render.render(d) == render.render(d)                 # deterministic
    p = S.spec_template("prose")
    p["title"] = "T"; p["tone"] = "formal"
    p["paragraphs"] = [{"topic": "memory", "points": ["it is a hash chain", "it is append-only"]}]
    out = render.render(p)
    assert "# T" in out and "memory" in out and "hash chain" in out
    assert S.validate(p) == [] and S.validate({"kind": "prose"}) != []


def test_validation_rejects_bad():
    assert S.validate({"kind": "nope"})
    assert S.validate({"kind": "records", "n": -1})
    assert S.validate({"kind": "records", "n": 1, "categories": ["x"], "pools": {}})
    assert S.validate({"kind": "report"})


if __name__ == "__main__":
    for name, fn in sorted(globals().items()):
        if name.startswith("test_") and callable(fn):
            fn(); print("ok", name)
    print("ALL PASS")
