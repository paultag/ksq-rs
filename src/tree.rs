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

use super::{
    Cell, CellRepr, CountOnes, Layer,
    std::{vec, vec::Vec},
};

/// A `tree` is the user-facing 1-dimensional bit vector. The `tree` can store
/// a fixed number of bits, which can be accessed using [Tree::get],
/// [Tree::set] or maybe [Tree::unset]
#[derive(Debug, Clone, PartialEq)]
pub struct Tree(Vec<Layer>);

/// Possible error types which may be returned by the [Tree] during
/// construction.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    /// The bytewise encoding of the K2 [Tree] is malformed -- usually this
    /// means some chunks of a layer are missing.
    Malformed,

    /// No data was provided, so no [Tree] can be constructed.
    Empty,
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}

impl Tree {
    /// Create a new [Tree] with the smallest possible capacity, `16` bits,
    /// in this case. You may [Tree::grow] to increase the bit capacity of
    /// the [Tree].
    pub fn new() -> Self {
        Tree(vec![Layer(vec![0.into()])])
    }

    /// Construct a new K2 [Tree] from a set of `u16` "Cells". The bytewise
    /// encoding of the K2 [Tree] is self-describing, so no additional data
    /// beyond the underlying values is required.
    pub fn from(v: &[CellRepr]) -> Result<Self, Error> {
        if v.is_empty() {
            return Err(Error::Empty);
        }

        let mut tree = vec![
            // create a tree (vec of layers).
            Layer::from([v[0].into()]),
        ];

        let mut v = &v[1..];
        while !v.is_empty() {
            let layer_len = tree[tree.len() - 1].count_ones();

            if layer_len == 0 && !v.is_empty() {
                return Err(Error::Malformed);
            }

            if layer_len > v.len() {
                return Err(Error::Malformed);
            }

            tree.push(Layer::from(v[0..layer_len].iter().map(|v| (*v).into())));
            v = &v[layer_len..];
        }

        Ok(Self(tree))
    }

    /// Return the largest bit offset representable given the current height
    /// of the [Tree]. If additional capacity is required, the tree can be
    /// grown using [Tree::grow].
    pub fn bits(&self) -> usize {
        Cell::bits() << (4 * (self.0.len() - 1))
    }

    /// Return the height of the tree.
    pub fn height(&self) -> usize {
        self.0.len()
    }

    /// Grow a [Tree] by one "level". The current implementation will grow by
    /// `1<<4` each time, due to the current Cell type.
    pub fn grow(&mut self) {
        self.0.insert(0, Layer(vec![1.into()]));
    }

    /// Return true/false if the requested bit is set/unset. If the bit
    /// is out of range, a panic will be triggered.
    pub fn get(&self, bit: usize) -> bool {
        if self.bits() <= bit {
            panic!("bit out of range {} (max={})", bit, self.bits());
        }
        let mut next_offset = 0;
        let mut set = false;
        for height in (0..self.0.len()).rev() {
            let layer_index = (self.0.len() - height) - 1;
            (next_offset, set) = self.0[layer_index].get((height, next_offset, bit));
            if !set {
                return false;
            }
        }
        set
    }

    /// Set the requested bit to true. If the bit is out of range, a panic
    /// will be triggered.
    pub fn set(&mut self, bit: usize) {
        if self.bits() <= bit {
            panic!("bit out of range {} (max={})", bit, self.bits());
        }
        let mut next_offset = 0;
        let mut should_create = false;
        for height in (0..self.0.len()).rev() {
            let layer_index = (self.0.len() - height) - 1;

            if should_create {
                self.0[layer_index].insert_cell(next_offset, Default::default());
            }
            (next_offset, should_create) = self.0[layer_index].set((height, next_offset, bit));
        }
    }

