// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use clap::Parser;
use std::path::PathBuf;
use tree::{fs::FileSystemNode, recurse};

// const BRANCH: &str = "│";
// const LEAF: &str = "├";
// const LAST_LEAF: &str = "└";

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let entry = FileSystemNode::new(args.directory)?;

    let print_path = move |path: &FileSystemNode, depth: u8| -> bool {
        if let Some(max_depth) = args.max_depth {
            if depth > max_depth {
                return false;
            }
        }

        if args.directories && !path.path().is_dir() {
            return true;
        }

        const HIDDEN_PREFIX: &[u8] = &[b'.'];
        if !args.hidden
            && path
                .path()
                .file_name()
                .is_some_and(|name| name.as_encoded_bytes().starts_with(HIDDEN_PREFIX))
        {
            return true;
        }

        let path = match args.full_path {
            true => format!("{path:#}"),
            false => format!("{path}"),
        };

        println!("{}{path}", "  ".repeat(depth as usize));
        true
    };

    recurse(&entry, move |path, depth| {
        if let Some(path) = path {
            return print_path(path, depth);
        }
        true
    });

    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about = "Print a tree of directories and files.", long_about = None)]
struct Args {
    /// Root directory to print.
    #[arg(value_parser)]
    directory: PathBuf,

    /// Show hidden directories and files.
    #[arg(short = 'a')]
    hidden: bool,

    /// Show only directories.
    #[arg(short = 'd')]
    directories: bool,

    /// Print the full path.
    #[arg(short = 'f')]
    full_path: bool,

    /// The maximum depth to recurse.
    #[arg(short = 'L')]
    max_depth: Option<u8>,
}
