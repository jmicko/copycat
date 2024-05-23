use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    version = env!("CARGO_PKG_VERSION"),
    about = "Concatenates multiple files into a single file with labels"
)]
struct Args {
    /// Sets the output file path
    #[arg(short, long, default_value = "concatenated_codebase.md")]
    output: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    output: String,
    exclude: Vec<String>,
    include: Vec<String>,
    ignore_hidden: bool,
}

impl Config {
    fn default() -> Self {
        Config {
            output: "concatenated_codebase.md".to_string(),
            exclude: vec![
                "*.lock".to_string(),
                "*.md".to_string(),
                // "/target".to_string(),
            ],
            include: vec![],
            ignore_hidden: true,
        }
    }

    fn load_from_file(file_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    fn save_to_file(&self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string(self)?;
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

fn load_gitignore_patterns(root_dir: &Path) -> Vec<String> {
    let gitignore_path = root_dir.join(".gitignore");
    let mut patterns = Vec::new();

    if let Ok(file) = File::open(gitignore_path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if !line.trim().is_empty() && !line.starts_with('#') {
                    patterns.push(line);
                }
            }
        }
    }

    patterns
}

fn classify_patterns(patterns: Vec<String>) -> Vec<(String, bool, bool)> {
    patterns
        .into_iter()
        .map(|pattern| {
            let is_glob = pattern.contains('*') || pattern.contains('?');
            let is_path = pattern.contains('/') || pattern.ends_with('/');
            (pattern, is_glob, is_path)
        })
        .collect()
}

fn should_exclude(
    path: &Path,
    classified_patterns: &[(String, bool, bool)],
    root_dir: &Path,
    ignore_hidden: bool,
) -> bool {
    let relative_path = path
        .strip_prefix(root_dir)
        .unwrap_or(path)
        .to_str()
        .unwrap_or_default();
    let filename = path
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    if ignore_hidden && filename.starts_with('.') {
        return true;
    }

    let result = classified_patterns
        .iter()
        .any(|(pattern, is_glob, is_path)| {
            let normalized_pattern = if pattern.starts_with('/') {
                &pattern[1..]
            } else {
                pattern
            };
            if *is_path {
                if *is_glob {
                    glob::Pattern::new(normalized_pattern)
                        .map_or(false, |p| p.matches(relative_path))
                } else {
                    relative_path == normalized_pattern
                        || relative_path.starts_with(normalized_pattern.trim_end_matches('/'))
                }
            } else {
                if *is_glob {
                    glob::Pattern::new(normalized_pattern).map_or(false, |p| p.matches(filename))
                } else {
                    filename == normalized_pattern
                }
            }
        });

    println!(
        "Checking path: {}, Relative path: {}, Filename: {}, Excluded: {}",
        path.display(),
        relative_path,
        filename,
        result
    );

    result
}

fn get_code_block_label(extension: &str) -> &str {
    match extension {
        "rs" => "rust",
        "js" => "javascript",
        "ts" => "typescript",
        "py" => "python",
        "html" => "html",
        "css" => "css",
        "java" => "java",
        "c" => "c",
        "cpp" => "cpp",
        "cs" => "csharp",
        "sh" => "bash",
        "json" => "json",
        // Add more file type mappings as needed
        _ => extension,
    }
}

fn concatenate_file(path: &Path, output: &mut File, root_dir: &Path) -> io::Result<()> {
    let relative_path = path.strip_prefix(root_dir).unwrap_or(path);
    writeln!(output, "--- {} ---\n", relative_path.display())?;

    if let Some(extension) = path.extension().and_then(std::ffi::OsStr::to_str) {
        writeln!(output, "```{}", get_code_block_label(extension))?;
    } else {
        writeln!(output, "```plaintext")?;
    }

    match fs::read_to_string(path) {
        Ok(content) => writeln!(output, "{}", content)?,
        Err(_) => writeln!(output, "<file content not valid UTF-8>")?,
    }
    writeln!(output, "```\n")?;

    Ok(())
}

fn walk_and_concatenate(
    root_dir: &Path,
    classified_patterns: &[(String, bool, bool)],
    ignore_hidden: bool,
    output: &mut File,
    limit: usize,
) -> io::Result<()> {
    fn visit_dirs(
        dir: &Path,
        patterns: &[(String, bool, bool)],
        root_dir: &Path,
        ignore_hidden: bool,
        output: &mut File,
        count: &mut usize,
        limit: usize,
    ) -> io::Result<()> {
        if *count >= limit {
            return Ok(());
        }

        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if should_exclude(&path, patterns, root_dir, ignore_hidden) {
                    if path.is_dir() {
                        println!("Skipping directory: {}", path.display());
                    } else {
                        println!("Skipping file: {}", path.display());
                    }
                } else {
                    if path.is_dir() {
                        println!("Entering directory: {}", path.display());
                        visit_dirs(
                            &path,
                            patterns,
                            root_dir,
                            ignore_hidden,
                            output,
                            count,
                            limit,
                        )?;
                    } else {
                        println!("Including file: {}", path.display());
                        concatenate_file(&path, output, root_dir)?;
                        *count += 1;
                    }
                }
            }
        }
        Ok(())
    }

    let mut count = 0;
    visit_dirs(
        root_dir,
        classified_patterns,
        root_dir,
        ignore_hidden,
        output,
        &mut count,
        limit,
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();

    // Use current directory if root is not specified
    let root_dir = env::current_dir()?;
    let config_path = root_dir.join("copycat_config.toml");

    // Check if config file exists
    let config = if config_path.exists() {
        Config::load_from_file(&config_path)?
    } else {
        println!("Config file not found. Creating a default config file.");
        let default_config = Config::default();
        default_config.save_to_file(&config_path)?;
        println!(
            "Default config created at {}. Please review and run again.",
            config_path.display()
        );
        return Ok(());
    };

    // Load gitignore patterns
    let gitignore_patterns = load_gitignore_patterns(&root_dir);

    // Combine config exclude patterns and gitignore patterns
    let all_patterns: Vec<String> = config
        .exclude
        .into_iter()
        .chain(gitignore_patterns)
        .collect();

    // Classify all patterns
    let classified_patterns = classify_patterns(all_patterns);

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
    walk_and_concatenate(
        &root_dir,
        &classified_patterns,
        config.ignore_hidden,
        &mut output,
        100,
    )?;

    println!("Concatenated codebase written to {}", config.output);
    Ok(())
}
