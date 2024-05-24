pub fn get_code_block_label(extension: &str) -> &str {
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
