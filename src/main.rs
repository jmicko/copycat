use clap::Parser;
use ignore::WalkBuilder;
use std::env;
use std::fs::File;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(
    version = "1.0",
    about = "Concatenates multiple files into a single file with labels"
)]
struct Args {
    /// Sets the output file path
    #[arg(short, long, default_value = "concatenated_codebase.md")]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Use current directory if root is not specified
    let root_dir = env::current_dir()?;
    let output_file = args.output;

    let mut output = File::create(&output_file)?;

    for result in WalkBuilder::new(&root_dir)
        .add_custom_ignore_filename(".gitignore")
        .build()
    {
        let entry = result?;
        if entry.file_type().map_or(false, |ft| ft.is_file()) {
            let path = entry.path();
            if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                let code_block_label = match extension {
                    "rs" => "rust",
                    "js" => "javascript",
                    "ts" => "typescript",
                    "py" => "python",
                    // Add more file type mappings as needed
                    _ => "plaintext",
                };
                writeln!(output, "--- {} ---", path.display())?;
                writeln!(output, "```{}```\n", code_block_label)?;

                let content = std::fs::read_to_string(path)?;
                writeln!(output, "{}", content)?;
                writeln!(output, "```")?;
            }
        }
    }

    println!("Concatenated codebase written to {}", output_file);
    Ok(())
}
