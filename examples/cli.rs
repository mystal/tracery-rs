use std::fs;

use tracery::{self, Result};

const USAGE: &str = r#"
Usage: cli <JSON_FILE> [N]

Arguments:
  <JSON_FILE> Path to Tracery JSON file.
  [N]         Number of lines to generate (default: 1).
"#;

fn main() -> Result<()> {
    let mut args = std::env::args_os().skip(1);

    // Parse args.
    let Some(json_path) = args.next() else {
        println!("{}", USAGE.trim());
        return Ok(());
    };
    let n: u32 = args.next()
        .and_then(|s| s.into_string().ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    let json_str = fs::read_to_string(json_path).unwrap();
    for _ in 0..n {
        println!("{}", tracery::flatten_json(&json_str)?);
    }

    Ok(())
}
