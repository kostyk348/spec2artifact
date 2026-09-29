#!/usr/bin/env python3
"""Cache-safety guard: fingerprint of the MCP tools/list block.

The tool block is part of the prompt PREFIX. If it changes between sessions, the cache breaks
once per change. This prints a stable sha256 of the block so drift is detectable:

    python3 mcp_fingerprint.py        # run in two different sessions -> identical hash expected

Rule: any change to this hash is one cache break. Batch such changes; never change the tool set
per turn.
"""
import hashlib
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from spec2artifact.mcp_server import TOOLS  # noqa: E402

block = json.dumps({"tools": TOOLS}, sort_keys=True, ensure_ascii=False, separators=(",", ":"))
print(f"tools={len(TOOLS)} sha256={hashlib.sha256(block.encode()).hexdigest()[:32]}")
