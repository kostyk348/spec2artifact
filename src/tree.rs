//! `tree` — deterministic Unicode trees (├── └──) for project layouts,
//! org charts, tech trees, category taxonomies.
//!
//! Input is a nested structure; rendering is exact, so the same tree spec
//! always draws the same box-drawing lines. Accepts plain-string leaves.

use serde_json::Value;

/// Render a tree. Public entry from the render dispatcher.
pub fn render_tree(spec: &Value) -> Result<String, String> {
    let mut out: Vec<String> = Vec::new();
    if let Some(t) = crate::render::truthy_title(spec) {
        out.push(format!("# {t}"));
        out.push(String::new());
    }
    match spec.get("root") {
        Some(Value::String(r)) if !r.is_empty() => out.push(r.clone()),
        None | Some(Value::Null) => {}
        Some(other) => return Err(format!("tree.root must be a string, got {other}")),
    }
    let nodes = spec
        .get("nodes")
        .ok_or_else(|| "tree needs \"nodes\": a list of labels or nested nodes".to_string())?;
    draw(nodes, "", &mut out)?;
    out.push(String::new());
    Ok(out.join("\n"))
}

fn draw(nodes: &Value, prefix: &str, out: &mut Vec<String>) -> Result<(), String> {
    let list = nodes
        .as_array()
        .ok_or_else(|| "tree.nodes must be a list (labels or nested nodes)".to_string())?;
    let n = list.len();
    for (i, node) in list.iter().enumerate() {
        let last = i == n - 1;
        let (label, children) = match node {
            Value::String(s) => (s.clone(), None),
            Value::Object(o) => {
                let label = o
                    .get("label")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "every tree node needs a string \"label\"".to_string())?;
                (label.to_string(), o.get("nodes"))
            }
            other => {
                return Err(format!(
                    "tree nodes must be strings or objects with a label, got {other}"
                ))
            }
        };
        out.push(format!(
            "{prefix}{} {label}",
            if last { "└──" } else { "├──" }
        ));
        if let Some(children) = children {
            let child_prefix = format!("{prefix}{}", if last { "    " } else { "│   " });
            draw(children, &child_prefix, out)?;
        }
    }
    Ok(())
}
