"""spec2artifact MCP server (stdio JSON-RPC) -- so an agent can render specs as a tool call.

Tools: spec_kinds, spec_template(kind), spec_validate(spec), spec_render(spec).
"""
from __future__ import annotations

import json
import sys

from . import render, spec as spec_mod

TOOLS = [
    {"name": "spec_kinds", "description": "list task kinds and their variant spec descriptions",
     "inputSchema": {"type": "object", "properties": {}}},
    {"name": "spec_template", "description": "the variant spec skeleton the LLM must fill for a kind",
     "inputSchema": {"type": "object", "properties": {"kind": {"type": "string"}}, "required": ["kind"]}},
    {"name": "spec_validate", "description": "validate a spec (structure only)",
     "inputSchema": {"type": "object", "properties": {"spec": {"type": "object"}}, "required": ["spec"]}},
    {"name": "spec_render", "description": "render the artifact from a spec (0 LLM tokens)",
     "inputSchema": {"type": "object", "properties": {"spec": {"type": "object"}}, "required": ["spec"]}},
]


def handle(name, args):
    if name == "spec_kinds":
        return {k: spec_mod.KINDS[k]["desc"] for k in spec_mod.kinds()}
    if name == "spec_template":
        return spec_mod.spec_template(args["kind"])
    if name == "spec_validate":
        return {"errors": spec_mod.validate(args["spec"])}
    if name == "spec_render":
        return {"artifact": render.render(args["spec"])}
    raise KeyError(name)


def main() -> None:
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            req = json.loads(line)
        except json.JSONDecodeError:
            continue
        try:
            rid = req.get("id")
            method = req.get("method")

            # JSON-RPC 2.0: a NOTIFICATION (no id) must NEVER be answered.
            if rid is None:
                continue

            if method == "initialize":
                pv = (req.get("params") or {}).get("protocolVersion", "2024-11-05")
                res = {"protocolVersion": pv,
                       "capabilities": {"tools": {"listChanged": False}},
                       "serverInfo": {"name": "spec2artifact", "version": "0.1.0"}}
            elif method == "tools/list":
                res = {"tools": TOOLS}
            elif method == "tools/call":
                p = req.get("params", {})
                try:
                    res = {"content": [{"type": "text",
                                        "text": json.dumps(handle(p.get("name"), p.get("arguments", {})))}]}
                except Exception as e:  # noqa: BLE001
                    res = {"content": [{"type": "text", "text": json.dumps({"error": str(e)})}], "isError": True}
            elif method == "ping":
                res = {}
            elif method in ("resources/list", "prompts/list", "resources/templates/list"):
                key = ("resources" if "resources" in method else "prompts")
                res = {key: []}
            else:
                out = {"jsonrpc": "2.0", "id": rid,
                       "error": {"code": -32601, "message": f"method not found: {method}"}}
                sys.stdout.write(json.dumps(out) + "\n"); sys.stdout.flush()
                continue

            sys.stdout.write(json.dumps({"jsonrpc": "2.0", "id": rid, "result": res}) + "\n")
            sys.stdout.flush()
        except Exception:  # noqa: BLE001 -- never die on one bad message
            continue


if __name__ == "__main__":
    main()
