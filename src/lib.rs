// Copyright 2017 Diggory Hardy and MaidSafe.net limited.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A vector-backed set optimised for storage space and read operations on ordered elements.
//! 
//! If you want to make frequent insertions and/or removals, a tree, hash or possibly unordered
//! vector based set is likely a better choice. This container is only intended for use where all
//! elements can be added in a single operation, or at least, where few insertions/deletions
//! happen.

use std::collections::BTreeSet;
use std::iter::FromIterator;
use std::ops::{Deref, Index};
use std::slice;
use std::vec;

#[cfg(feature = "rustc-serialize")]
extern crate rustc_serialize;

/// A sorted Vec type.
///
/// This is useful where you want a Vec which is guaranteed to be sorted.
#[derive(Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash, RustcEncodable,
    RustcDecodable)]
pub struct SortedVec<T: Ord> {
    v: Vec<T>,
}

// Currently we don't implement anything associated with modifying the vector,
// although many `Vec` operations could be implemented.
impl<T: Ord> SortedVec<T> {
    /// Construct a new, empty, `SortedVec<T>`.
    pub fn new() -> Self {
        SortedVec { v: vec![] }
    }

    /// Extracts a slice containing the entire vector.
    pub fn as_slice(&self) -> &[T] {
        self.v.as_slice()
    }

    /// Returns the number of elements in the vector.
    pub fn len(&self) -> usize {
        self.v.len()
    }

    /// Returns `true` if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }
}

impl<T: Ord> Deref for SortedVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.v.deref()
    }
}

impl<T: Ord> FromIterator<T> for SortedVec<T> {
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item = T>
    {
        let mut v = Vec::from_iter(iter);
        v.sort();
        SortedVec { v: v }
    }
}

impl<T: Ord> IntoIterator for SortedVec<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;
    fn into_iter(self) -> vec::IntoIter<T> {
        self.v.into_iter()
    }
}

impl<'a, T: Ord> IntoIterator for &'a SortedVec<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> slice::Iter<'a, T> {
        (&self.v).into_iter()
    }
}

impl<T: Ord> Index<usize> for SortedVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        self.v.index(index)
    }
}

impl<T: Ord> From<Vec<T>> for SortedVec<T> {
    fn from(mut v: Vec<T>) -> Self {
        v.sort();
        SortedVec { v: v }
    }
}

impl<T: Ord> From<BTreeSet<T>> for SortedVec<T> {
    fn from(t: BTreeSet<T>) -> Self {
        let mut v = Vec::from_iter(t.into_iter());
        v.sort();
        SortedVec { v: v }
    }
}
