# Comment Header Replacer

A command-line tool written in Rust for replacing or adding a comment header to source files.
This tool is particularly useful for managing and standardizing license headers across multiple files in a codebase.

## Features

- **Replace or Add Headers**: Automatically replaces existing headers or adds a new header to source files.
- **Supports Multiple Languages**: Compatible with programming languages that utilize `/*` and `*/` for multi-line comments, including Rust, Swift, C#, C, C++, TypeScript, and others.
- **Customizable Headers**: Use a custom header read from a file and replace placeholders with dynamic content such as the repository URL.

## Installation

To build and run this tool, you'll need to have Rust installed. You can install Rust from [rustup.rs](https://rustup.rs).

```sh
cargo install comment-header-rs
```

## Usage

The tool is invoked from the command line and takes the following arguments:

```sh
comment-header-rs --path <DIRECTORY> --license <FILE>
```

### Arguments

- `--path <DIRECTORY>`: The root directory to recursively scan for source files.
- `--license <FILE>`: Path to the file containing the new header.
- `--extensions <EXTENSIONS>`: Comma-separated list of file extensions. (default: `rs,cs`).

## Template Variables

You can include specific variables in your license file that will be automatically replaced with relevant values when the header is added to your source files. This allows for dynamic content, making your headers more informative and tailored to your project.

### Available Variables

- `$origin`. This variable will be replaced with the Git URL of the origin remote repository. It provides a direct link to the repository where the source code is hosted, which can be useful for reference and compliance.

## Example

Assuming you have a header file `header.txt` and want to apply it recursively to all `.rs` and `.cs` files in the src directory:

Create a header file, `header.txt`, with the desired content, eg:

```txt
/*
 * Copyright (c) Your Name. All rights reserved.
 * Licensed under the MIT License.
 */
```

Run the tool:

```sh
comment-header-rs --path src --license header.txt
```

This will replace or add the header to all Rust and C# files in the src directory.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
