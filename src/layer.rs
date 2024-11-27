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

use super::{Cell, CellRepr, CountOnes};

/// A [Layer] is a collection of [Cell]s.
///
/// If you're looking to get or set data, a [Layer] is usually the wrong
/// abstraction to use, usually you want a [crate::Tree]. Very few knobs are
/// exposed on a Layer (intentionally), since they have to be used within
/// the context of all the other layers.
#[derive(Debug, Clone, PartialEq)]
pub struct Layer(pub(crate) Vec<Cell>);

impl CountOnes for Layer {
    fn count_ones(&self) -> usize {
        self.0.count_ones()
    }

    fn count_ones_until(&self, idx: usize) -> usize {
        self.0.count_ones_until(idx)
    }
}

impl CountOnes for [Cell] {
    fn count_ones(&self) -> usize {
        let mut r = 0;
        for cell in self.iter() {
            r += cell.count_ones();
        }
        r
    }

    fn count_ones_until(&self, idx: usize) -> usize {
        let mut r = 0;
        for cell in self[0..idx].iter() {
            r += cell.count_ones();
        }
        r
    }
}

/// A "Layer Index" is the height, the cell-wise offset into the layer
/// (which is derived from the next higher layer), plus the bitwise offset
/// into the layer.
pub(crate) type LayerIndex = (usize, usize, usize);

impl Layer {
    /// Create a [Layer] from some [Cell]s.
    pub fn from(iter: impl IntoIterator<Item = Cell>) -> Self {
        Layer(iter.into_iter().collect())
    }

    /// Return the total number of bits represented by a cell on this Layer.
    pub(crate) fn layer_bits(height: usize) -> usize {
        Cell::bits() << (4 * height)
    }

    /// Return the bitwise offset within the cell of the provided bit offset
    /// value. For instance, if you're looking for bit 10, you need to know
    /// which cell and which bit maps to global bit 10 for the higher layers.
    pub(crate) fn cell_bit(&self, li: LayerIndex) -> usize {
        let (height, _, bit) = li;
        (bit / (Self::layer_bits(height) >> 4)) % Cell::bits()
    }

    /// Get the value at some offset, as well as offset information used
    /// to do a [Layer::get] on the next layer of the [crate::Tree].
    pub(crate) fn get(&self, li: LayerIndex) -> (usize, bool) {
        let (height, offset, _) = li;

        let cell = self.0[offset];
        let o = self.cell_bit(li);

        let next_offset = if height > 0 {
            self.0[0..offset].count_ones() + cell.count_ones_until(o)
        } else {
            0
        };

        (next_offset, cell.get(o))
    }

    /// Insert a [Cell] into this layer at the provided index. This is done
    /// if you are adding a newly set bit, or growing the tree.
    pub(crate) fn insert_cell(&mut self, n: usize, cell: Cell) {
        self.0.insert(n, cell);
    }

    /// Set a bit. If the bit is already set the returned boolean value will
    /// be false. If this set a new value, this will return true.
    pub(crate) fn set(&mut self, li: LayerIndex) -> (usize, bool) {
        let (_, offset, _) = li;
        let (next_offset, set) = self.get(li);
        if set {
            return (next_offset, false);
        }
        let o = self.cell_bit(li);
        self.0[offset] = self.0[offset].set(o, true);
        (next_offset, true)
    }

    /// "Unset" a bit. This will not clean up any layers "above" our layer.
    pub(crate) fn unset(&mut self, li: LayerIndex) {
        let (_, offset, _) = li;
        let o = self.cell_bit(li);
        self.0[offset] = self.0[offset].set(o, false);
    }

    /// Turn this layer into a stream of self-describing `u16` values in a
    /// [Vec] which can be used to re-construct this [Layer].
    pub(crate) fn to_vec(&self) -> Vec<CellRepr> {
        self.0.iter().map(|v| (*v).into()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_basic() {
        let layer = Layer::from([1.into()]);
        let (_, v) = layer.get((0, 0, 0));
        assert!(v);
    }
}

// vim: foldmethod=marker
