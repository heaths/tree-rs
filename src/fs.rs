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
            .field("entry", &self.to_string())
            .finish()
    }
}

impl fmt::Display for FileSystemNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = self.entry.path();
        let s = if f.alternate() {
            path.as_os_str()
        } else {
            path.file_name().ok_or(fmt::Error)?
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
    use super::*;
    use std::{ffi::OsStr, fs::read_dir, path::Path};

    #[test]
    fn has_children() {
        let mut found_dir = false;
        let mut found_file = false;

        let iter = NodeIterator(read_dir(env!("CARGO_MANIFEST_DIR")));
        for node in iter {
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

    #[test]
    fn display() {
        let nodes = FileSystemNodes::new().unwrap();

        assert_eq!("src/", &nodes.src_dir.to_string());
        assert_eq!(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("src/")
                .display()
                .to_string(),
            format!("{:#}", &nodes.src_dir)
        );

        assert_eq!("Cargo.toml", &nodes.manifest_file.to_string());
        assert_eq!(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("Cargo.toml")
                .display()
                .to_string(),
            format!("{:#}", &nodes.manifest_file),
        );
    }

    #[test]
    fn debug() {
        let nodes = FileSystemNodes::new().unwrap();

        assert_eq!(
            "FileSystemNode { entry: \"src/\" }",
            &format!("{:?}", &nodes.src_dir)
        );
        assert_eq!(
            "FileSystemNode { entry: \"Cargo.toml\" }",
            &format!("{:?}", &nodes.manifest_file)
        );
    }

    struct FileSystemNodes {
        src_dir: FileSystemNode,
        manifest_file: FileSystemNode,
    }

    impl FileSystemNodes {
        fn new() -> io::Result<Self> {
            let mut src_dir: Option<FileSystemNode> = None;
            let mut manifest_file: Option<FileSystemNode> = None;

            let iter = NodeIterator(read_dir(env!("CARGO_MANIFEST_DIR")));
            for entry in iter {
                if entry.is_dir()? && entry.path().file_name() == Some(OsStr::new("src")) {
                    src_dir = Some(entry);
                } else if entry.is_file()?
                    && entry.path().file_name() == Some(OsStr::new("Cargo.toml"))
                {
                    manifest_file = Some(entry);
                }
            }

            if let (Some(src_dir), Some(manifest_file)) = (src_dir, manifest_file) {
                return Ok(Self {
                    src_dir,
                    manifest_file,
                });
            }

            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "src/ or Cargo.toml not found",
            ))
        }
    }
}
