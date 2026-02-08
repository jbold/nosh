use clap::{Parser, Subcommand};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

mod content_types;
mod output;
mod schema;

#[derive(Parser)]
#[command(name = "nosh", version, about = "Nosh specification validator CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate .nosh files against the Nosh JSON Schema
    Validate {
        /// File or directory to validate (.nosh files)
        path: PathBuf,

        /// Enable strict mode: warn on missing content-type-specific fields
        #[arg(long)]
        strict: bool,

        /// Output results as JSON
        #[arg(long)]
        json: bool,

        /// Suppress output for valid files
        #[arg(long)]
        quiet: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate {
            path,
            strict,
            json,
            quiet,
        } => {
            let exit_code = run_validate(&path, strict, json, quiet);
            process::exit(exit_code);
        }
    }
}

fn run_validate(path: &Path, strict: bool, json_output: bool, quiet: bool) -> i32 {
    let validator = schema::compile_schema();

    let files = if path.is_dir() {
        collect_nosh_files(path)
    } else if path.is_file() {
        vec![path.to_path_buf()]
    } else {
        eprintln!("Error: {} does not exist", path.display());
        return 2;
    };

    if files.is_empty() {
        eprintln!("No .nosh files found in {}", path.display());
        return 2;
    }

    let mut results = Vec::new();
    let mut has_invalid = false;

    for file in &files {
        let result = validate_file(file, &validator, strict);
        if !result.valid {
            has_invalid = true;
        }
        results.push(result);
    }

    if json_output {
        println!("{}", output::format_json(&results));
    } else {
        let text = output::format_human(&results, quiet);
        if !text.is_empty() {
            print!("{text}");
        }
    }

    if has_invalid {
        1
    } else {
        0
    }
}

fn validate_file(
    path: &Path,
    validator: &jsonschema::Validator,
    strict: bool,
) -> output::FileResult {
    let path_str = path.display().to_string();

    // Read file
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            return output::build_result(
                path_str,
                vec![schema::ValidationError {
                    path: String::new(),
                    message: format!("Cannot read file: {e}"),
                }],
                vec![],
                vec![],
            );
        }
    };

    // Check file size
    let mut warnings = Vec::new();
    let metadata = fs::metadata(path).ok();
    if let Some(ref meta) = metadata {
        if let Some(warning) = schema::check_file_size(meta.len()) {
            warnings.push(warning);
        }
    }

    // Parse JSON
    let doc: Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            return output::build_result(
                path_str,
                vec![schema::ValidationError {
                    path: String::new(),
                    message: format!("Invalid JSON: {e}"),
                }],
                warnings,
                vec![],
            );
        }
    };

    // Check version warning
    if let Some(warning) = schema::check_version(&doc) {
        warnings.push(warning);
    }

    // Validate against schema
    let errors = schema::validate(validator, &doc);

    // Strict mode: content-type-specific checks
    let strict_warnings = if strict {
        content_types::strict_validate(&doc)
    } else {
        vec![]
    };

    output::build_result(path_str, errors, warnings, strict_warnings)
}

fn collect_nosh_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_nosh_files_recursive(dir, &mut files);
    files.sort();
    files
}

fn collect_nosh_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_nosh_files_recursive(&path, files);
        } else if let Some(ext) = path.extension() {
            if ext == "nosh" {
                files.push(path);
            }
        }
    }
}
