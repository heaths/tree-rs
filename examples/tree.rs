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

    recurse(&entry, |path, depth| {
        if let Some(path) = path {
            println!("{}{path:#}", "  ".repeat((depth - 1) as usize));
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
}
