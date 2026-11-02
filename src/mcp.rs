//! spec2artifact MCP server (stdio JSON-RPC 2.0) — so an agent can render specs as a tool call.
//!
//! Tools: `spec_kinds`, `spec_template(kind)`, `spec_validate(spec)`, `spec_render(spec)`.
//! Runs forever on stdin/stdout; one bad message never kills the server; a JSON-RPC
//! NOTIFICATION (no `id`) is never answered.

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::io::{BufRead, Write};

/// The tool manifest served over `tools/list` (also the fingerprint input).
pub fn tools() -> Value {
    json!([
        {
            "name": "spec_kinds",
            "description": "list task kinds and their variant spec descriptions",
            "inputSchema": {"type": "object", "properties": {}}
        },
        {
            "name": "spec_template",
            "description": "the variant spec skeleton the LLM must fill for a kind",
            "inputSchema": {"type": "object", "properties": {"kind": {"type": "string"}}, "required": ["kind"]}
        },
        {
            "name": "spec_validate",
            "description": "validate a spec (structure only)",
            "inputSchema": {"type": "object", "properties": {"spec": {"type": "object"}}, "required": ["spec"]}
        },
        {
            "name": "spec_render",
            "description": "render the artifact from a spec (0 LLM tokens)",
            "inputSchema": {"type": "object", "properties": {"spec": {"type": "object"}}, "required": ["spec"]}
        }
    ])
}

/// Stable fingerprint of the tools block (cache-safety guard: any change to this
/// hash is one prompt-cache break; batch such changes, never change tools per turn).
pub fn tools_fingerprint() -> String {
    let block = json!({ "tools": tools() }).to_string();
    let digest = Sha256::digest(block.as_bytes());
    let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    format!(
        "tools={} sha256={}",
        tools().as_array().map_or(0, |a| a.len()),
        &hex[..32]
    )
}

/// Dispatch one MCP tool call.
///
/// # Errors
/// Unknown tool name, or a malformed/unknown spec.
pub fn handle(name: &str, args: &Value) -> Result<Value, String> {
    match name {
        "spec_kinds" => {
            let mut map = serde_json::Map::new();
            for k in crate::spec::kinds() {
                let desc = crate::spec::kind_description(k).unwrap_or_default();
                map.insert(k.to_string(), json!(desc));
            }
            Ok(Value::Object(map))
        }
        "spec_template" => {
            let kind = args
                .get("kind")
                .and_then(|v| v.as_str())
                .ok_or("missing required argument 'kind'")?;
            crate::spec::spec_template(kind)
        }
        "spec_validate" => {
            let spec = args.get("spec").ok_or("missing required argument 'spec'")?;
            Ok(json!({ "errors": crate::spec::validate(spec) }))
        }
        "spec_render" => {
            let spec = args.get("spec").ok_or("missing required argument 'spec'")?;
            let artifact = crate::render::render(spec)?;
            Ok(json!({ "artifact": artifact }))
        }
        other => Err(format!("unknown tool: {other}")),
    }
}

fn write_line(out: &mut impl Write, value: &Value) {
    let _ = writeln!(out, "{value}");
    let _ = out.flush();
}

/// Serve MCP over stdio until EOF.
pub fn serve() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().lock();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Ok(req) = serde_json::from_str::<Value>(line) else {
            continue; // malformed JSON: ignore, never die
        };
        let result = (|| -> Option<Value> {
            let rid = req.get("id")?;
            // JSON-RPC 2.0: a NOTIFICATION (no id) must NEVER be answered.
            let method = req.get("method").and_then(|m| m.as_str())?;
            Some(match method {
                "initialize" => {
                    let pv = req
                        .get("params")
                        .and_then(|p| p.get("protocolVersion"))
                        .cloned()
                        .unwrap_or_else(|| json!("2024-11-05"));
                    json!({
                        "protocolVersion": pv,
                        "capabilities": {"tools": {"listChanged": false}},
                        "serverInfo": {"name": "spec2artifact", "version": env!("CARGO_PKG_VERSION")}
                    })
                }
                "tools/list" => json!({ "tools": tools() }),
                "tools/call" => {
                    let p = req.get("params").cloned().unwrap_or_else(|| json!({}));
                    let name = p.get("name").and_then(|n| n.as_str()).unwrap_or("");
                    let args = p.get("arguments").cloned().unwrap_or_else(|| json!({}));
                    match handle(name, &args) {
                        Ok(result) => json!({
                            "content": [{"type": "text", "text": result.to_string()}]
                        }),
                        Err(e) => json!({
                            "content": [{"type": "text", "text": json!({"error": e}).to_string()}],
                            "isError": true
                        }),
                    }
                }
                "ping" => json!({}),
                "resources/list" => json!({ "resources": [] }),
                "prompts/list" => json!({ "prompts": [] }),
                "resources/templates/list" => json!({ "resources": [] }),
                other => {
                    write_line(
                        &mut stdout,
                        &json!({
                            "jsonrpc": "2.0", "id": rid,
                            "error": {"code": -32601, "message": format!("method not found: {other}")}
                        }),
                    );
                    return None;
                }
            })
        })();
        if let Some(res) = result {
            write_line(
                &mut stdout,
                &json!({ "jsonrpc": "2.0", "id": req.get("id"), "result": res }),
            );
        }
    }
}
