"""spec2artifact CLI.

  spec2artifact kinds
  spec2artifact template <kind>          # the VARIANT spec skeleton for the LLM to fill
  spec2artifact validate <spec.json>
  spec2artifact render <spec.json> [-o OUT]

The token game: the LLM writes only the small spec; `render` produces the whole artifact locally.
"""
from __future__ import annotations

import argparse
import json
import sys

from . import render, spec as spec_mod


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(prog="spec2artifact")
    sub = ap.add_subparsers(dest="cmd", required=True)
    sub.add_parser("kinds")
    p_t = sub.add_parser("template"); p_t.add_argument("kind")
    p_v = sub.add_parser("validate"); p_v.add_argument("spec")
    p_r = sub.add_parser("render"); p_r.add_argument("spec"); p_r.add_argument("-o", "--out")
    a = ap.parse_args(argv)

    if a.cmd == "kinds":
        for k in spec_mod.kinds():
            print(f"{k:9s} {spec_mod.KINDS[k]['desc']}")
        return 0
    if a.cmd == "template":
        print(json.dumps(spec_mod.spec_template(a.kind), ensure_ascii=False, indent=1))
        return 0
    if a.cmd in ("validate", "render"):
        s = json.load(open(a.spec))
        errs = spec_mod.validate(s)
        if errs:
            print("INVALID SPEC:", *errs, sep="\n  ", file=sys.stderr)
            return 2
        if a.cmd == "validate":
            print(f"ok: kind={s['kind']}")
            return 0
        text = render.render(s)
        if a.out:
            open(a.out, "w").write(text)
            print(f"wrote {a.out} ({len(text)} chars, ~{len(text)//4} tokens)")
        else:
            sys.stdout.write(text)
        return 0
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
