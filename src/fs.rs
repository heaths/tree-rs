// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use crate::{HasChildren, Node};
use std::{
    fmt,
    fs::{DirEntry, ReadDir},
    io,
    path::PathBuf,
};

pub struct FileSystemNode {
    entry: DirEntry,
}

impl FileSystemNode {
    pub fn path(&self) -> PathBuf {
        self.entry.path()
    }

    #[cfg(test)]
    #[allow(dead_code)]
    fn is_dir(&self) -> io::Result<bool> {
        Ok(self.entry.metadata()?.is_dir())
    }

    #[cfg(test)]
    #[allow(dead_code)]
    fn is_file(&self) -> io::Result<bool> {
        Ok(self.entry.metadata()?.is_file())
    }
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

#[cfg(test)]
mod tests {
    use std::fs::read_dir;

    use super::*;

    #[test]
    fn has_children() {
        let mut found_dir = false;
        let mut found_file = false;

        let dir = NodeIterator(read_dir(env!("CARGO_MANIFEST_DIR")));
        for node in dir {
            if node.path().is_dir() {
                found_dir = true;
                assert!(node.to_string().ends_with("/"));
            }
            if node.path().is_file() {
                found_file = true;
                assert!(!node.to_string().ends_with("/"));
            }
        }

        assert!(found_dir);
        assert!(found_file);
    }
}