    /// Set the requested bit to false. This will *only* set the lowest level
    /// of bits in the tree, and will *not* remove layers which may be pruned.
    ///
    /// This means setting all values to true, then unsetting them all will
    /// result in a different tree than initalizing the tree with only the
    /// required set bits.
    pub fn unset(&mut self, bit: usize) {
        if self.bits() <= bit {
            panic!("bit out of range {} (max={})", bit, self.bits());
        }
        let mut next_offset = 0;
        for height in (0..self.0.len()).rev() {
            let layer_index = (self.0.len() - height) - 1;
            let li = (height, next_offset, bit);

            if height == 0 {
                self.0[layer_index].unset(li);
            } else {
                let set;
                (next_offset, set) = self.0[layer_index].get(li);
                if !set {
                    return;
                }
            }
        }
    }

    /// Turn the tree into a [Vec] of Cells -- this can be exported,
    /// and later re-loaded to create the same [Tree] again.
    pub fn to_vec(&self) -> Vec<CellRepr> {
        let mut ret = vec![];
        for layer in self.0.iter() {
            ret.append(&mut layer.to_vec());
        }
        ret
    }

    /// Return a mapping of Cells and their starting offset in the tree.
    pub(crate) fn leaf_layer(&self) -> Vec<(usize, Cell)> {
        // for each layer from the top down, let's compute the starting indexes
        // for each one.

        // the top layer starts at 0, ends at self.bits(), always.
        let mut layer_map = vec![0];

        for layer_index in 0..(self.0.len() - 1) {
            let height = self.0.len() - layer_index - 1;

            // number of bits that a 1 represents. At the highest level, this
            // is the number of bits representable in the tree. At the lowest
            // level this is '1' bit per bit.
            let bits_per_bit = 1 << (4 * height);

            let mut next_layer_map = vec![];

            for (cell, offset) in self.0[layer_index].0.iter().zip(layer_map.iter()) {
                for idx in 0..16usize {
                    if cell.get(idx) {
                        // if set, let's add it to the map
                        next_layer_map.push(offset + (bits_per_bit * idx));
                    }
                }
            }

            layer_map = next_layer_map;
        }

        layer_map
            .iter()
            .zip(self.0[self.0.len() - 1].0.iter())
            .map(|(idx, cell)| ((*idx), *cell))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_parse() {
        Tree::from(&[0]).unwrap();
    }

    #[test]
    fn tree_parse_111() {
        let tree = Tree::from(&[1, 1, 1]).unwrap();
        assert_eq!(4096, tree.bits());
    }

    #[test]
    fn tree_parse_111_get() {
        let tree = Tree::from(&[1, 1, 1]).unwrap();
        assert!(tree.get(0));
        assert!(!tree.get(1));
        assert!(!tree.get(4095));
    }

    #[test]
    fn tree_parse_00_error() {
        assert!(Tree::from(&[0, 0]).is_err());
    }

    #[test]
    fn tree_mega() {
        let mut tree = Tree::from(&[1, 1, 0]).unwrap();
        for idx in 0..4096 {
            assert!(!tree.get(idx));
            tree.set(idx);
            assert!(tree.get(idx));
        }
    }

    #[test]
    fn tree_mega_loopback() {
        let mut tree = Tree::from(&[1, 1, 0]).unwrap();
        for idx in 0..4096 {
            assert!(!tree.get(idx));
            tree.set(idx);
            assert!(tree.get(idx));

            {
                let tree = Tree::from(&tree.to_vec()).unwrap();
                assert!(tree.get(idx));
            }

            tree.unset(idx);
            assert!(!tree.get(idx));

            {
                let tree = Tree::from(&tree.to_vec()).unwrap();
                assert!(!tree.get(idx));
            }
        }
    }

    #[test]
    fn tree_mega_unset() {
        let mut tree = Tree::from(&[1, 1, 0]).unwrap();
        for idx in 0..4096 {
            assert!(!tree.get(idx));
            tree.set(idx);
            assert!(tree.get(idx));
            tree.unset(idx);
            assert!(!tree.get(idx));
        }
    }
}

// vim: foldmethod=marker
