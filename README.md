# Comment Header Replacer

A command-line tool written in Rust for replacing or adding a comment header to source files.
This tool is particularly useful for managing and standardizing license headers across multiple files in a codebase.

## Features

- **Replace or Add Headers**: Automatically replaces existing headers or adds a new header to source files.
- **Supports Multiple Languages**: Works with Rust (`.rs`) and C# (`.cs`) files.
- **Customizable Headers**: Use a custom header read from a file and replace placeholders with dynamic content such as the repository URL.

## Installation

To build and run this tool, you'll need to have Rust installed. You can install Rust from [rustup.rs](https://rustup.rs).

  1. Clone the repository:

```sh
git clone https://github.com/piot/comment-header-rs.git
cd comment-header-rs
```

  2. Build the project:

```sh
cargo build --release
```

The compiled binary will be located in the `target/release` directory.

## Usage

The tool is invoked from the command line and takes the following arguments:

```sh
comment-header-rs --path <DIRECTORY> --license <FILE>
```

## Arguments

- `--path <DIRECTORY>`: The root directory to recursively scan for source files.
- `--license <FILE>`: Path to the file containing the new header.

## Template Variables

- `$origin`. the git URL to the origin remote.

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
./target/release/comment-header-rs --path src --license header.txt
```

This will replace or add the header to all Rust and C# files in the src directory.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.
