// {{{ Copyright (c) Paul R. Tagliamonte <paultag@gmail.com>, 2024
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE. }}}

#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]

//! `ksq` is an implementation of a K2 tree (k²-tree), which, when storing
//! sparse bits, is a very space-effective matrix. This library implements
//! the tree as a flat 1-d array, rather than explicitly encoding
//! dimensionality.
//!
//! Unlike some other k2 trees, I've opted to use a `u16`, not a `u8`. This
//! means that the tree will grow by `N<<4` each layer -- and each cell can
//! represent a maximum of 16 other cells, not 8.

mod cell;
mod layer;
mod tree;
mod tree_iterator;

pub(crate) use cell::Cell;
pub(crate) use layer::Layer;
pub use tree::{Error, Tree};

pub(crate) use cell::CellRepr;

/// Crate-internal trait to abstract counting the number of set bits within
/// some value, or set bits until some stop offset.
pub(crate) trait CountOnes {
    /// count the set bits.
    fn count_ones(&self) -> usize;

    /// count the set bits, stopping at the provided offset.
    fn count_ones_until(&self, idx: usize) -> usize;
}

// vim: foldmethod=marker
