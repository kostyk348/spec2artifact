"""spec2artifact: LLM writes a small VARIANT SPEC, the algebra renders the artifact (0 extra tokens)."""
from . import render, spec  # noqa: F401

__version__ = "0.1.0"
__all__ = ["render", "spec"]
