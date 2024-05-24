use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

use crate::utils::get_code_block_label;

pub fn load_gitignore_patterns(root_dir: &Path) -> Vec<String> {
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

pub fn classify_patterns(patterns: Vec<String>) -> Vec<(String, bool, bool)> {
    patterns
        .into_iter()
        .map(|pattern| {
            let is_glob = pattern.contains('*') || pattern.contains('?');
            let is_path = pattern.contains('/') || pattern.ends_with('/');
            (pattern, is_glob, is_path)
        })
        .collect()
}

pub fn should_exclude(
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

    classified_patterns
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
        })
}

pub fn concatenate_file(path: &Path, output: &mut File, root_dir: &Path) -> io::Result<()> {
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

pub fn walk_and_concatenate(
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
                if super::file_handling::should_exclude(&path, patterns, root_dir, ignore_hidden) {
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
                        super::file_handling::concatenate_file(&path, output, root_dir)?;
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
