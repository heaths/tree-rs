// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

mod error;
#[cfg(feature = "fs")]
pub mod fs;

pub use error::*;
use std::fmt::Display;

pub trait Node: Display {
    fn has_children(&self) -> HasChildren {
        HasChildren::Maybe
    }

    fn children(&self) -> impl Iterator;
}

#[derive(Debug, PartialEq)]
pub enum HasChildren {
    False,
    Maybe,
    True,
}
