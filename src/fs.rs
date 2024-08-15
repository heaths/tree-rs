// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use crate::{HasChildren, Node};
use std::{
    fmt,
    fs::{DirEntry, ReadDir},
    io,
};

pub struct FileSystemNode {
    entry: DirEntry,
}

impl fmt::Debug for FileSystemNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileSystemNode")
            .field("entry", &self.entry.path())
            .finish()
    }
}

impl fmt::Display for FileSystemNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = self.entry.path();
        let s = if f.alternate() {
            path.file_name().ok_or(fmt::Error)?
        } else {
            path.as_os_str()
        };
        let s = s.to_str().ok_or(fmt::Error)?;
        if path.is_dir() {
            return f.write_fmt(format_args!("{}/", s));
        }

        f.write_str(s)
    }
}

impl From<DirEntry> for FileSystemNode {
    fn from(entry: DirEntry) -> Self {
        Self { entry }
    }
}

impl Node for FileSystemNode {
    fn has_children(&self) -> HasChildren {
        let path = self.entry.path();
        if path.is_dir() {
            return HasChildren::Maybe;
        }

        HasChildren::False
    }

    fn children(&self) -> impl Iterator {
        let path = self.entry.path();
        if !path.is_dir() {
            panic!("not a directory");
        }

        NodeIterator(path.read_dir())
    }
}

struct NodeIterator(io::Result<ReadDir>);

impl Iterator for NodeIterator {
    type Item = FileSystemNode;

    fn next(&mut self) -> Option<Self::Item> {
        let Ok(ref mut read) = self.0 else {
            return None;
        };

        #[allow(clippy::while_let_on_iterator)]
        while let Some(result) = read.next() {
            if let Ok(entry) = result {
                return Some(FileSystemNode { entry });
            }
        }

        None
    }
}
