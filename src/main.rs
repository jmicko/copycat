mod config;
mod file_handling;
mod utils;

use clap::Parser;
use config::Config;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(
    version = env!("CARGO_PKG_VERSION"),
    about = "Concatenates multiple files into a single file with labels"
)]
struct Args {
    /// Sets the output file path
    #[arg(short, long, default_value = "copycat/concatenated_codebase.md")]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();

    // Use current directory if root is not specified
    let root_dir = env::current_dir()?;
    let copycat_dir = root_dir.join("copycat");
    let config_path = copycat_dir.join("copycat_config.toml");

    // Ensure copycat directory exists
    fs::create_dir_all(&copycat_dir)?;

    // Check if config file exists and prompt the user
    let config = if config_path.exists() {
        Config::load_from_file(&config_path)?
    } else {
        println!("Config file not found. Do you want to create a default config file? (yes/y/no/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input_trimmed = input.trim().to_lowercase();
        if input_trimmed == "yes" || input_trimmed == "y" {
            let default_config = Config::default();
            default_config.save_to_file(&config_path)?;
            println!(
                "Default config created at {}. Please review and run again.",
                config_path.display()
            );
            return Ok(());
        } else {
            println!("Exiting without creating a config file.");
            return Ok(());
        }
    };

    // Check if copycat directory is in .gitignore
    let gitignore_path = root_dir.join(".gitignore");
    if let Ok(mut gitignore_file) = OpenOptions::new()
        .read(true)
        .append(true)
        .open(&gitignore_path)
    {
        let reader = BufReader::new(&gitignore_file);
        let mut copycat_ignored = false;

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.trim() == "/copycat" {
                    copycat_ignored = true;
                    break;
                }
            }
        }

        if !copycat_ignored {
            println!("Do you want to add /copycat to .gitignore? (yes/y/no/n)");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input_trimmed = input.trim().to_lowercase();
            if input_trimmed == "yes" || input_trimmed == "y" {
                writeln!(gitignore_file, "/copycat")?;
                println!("/copycat added to .gitignore.");
            } else {
                println!("/copycat not added to .gitignore.");
            }
        }
    }

    // Load gitignore patterns
    let gitignore_patterns = file_handling::load_gitignore_patterns(&root_dir);

    // Combine config exclude patterns and gitignore patterns
    let all_patterns: Vec<String> = config
        .exclude
        .into_iter()
        .chain(gitignore_patterns)
        .collect();

    // Classify all patterns
    let classified_patterns = file_handling::classify_patterns(all_patterns);

    // Open output file
    let mut output = File::create(&config.output)?;

    // Derive project name from the root directory name
    let project_name = root_dir
        .file_name()
        .unwrap_or_else(|| root_dir.as_os_str())
        .to_str()
        .unwrap_or("Project");

    // Write top-level heading to the output file
    writeln!(output, "# {} codebase\n", project_name)?;

    // Walk and concatenate files
    file_handling::walk_and_concatenate(
        &root_dir,
        &classified_patterns,
        config.ignore_hidden,
        &mut output,
        100,
    )?;

    println!("Concatenated codebase written to {}", config.output);
    Ok(())
}
