use std::fs;
use std::io::{self};
use std::path::Path;
use std::process::Command as ProcessCommand;

use clap::{Arg, Command};
use regex::Regex;
use walkdir::WalkDir;

const HEADER_PATTERN: &str = r"(?s)^\s*/\*.*?\*/";

fn replace_header(file_path: &Path, new_header: &str) -> io::Result<()> {
    let content = fs::read_to_string(file_path)?;

    // Compile regex for the existing header
    let re = Regex::new(HEADER_PATTERN).unwrap();

    // Check if the file has an existing header
    let existing_header = if let Some(matched) = re.find(&content) {
        matched.as_str()
    } else {
        ""
    };

    if existing_header != new_header {
        let new_content = if re.is_match(&content) {
            println!("Replacing header in file {:?}", file_path);
            re.replace_all(&content, new_header).to_string()
        } else {
            println!("Adding header to file {:?}", file_path);
            format!("{}\n{}", new_header, content)
        };

        // Write the updated content back to the file
        fs::write(file_path, new_content)?;
    } else {
        println!("Skipping file {:?} as the header is already up-to-date.", file_path);
    }

    Ok(())
}

fn read_header_from_file(header_file_path: &Path) -> io::Result<String> {
    fs::read_to_string(header_file_path)
}

fn get_remote_url(root_dir: &Path) -> io::Result<String> {
    let output = ProcessCommand::new("git")
        .current_dir(root_dir)
        .args(["remote", "get-url", "origin"])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to get remote URL from git",
        ));
    }

    let url = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Remove the trailing .git if present
    let trimmed_url = if url.ends_with(".git") {
        url.trim_end_matches(".git").to_string()
    } else {
        url
    };

    Ok(trimmed_url)
}

fn replace_origin_in_header(new_header: &str, origin: &str) -> String {
    new_header.replace("$origin", origin)
}

fn process_files(root_dir: &Path, new_header: &str) -> io::Result<()> {
    let re_extensions = Regex::new(r"\.(rs|cs)$").unwrap();

    for entry in WalkDir::new(root_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let path = entry.path();
        if !re_extensions.is_match(path.to_str().unwrap()) {
            continue;
        }
        replace_header(path, new_header)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let matches = Command::new("Comment Header Replacer")
        .version("0.1")
        .author("Peter Bjorklund <piot@hotmail.com>")
        .about("Replaces or adds a header in source files (Rust and C#)")
        .arg(
            Arg::new("path")
                .long("path")
                .required(true)
                .value_name("DIRECTORY")
                .help("The root directory to scan for source files"),
        )
        .arg(
            Arg::new("license")
                .long("license")
                .required(true)
                .value_name("FILE")
                .help("Path to the file containing the new header"),
        )
        .get_matches();

    let root_dir = Path::new(matches.get_one::<String>("path").unwrap());
    let header_file_path = Path::new(matches.get_one::<String>("license").unwrap());

    let new_header = read_header_from_file(header_file_path)?;

    let remote_url = get_remote_url(root_dir)?;

    let final_header = replace_origin_in_header(&new_header, &remote_url);

    process_files(root_dir, &final_header)?;

    println!("Comment Header done.");

    Ok(())
}
