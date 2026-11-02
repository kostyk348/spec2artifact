//! Binary entry point — the CLI lives in the `cli` module.

#[path = "cli.rs"]
mod cli;

fn main() {
    cli::main();
}
