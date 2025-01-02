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

use super::{Cell, Tree};
use std::ops::Range;

impl Tree {
    /// Iterate over all the bits in the tree. Once called, this will take
    /// a copy of the data in the [Tree], which means any changes to the tree
    /// during iteration will be ignored.
    pub fn iter(&self) -> impl Iterator<Item = bool> {
        self.iter_from_to(0, self.bits())
    }

    /// Iterate over all the set bits in the tree. Once called, this will take
    /// a copy of the data in the [Tree], which means any changes to the tree
    /// during iteration will be ignored.
    pub fn iter_ones(&self) -> impl Iterator<Item = usize> {
        self.iter_ones_from_to(0, self.bits())
    }

    /// Iterate over a subset of the bits in the tree. Once called, this will
    /// take a copy of the data in the [Tree], which means any changes to the
    /// tree during iteration will be ignored.
    pub fn iter_range(&self, range: Range<usize>) -> impl Iterator<Item = bool> {
        self.iter_from_to(range.start, range.end)
    }

    /// Iterate over a subset of the bits in the tree. Once called, this will
    /// take a copy of the data in the [Tree], which means any changes to the
    /// tree during iteration will be ignored.
    pub fn iter_ones_range(&self, range: Range<usize>) -> impl Iterator<Item = usize> {
        self.iter_ones_from_to(range.start, range.end)
    }

    /// Dump cells until we catch up to the commanded 'from' value.
    fn _scan_iter_forward(
        &self,
        mut iter: impl Iterator<Item = (usize, Cell)>,
        from: usize,
    ) -> Option<(usize, Cell)> {
        loop {
            match iter.next() {
                Some((offset, cell)) => {
                    if offset + Cell::bits() >= from {
                        return Some((offset, cell));
                    }
                }
                None => {
                    return None;
                }
            }
        }
    }

    /// Return an iterator over the tree.
    fn iter_from_to(&self, from: usize, to: usize) -> impl Iterator<Item = bool> {
        let leaf_layer = self.leaf_layer();
        let mut leaf_layer_iter = leaf_layer.clone().into_iter();
        let mut leaf_layer_cur = leaf_layer_iter.next();

        if from >= Cell::bits() {
            // here we need to scan forward until the leaf_layer_cur is
            // gte from

            if let Some((offset, _)) = leaf_layer_cur {
                if (offset + Cell::bits()) <= from {
                    leaf_layer_cur = self._scan_iter_forward(&mut leaf_layer_iter, from);
                }
            }
        }

        LeafIterator {
            index: from,
            bits: to,

            leaf_layer_cur,
            leaf_layer_iter,
            _leaf_layer: leaf_layer,
        }
    }

    /// Return a ones iterator over the tree.
    fn iter_ones_from_to(&self, from: usize, to: usize) -> impl Iterator<Item = usize> {
        let leaf_layer = self.leaf_layer();
        let mut leaf_layer_iter = leaf_layer.clone().into_iter();
        let mut leaf_layer_cur = leaf_layer_iter.next();

        if from >= Cell::bits() {
            if let Some((offset, _)) = leaf_layer_cur {
                if (offset + Cell::bits()) <= from {
                    leaf_layer_cur = self._scan_iter_forward(&mut leaf_layer_iter, from);
                }
            }
        }

        LeafIteratorOnes {
            index: from.max(leaf_layer_cur.map(|(v, _)| v).unwrap_or(0)),
            bits: to,

            leaf_layer_cur,
            leaf_layer_iter,
            _leaf_layer: leaf_layer,
        }
    }
}

/// Iterator which takes a copy of the leaf layer, aligned with the starting
/// offset(s) of the leaf Cell values.
struct LeafIterator<IterT>
where
    IterT: Iterator<Item = (usize, Cell)>,
{
    index: usize,
    bits: usize,

    // needed for ownership reasons
    _leaf_layer: Vec<(usize, Cell)>,

    leaf_layer_cur: Option<(usize, Cell)>,
    leaf_layer_iter: IterT,
}

impl<IterT> Iterator for LeafIterator<IterT>
where
    IterT: Iterator<Item = (usize, Cell)>,
{
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.index >= self.bits {
            return None;
        }

        let Some((offset, cell)) = self.leaf_layer_cur else {
            // we're at the end of our layer, but we need to slide on through
            // to the end of the tree itself. let's inject some trailing false
            // as required.
            if self.index < self.bits {
                self.index += 1;
                return Some(false);
            }

            unreachable!();
        };

        // if we're before the next cell, let's return some falses
        if self.index < offset {
            self.index += 1;
            return Some(false);
        }

        let bit_index = self.index - offset;
        let value = cell.get(bit_index);

        self.index += 1;

        // check to see if we need to get the next cell; if we've just
        // handled the last bit of the cell.
        if self.index >= offset + Cell::bits() {
            self.leaf_layer_cur = self.leaf_layer_iter.next();
        }

        Some(value)
    }
}

/// Iterator which takes a copy of the leaf layer, aligned with the starting
/// offset(s) of the leaf Cell values.
struct LeafIteratorOnes<IterT>
where
    IterT: Iterator<Item = (usize, Cell)>,
{
    index: usize,
    bits: usize,

    // needed for ownership reasons
    _leaf_layer: Vec<(usize, Cell)>,

    leaf_layer_cur: Option<(usize, Cell)>,
    leaf_layer_iter: IterT,
}

impl<IterT> Iterator for LeafIteratorOnes<IterT>
where
    IterT: Iterator<Item = (usize, Cell)>,
{
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        loop {
            if self.index >= self.bits {
                return None;
            }
            let (offset, cell) = self.leaf_layer_cur?;
            if self.index < offset {
                self.index = offset;
            }

            let idx = self.index;
            self.index += 1;

            let bit_index = idx - offset;

            if self.index >= offset + Cell::bits() {
                self.leaf_layer_cur = self.leaf_layer_iter.next();
            }

            if cell.get(bit_index) {
                return Some(idx);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_iter() {
        let tree = Tree::from(&[1, 10]).unwrap();

        let v: Vec<bool> = tree.iter().collect();
        let mut r = vec![false; 256];
        r[1] = true;
        r[3] = true;
        assert_eq!(r, v);
    }

    #[test]
    fn tree_mid_iter() {
        let tree = Tree::from(&[2, 10]).unwrap();

        assert!(tree.get(17));
        assert!(tree.get(19));

        let v: Vec<bool> = tree.iter_range(16..32).collect();
        let mut r = vec![false; 16];
        r[1] = true;
        r[3] = true;
        assert_eq!(r, v);
    }

    #[test]
    fn tree_iter_ones() {
        let tree = Tree::from(&[2, 10]).unwrap();

        assert!(tree.get(17));
        assert!(tree.get(19));

        let v: Vec<usize> = tree.iter_ones().collect();
        assert_eq!(vec![17, 19], v);
    }
}

// vim: foldmethod=marker
