#!/usr/bin/env python3
"""Launcher so the MCP server runs regardless of cwd / PYTHONPATH."""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from spec2artifact.mcp_server import main  # noqa: E402

if __name__ == "__main__":
    main()
