use std::env;
use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> io::Result<()> {
    let dir = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| env::current_dir().unwrap());
    let cargo_toml_paths = find_cargo_toml(&dir)?;
    let mut total_space_cleaned = 0;

    for path in cargo_toml_paths {
        let dir = path.parent().unwrap();

        let output = Command::new("cargo")
            .current_dir(dir)
            .arg("clean")
            .output()?;

        if output.status.success() {
            let cleaned_space = parse_cargo_clean_output(&output.stderr)?;
            total_space_cleaned += cleaned_space;
        } else {
            eprintln!("Failed to run `cargo clean` in {:?}: {}", dir, String::from_utf8_lossy(&output.stderr));
        }
    }

    println!("Total disk space cleaned: {:.2} GB", total_space_cleaned as f64 / 1_000_000_000.0);

    Ok(())
}

fn find_cargo_toml(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut cargo_toml_paths = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            cargo_toml_paths.extend(find_cargo_toml(&path)?);
        } else if path.file_name() == Some("Cargo.toml".as_ref()) {
            println!("Cargo.toml found: {}", path.display());
            cargo_toml_paths.push(path);
        }
    }
    Ok(cargo_toml_paths)
}

fn parse_cargo_clean_output(output: &[u8]) -> io::Result<u64> {
    let output_str = String::from_utf8_lossy(output);

    for line in output_str.lines() {
        if line.contains("Removed") {
            if let Some(size_part) = line.split(',').nth(1) {
                let size_str = size_part.trim().split_whitespace().next().unwrap_or("0");
                return Ok(parse_size_to_bytes(size_str));
            }
        }
    }

    Ok(0)
}

fn parse_size_to_bytes(size_str: &str) -> u64 {
    let size_str = size_str.to_lowercase();
    if size_str.ends_with("kib") {
        let num = size_str.trim_end_matches("kib").parse::<f64>().unwrap_or(0.0);
        (num * 1024.0) as u64
    } else if size_str.ends_with("mib") {
        let num = size_str.trim_end_matches("mib").parse::<f64>().unwrap_or(0.0);
        (num * 1024.0 * 1024.0) as u64
    } else if size_str.ends_with("gib") {
        let num = size_str.trim_end_matches("gib").parse::<f64>().unwrap_or(0.0);
        (num * 1024.0 * 1024.0 * 1024.0) as u64
    } else if size_str.ends_with("b") {
        size_str.trim_end_matches("b").parse::<u64>().unwrap_or(0)
    } else {
        0
    }
}