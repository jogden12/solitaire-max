//! Command-line entry point. It reads the command name and hands off to the library.
//! None of the three commands exist yet; this file only reserves their names.

use std::process::ExitCode;

const USAGE: &str = "\
solitaire-solver: Klondike solver (draw-1)

Commands:
  deal  --seed <hex>                                      print a deal as JSON
  solve --seed <hex> [--timeout <seconds>] [--optimal]    solve one deal
  bench --count <n> [--timeout <seconds>] [--seed-start <hex>]
                                                          solve many deals";

fn main() -> ExitCode {
    let command = std::env::args().nth(1);

    match command.as_deref() {
        None | Some("help" | "--help" | "-h") => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        Some(name @ ("deal" | "solve" | "bench")) => {
            eprintln!("`{name}` is not built yet.");
            ExitCode::from(2)
        }
        Some(other) => {
            eprintln!("Unknown command `{other}`.\n\n{USAGE}");
            ExitCode::from(2)
        }
    }
}
