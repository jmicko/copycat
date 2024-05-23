
# Project Specification

## 1. Project Overview

- **Project Name**: Copycat
- **Description**: A tool designed to concatenate multiple files in a project into a single text file with labels indicating file paths and names. This facilitates easy sharing of an entire codebase in a single step, particularly useful when working with AI or collaborative environments, particularly when said environment is pastebin. The tool will be built in Rust for *blazingly fast* and ease of installation.

## 2. Goals and Objectives

- **Primary Goal**: Simplify the process of sharing an entire codebase by concatenating all files with clear labels.
- **Secondary Goal**: Ensure the tool is fast, easy to use, and respects project-specific configurations.

## 3. Features and Requirements

- **Core Features**:
  1. Traverse a directory recursively to access all files, respecting `.gitignore`.
  2. Concatenate the contents of each file with a label indicating the file path and name.
  3. Format the concatenated code in markdown-style code blocks based on file type.
  4. Output the concatenated result to a single text file.
  5. Allow configuration via a config file for inclusion/exclusion rules and segmentation.

- **Additional Features**:
  1. Option to exclude certain file types or directories.
  2. Option to specify file extensions to include.
  3. Segment the output into different files by directories or file types.
  4. Set a length limit for the concatenated output.
  5. Watch mode to automatically update the concatenated file on changes.
  6. Ignore large files by default and provide configuration options for this limit.

## 4. Functional Requirements

- **Input**:
  1. Root directory of the codebase.
  2. Output file path and name.
  3. Optional: Configuration file for inclusion/exclusion rules, segmentation, and length limits.

- **Output**:
  1. A single text file containing the concatenated contents of all files in the specified directory, segmented as configured, with each file's content in a markdown-style code block.

- **Processing**:
  1. Traverse the directory tree starting from the root directory.
  2. Respect `.gitignore` and configuration file rules.
  3. For each file, determine the file type and format its contents in a markdown code block with a label indicating the file path.
  4. Segment the output as per configuration.
  5. Implement watch mode to monitor file changes and update the output file automatically.

## 5. Non-Functional Requirements

- **Performance**: The tool should process large codebases efficiently.
- **Usability**: Should have a simple and intuitive command-line interface.
- **Portability**: Should be platform-independent, running on major operating systems like Windows, macOS, and Linux.

## 6. Technical Stack

- **Programming Language**: Rust (*blazingly fast* and single binary distribution)
- **Libraries**:
  - `walkdir` for directory traversal
  - `ignore` for handling `.gitignore` files
  - `notify` for file watching
  - `clap` for command-line argument parsing
  - `serde` and `toml` for configuration file handling

## 7. Project Milestones

- **Milestone 1**: Initial prototype
  - Traverse directory and list files
  - Concatenate file contents with labels and markdown code blocks
  - Output to a single text file

- **Milestone 2**: Add configuration and segmentation
  - Implement `.gitignore` and custom config file handling
  - Add segmentation by directories or file types

- **Milestone 3**: Add advanced features
  - Implement file size limit and exclusion options
  - Implement watch mode for automatic updates

## 8. Risks and Mitigations

- **Risk**: Handling very large files or directories may slow down processing.
  - **Mitigation**: Implement efficient file reading and writing techniques.
- **Risk**: Users may find command-line options confusing.
  - **Mitigation**: Provide clear documentation and examples.

## 9. Future Enhancements

- VS Code extension graphical User Interface (GUI) for non-technical users.
- Additional customization options for file concatenation.

## 10. Documentation and Support

- **User Guide**: Detailed instructions on how to use the tool.
- **Developer Guide**: Instructions for contributing to the project.
