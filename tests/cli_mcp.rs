//! End-to-end tests over the real binary: CLI subcommands and the MCP stdio server.

use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::process::{Command, Stdio};

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_spec2artifact"))
}

fn template_file(dir: &std::path::Path, name: &str, v: &Value) -> std::path::PathBuf {
    let p = dir.join(name);
    std::fs::write(&p, v.to_string()).expect("write spec file");
    p
}

#[test]
fn cli_kinds_and_template() {
    let out = bin().args(["kinds"]).output().unwrap();
    assert!(out.status.success());
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(text.contains("records") && text.contains("dataset") && text.contains("prose"));

    let out = bin().args(["template", "records"]).output().unwrap();
    assert!(out.status.success());
    let t: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(t["kind"], json!("records"));

    let out = bin().args(["template", "nope"]).output().unwrap();
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn cli_validate_and_render_roundtrip() {
    let tmp = std::env::temp_dir().join(format!("s2a-test-{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();
    let mut s = serde_json::from_str::<Value>(
        &spec2artifact::spec_template("records").unwrap().to_string(),
    )
    .unwrap();
    s["n"] = json!(12);
    let path = template_file(&tmp, "spec.json", &s);

    let out = bin().args(["validate"]).arg(&path).output().unwrap();
    assert!(out.status.success(), "{out:?}");
    assert!(String::from_utf8(out.stdout)
        .unwrap()
        .starts_with("ok: kind=records"));

    let out = bin().args(["render"]).arg(&path).output().unwrap();
    assert!(out.status.success());
    let text = String::from_utf8(out.stdout).unwrap();
    assert_eq!(text.lines().count(), 12);

    let out_path = tmp.join("out.jsonl");
    let out = bin()
        .args(["render"])
        .arg(&path)
        .args(["-o"])
        .arg(&out_path)
        .output()
        .unwrap();
    assert!(out.status.success());
    let summary = String::from_utf8(out.stdout).unwrap();
    assert!(summary.starts_with("wrote ") && summary.contains("chars"));
    assert_eq!(std::fs::read_to_string(&out_path).unwrap(), text);

    // invalid spec -> exit 2 with INVALID SPEC on stderr
    let bad = template_file(&tmp, "bad.json", &json!({"kind": "records", "n": -1}));
    let out = bin().args(["validate"]).arg(&bad).output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    let err = String::from_utf8(out.stderr).unwrap();
    assert!(err.contains("INVALID SPEC"));
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn cli_render_rejects_broken_json() {
    let tmp = std::env::temp_dir().join(format!("s2a-bad-{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();
    let path = tmp.join("broken.json");
    std::fs::write(&path, "{not json").unwrap();
    let out = bin().args(["render"]).arg(&path).output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    let err = String::from_utf8(out.stderr).unwrap();
    assert!(err.contains("not valid JSON"), "{err}");
    let _ = std::fs::remove_dir_all(&tmp);
}

/// Minimal JSON-RPC client over the spawned MCP server.
struct McpSession {
    child: std::process::Child,
}

impl McpSession {
    fn start() -> Self {
        let child = bin()
            .args(["mcp"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("spawn mcp server");
        McpSession { child }
    }

    fn call(&mut self, req: &Value) -> Value {
        let stdin = self.child.stdin.as_mut().expect("stdin");
        writeln!(stdin, "{req}").expect("write request");
        let mut buf = String::new();
        std::io::BufReader::new(self.child.stdout.as_mut().expect("stdout"))
            .read_line(&mut buf)
            .expect("read response");
        serde_json::from_str(&buf).expect("json-rpc response")
    }
}

impl Drop for McpSession {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn mcp_server_full_flow() {
    let mut mcp = McpSession::start();

    // initialize
    let res = mcp
        .call(&json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2024-11-05"}}))
        ["result"]
        .clone();
    assert_eq!(res["serverInfo"]["name"], json!("spec2artifact"));

    // tools/list
    let res =
        mcp.call(&json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list"}))["result"].clone();
    let tools = res["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 4);

    // tools/call spec_template + spec_render
    let res = mcp.call(&json!({
        "jsonrpc": "2.0", "id": 3, "method": "tools/call",
        "params": {"name": "spec_template", "arguments": {"kind": "records"}}
    }))["result"]
        .clone();
    assert!(!res["isError"].as_bool().unwrap_or(false));
    let inner: Value = serde_json::from_str(res["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(inner["kind"], json!("records"));

    let mut spec = inner.clone();
    spec["n"] = json!(7);
    let res = mcp.call(&json!({
        "jsonrpc": "2.0", "id": 4, "method": "tools/call",
        "params": {"name": "spec_render", "arguments": {"spec": spec}}
    }))["result"]
        .clone();
    let inner: Value = serde_json::from_str(res["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(inner["artifact"].as_str().unwrap().lines().count(), 7);

    // unknown method -> -32601
    let res = mcp.call(&json!({"jsonrpc": "2.0", "id": 5, "method": "no/such"}));
    assert_eq!(res["error"]["code"], json!(-32601));

    // a tool error is an isError result, and the server SURVIVES
    let res = mcp.call(&json!({
        "jsonrpc": "2.0", "id": 6, "method": "tools/call",
        "params": {"name": "spec_template", "arguments": {"kind": "nope"}}
    }));
    assert_eq!(res["result"]["isError"], json!(true));
    let res = mcp.call(&json!({"jsonrpc": "2.0", "id": 7, "method": "ping"}));
    assert_eq!(res["id"], json!(7));
}

#[test]
fn mcp_ignores_garbage_and_notifications() {
    let mut mcp = McpSession::start();
    let stdin = mcp.child.stdin.as_mut().unwrap();
    writeln!(stdin, "this is not json").unwrap();
    writeln!(stdin, "{{}}").unwrap();
    writeln!(
        stdin,
        "{{\"jsonrpc\": \"2.0\", \"method\": \"notify/anything\"}}"
    )
    .unwrap();
    // the server must still answer the next real request
    let res = mcp.call(&json!({"jsonrpc": "2.0", "id": 42, "method": "ping"}));
    assert_eq!(res["id"], json!(42));
}
