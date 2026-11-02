//! spec2artifact CLI.
//!
//! ```text
//! spec2artifact kinds
//! spec2artifact template <kind>          # the VARIANT spec skeleton for the LLM to fill
//! spec2artifact validate <spec.json>
//! spec2artifact render <spec.json> [-o OUT]
//! spec2artifact mcp                      # stdio JSON-RPC MCP server
//! spec2artifact fingerprint              # stable hash of the MCP tools block
//! ```
//!
//! The token game: the LLM writes only the small spec; `render` produces the whole
//! artifact locally, for 0 LLM tokens.

use clap::{Parser, Subcommand};
use spec2artifact::{mcp, render, spec};
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "spec2artifact",
    version,
    about = "LLM writes a small variant spec; the algebra renders the artifact"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List task kinds and their variant spec descriptions
    Kinds,
    /// Print the variant spec skeleton for a kind
    Template { kind: String },
    /// Structurally validate a spec file
    Validate { spec: PathBuf },
    /// Render the artifact from a spec file
    Render {
        spec: PathBuf,
        /// Write the artifact to a file instead of stdout
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
    /// Serve the MCP stdio JSON-RPC server (tools: spec_kinds, spec_template, spec_validate, spec_render)
    Mcp,
    /// Print the stable fingerprint of the MCP tools block (cache-safety guard)
    Fingerprint,
}

/// Binary entry point.
pub fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    let cli = Cli::parse();
    match cli.cmd {
        Command::Kinds => {
            for k in spec::kinds() {
                let desc = spec::kind_description(k).unwrap_or("");
                println!("{k:9} {desc}");
            }
            0
        }
        Command::Template { kind } => match spec::spec_template(&kind) {
            Ok(t) => {
                println!("{t:#}");
                0
            }
            Err(e) => {
                eprintln!("error: {e}");
                2
            }
        },
        Command::Validate { spec: path } => match load_spec(&path) {
            Ok(s) => {
                let errs = spec::validate(&s);
                if errs.is_empty() {
                    let kind = s.get("kind").and_then(|v| v.as_str()).unwrap_or("?");
                    println!("ok: kind={kind}");
                    0
                } else {
                    eprintln!("INVALID SPEC:\n  {}", errs.join("\n  "));
                    2
                }
            }
            Err(e) => {
                eprintln!("error: {e}");
                2
            }
        },
        Command::Render { spec: path, out } => match load_spec(&path) {
            Ok(s) => {
                let errs = spec::validate(&s);
                if !errs.is_empty() {
                    eprintln!("INVALID SPEC:\n  {}", errs.join("\n  "));
                    return 2;
                }
                match render::render(&s) {
                    Ok(text) => match out {
                        Some(path) => match std::fs::write(&path, &text) {
                            Ok(()) => {
                                println!(
                                    "wrote {} ({} chars, ~{} tokens)",
                                    path.display(),
                                    text.len(),
                                    text.len() / 4
                                );
                                0
                            }
                            Err(e) => {
                                eprintln!("error: cannot write {}: {e}", path.display());
                                1
                            }
                        },
                        None => {
                            print!("{text}");
                            let _ = std::io::stdout().flush();
                            0
                        }
                    },
                    Err(e) => {
                        eprintln!("error: {e}");
                        1
                    }
                }
            }
            Err(e) => {
                eprintln!("error: {e}");
                2
            }
        },
        Command::Mcp => {
            mcp::serve();
            0
        }
        Command::Fingerprint => {
            println!("{}", mcp::tools_fingerprint());
            0
        }
    }
}

fn load_spec(path: &std::path::Path) -> Result<serde_json::Value, String> {
    let raw = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("{} is not valid JSON: {e}", path.display()))
}
