// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#[cfg(feature = "fs")]
pub mod fs;

use std::fmt::Display;

/// Represents a branch of the tree.
pub trait Node: Display {
    /// The type of iterator to return from [`Node::nodes()`].
    type Iter: Iterator<Item = Self>;

    /// Whether this node may have children.
    ///
    /// This should return `false` for leave nodes,
    /// but `true` if the node is a container and may have children.
    fn has_nodes(&self) -> bool {
        false
    }

    /// Gets any child nodes of this node.
    ///
    /// This should return an empty iterator if [`Node::has_nodes()`] returns `false`.
    fn nodes(&self) -> Self::Iter;
}

// [TODO]
// To enumerate children efficiently while maintaining the ability to sort,
// have a function we can pass in to sort (which may or may not sort) -
// might even consider a Fn to the provider if it can do so more efficiently.
// The loop for each level should fetch the next node and determine if its
// the last one before recursing into the children e.g.,

pub fn recurse<T, F>(node: &T, f: F)
where
    T: Node,
    F: Fn(Option<&T>, u8) -> bool + Copy + 'static,
{
    fn inner<T, F>(node: &T, f: F, depth: u8)
    where
        T: Node,
        F: Fn(Option<&T>, u8) -> bool + Copy + 'static,
    {
        let mut nodes: Vec<T> = node.nodes().collect();
        nodes.sort_by_key(|a| a.to_string());

        let mut iter = nodes.iter();
        loop {
            let next = iter.next();
            if !f(next, depth + 1) {
                break;
            }

            if let Some(cur) = next {
                if cur.has_nodes() {
                    inner(cur, f, depth + 1);
                }
            } else {
                break;
            }
        }
    }

    if !f(Some(node), 0) || !node.has_nodes() {
        return;
    }

    inner(node, f, 0);

    // // We'd want to sort - or simply pass through - this, so the following algorithm works unconditionally.
    // let mut iter = nodes.into_iter();

    // let mut next = iter.next();
    // while let Some(current) = next {
    //     next = iter.next();

    //     let line = if next.is_none() { "└ " } else { "├ " };
    //     if current.has_nodes() {
    //         recurse(current.nodes())
    //     }
    // }
}
