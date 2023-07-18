use std::fs;

use tracery::{self, Result};

fn main() -> Result<()> {
    if let Some(json_path) = std::env::args_os().nth(1) {
        let json_str = fs::read_to_string(json_path).unwrap();
        eprintln!("{}", json_str);
        println!("{}", tracery::flatten_json(json_str)?);
    }

    Ok(())
}
