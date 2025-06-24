use anyhow::{Context, Result};
use clap::{Arg, ArgAction, Command};
use colored::*;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use walkdir::WalkDir;

mod config;
mod formatter;

use config::Config;
use formatter::MustacheJson5Formatter;

fn main() -> Result<()> {
    let matches = Command::new("mustache-json5-fmt")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A formatter for Mustache JSON5 templates")
        .arg(
            Arg::new("files")
                .help("Files to format")
                .num_args(1..)
                .value_name("FILE"),
        )
        .arg(
            Arg::new("check")
                .long("check")
                .help("Check if files are formatted (exit code 1 if not)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("write")
                .long("write")
                .short('w')
                .help("Write formatted output back to files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("stdin")
                .long("stdin")
                .help("Read from stdin")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("indent-size")
                .long("indent-size")
                .help("Number of spaces for indentation")
                .value_name("SIZE")
                .default_value("2"),
        )
        .arg(
            Arg::new("tab-width")
                .long("tab-width")
                .help("Width of tab characters")
                .value_name("WIDTH")
                .default_value("2"),
        )
        .arg(
            Arg::new("use-tabs")
                .long("use-tabs")
                .help("Use tabs for indentation")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("config")
                .long("config")
                .short('c')
                .help("Path to configuration file")
                .value_name("PATH"),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Verbose output")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // Load configuration
    let mut config = Config::default();

    // Override with command line arguments
    if let Some(indent_size) = matches.get_one::<String>("indent-size") {
        config.indent_size = indent_size.parse().context("Invalid indent size")?;
    }

    if let Some(tab_width) = matches.get_one::<String>("tab-width") {
        config.tab_width = tab_width.parse().context("Invalid tab width")?;
    }

    if matches.get_flag("use-tabs") {
        config.use_tabs = true;
    }

    let verbose = matches.get_flag("verbose");
    let check_mode = matches.get_flag("check");
    let write_mode = matches.get_flag("write");

    // Initialize formatter
    let mut formatter = MustacheJson5Formatter::new(config)?;

    // Handle stdin input
    if matches.get_flag("stdin") {
        return handle_stdin(&mut formatter, check_mode);
    }

    // Get files to process
    let files: Vec<String> = if let Some(file_args) = matches.get_many::<String>("files") {
        file_args.cloned().collect()
    } else {
        // If no files specified, look for mustache_json5 files in current directory
        find_mustache_json5_files(".")?
    };

    if files.is_empty() {
        eprintln!("{}", "No mustache_json5 files found".yellow());
        return Ok(());
    }

    let mut needs_formatting = false;
    let mut total_files = 0;
    let mut formatted_files = 0;

    for file_pattern in files {
        let file_paths = expand_file_pattern(&file_pattern)?;

        for file_path in file_paths {
            total_files += 1;

            if verbose {
                println!("Processing: {}", file_path.blue());
            }

            match process_file(&mut formatter, &file_path, check_mode, write_mode) {
                Ok(was_formatted) => {
                    if was_formatted {
                        formatted_files += 1;
                        if check_mode {
                            println!("{} {}", "✗".red(), file_path);
                            needs_formatting = true;
                        } else if write_mode {
                            println!("{} {}", "✓".green(), file_path);
                        }
                    } else if verbose && check_mode {
                        println!("{} {}", "✓".green(), file_path);
                    }
                }
                Err(e) => {
                    eprintln!("{} {}: {}", "Error".red(), file_path, e);
                    return Err(e);
                }
            }
        }
    }

    // Summary
    if verbose || check_mode {
        if check_mode && needs_formatting {
            println!(
                "\n{} {} of {} files need formatting",
                "⚠".yellow(),
                formatted_files,
                total_files
            );
            std::process::exit(1);
        } else if check_mode {
            println!(
                "\n{} All {} files are properly formatted",
                "✓".green(),
                total_files
            );
        } else if write_mode && formatted_files > 0 {
            println!(
                "\n{} Formatted {} of {} files",
                "✓".green(),
                formatted_files,
                total_files
            );
        }
    }

    Ok(())
}

fn handle_stdin(formatter: &mut MustacheJson5Formatter, check_mode: bool) -> Result<()> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .context("Failed to read from stdin")?;

    let formatted = formatter.format(&input).context("Failed to format input")?;

    if check_mode {
        if input != formatted {
            eprintln!("{}", "Input is not formatted".red());
            std::process::exit(1);
        }
    } else {
        print!("{}", formatted);
    }

    Ok(())
}

fn process_file(
    formatter: &mut MustacheJson5Formatter,
    file_path: &str,
    check_mode: bool,
    write_mode: bool,
) -> Result<bool> {
    let original_content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;

    let formatted_content = formatter
        .format(&original_content)
        .with_context(|| format!("Failed to format file: {}", file_path))?;

    let needs_formatting = original_content != formatted_content;

    if needs_formatting {
        if write_mode {
            fs::write(file_path, &formatted_content)
                .with_context(|| format!("Failed to write file: {}", file_path))?;
        } else if !check_mode {
            // Print to stdout if not in check mode and not writing
            print!("{}", formatted_content);
        }
    }

    Ok(needs_formatting)
}

fn find_mustache_json5_files(dir: &str) -> Result<Vec<String>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == "mustache_json5" || extension == "mjson5" {
                if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            }
        }
    }

    Ok(files)
}

fn expand_file_pattern(pattern: &str) -> Result<Vec<String>> {
    // Simple glob expansion - for now just handle single files and basic wildcards
    if pattern.contains('*') {
        // Use walkdir to find matching files
        let mut files = Vec::new();

        // Simple implementation: if pattern ends with *.mustache_json5, find all such files
        if pattern.ends_with("*.mustache_json5") || pattern.ends_with("*.mjson5") {
            let dir = if pattern.starts_with("*") {
                "."
            } else {
                Path::new(pattern)
                    .parent()
                    .unwrap_or(Path::new("."))
                    .to_str()
                    .unwrap()
            };

            files.extend(find_mustache_json5_files(dir)?);
        }

        Ok(files)
    } else {
        // Single file
        Ok(vec![pattern.to_string()])
    }
}
