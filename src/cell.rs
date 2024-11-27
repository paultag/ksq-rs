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

use super::CountOnes;

pub(crate) type CellRepr = u16;

/// Cell is the lowest level bit field. This is (as an implementation detail)
/// an integer being used as a bit array. It's currently a u16, but any
/// assumptions on bit size must use `Cell::bits` instead, as it may
/// change as performance is tweaked (or maybe at runtime!)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Cell(CellRepr);

macro_rules! bounds_check {
    ($n:expr) => {
        assert!($n < $crate::Cell::bits(), "out of bounds");
    };
}

impl CountOnes for Cell {
    fn count_ones(&self) -> usize {
        self.0.count_ones() as usize
    }

    fn count_ones_until(&self, idx: usize) -> usize {
        bounds_check!(idx);
        (self.0 & ((1 << idx) - 1)).count_ones() as usize
    }
}

impl From<CellRepr> for Cell {
    fn from(n: CellRepr) -> Self {
        Self(n)
    }
}

impl From<Cell> for CellRepr {
    fn from(n: Cell) -> Self {
        n.0
    }
}

impl Cell {
    /// Return a new [Cell].
    pub fn new() -> Self {
        Self(0)
    }

    /// Return the number of bits stored in a [Cell].
    pub const fn bits() -> usize {
        16
    }

    /// Return the status of a bit index inside the cell. If the bit is out
    /// of range, this will induce a panic.
    pub fn get(&self, n: usize) -> bool {
        bounds_check!(n);

        if self.0 == 0 {
            return false;
        }

        (self.0 & (1 << n)) != 0
    }

    /// Set a bit index, returning the new Cell. If the bit is out of range,
    /// this will induce a panic.
    pub fn set(&self, n: usize, v: bool) -> Self {
        bounds_check!(n);

        if v {
            Self(self.0 | (1 << n))
        } else {
            Self(self.0 & (!(1 << n)))
        }
    }

    /// Return the inner type. This may change over time.
    pub fn inner(&self) -> CellRepr {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_get_set() {
        let mut c = Cell::new().set(1, true).set(3, true);
        assert_eq!(10, c.inner());

        assert_eq!(2, c.count_ones());
        assert_eq!(2, c.count_ones_until(4));
        assert_eq!(1, c.count_ones_until(2));

        assert!(!c.get(0));
        assert!(c.get(1));
        assert!(!c.get(2));
        assert!(c.get(3));
        assert!(!c.get(4));

        c = c.set(1, false).set(3, false);
        assert_eq!(0, c.inner());
    }

    #[test]
    fn cell_get_set_all() {
        for i in 0..16 {
            let mut c = Cell::new();
            assert!(!c.get(i));
            c = c.set(i, true);
            assert!(c.get(i));
            c = c.set(i, false);
            assert!(!c.get(i));
        }
    }

    #[test]
    fn cell_get_set_all_dirty() {
        let mut c = Cell::new();
        for i in 0..16 {
            assert!(!c.get(i));
            c = c.set(i, true);
            assert!(c.get(i));
        }
    }
}

// vim: foldmethod=marker
