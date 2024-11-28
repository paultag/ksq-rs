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

use super::Tree;

/// A [Matrix] is the user-facing 2-dimensional bit vector built on a
/// [Tree]. The [Matrix] can store a fixed number of bits, which can be
/// accessed using [Tree::get], [Tree::set] or [Tree::unset].
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix(Tree);

impl Matrix {
    /// Return a new [Matrix] with a new [Tree].
    pub fn new() -> Self {
        Matrix(Tree::new())
    }

    /// Create a matrix from a [Tree].
    pub fn from(tree: Tree) -> Self {
        Self(tree)
    }

    /// Return the inner tree.
    pub fn into_inner(self) -> Tree {
        self.0
    }

    /// Return the total number of bits addressable by the Matrix.
    pub fn bits(&self) -> usize {
        self.0.bits()
    }

    /// Return the number of rows or columns in the Matrix.
    pub fn side(&self) -> usize {
        1 << ((4 * (self.0.height())) / 2)
    }

    /// return the offset into the 1d tree.
    fn offset(&self, x: usize, y: usize) -> usize {
        (self.side() * y) + x
    }

    /// return the value of the bit at (x, y)
    pub fn get(&mut self, x: usize, y: usize) -> bool {
        self.0.get(self.offset(x, y))
    }

    /// set the value of the bit at (x, y)
    pub fn set(&mut self, x: usize, y: usize) {
        self.0.set(self.offset(x, y));
    }

    /// unset the value of the bit at (x, y)
    pub fn unset(&mut self, x: usize, y: usize) {
        self.0.unset(self.offset(x, y));
    }

    /// Grow the underlying [Tree] by a layer, increasing capacity by the Cell
    /// bit factor (currently 16, this may change).
    pub fn grow(&mut self) {
        self.0.grow();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_dimensions() {
        let mut mat = Matrix::new();
        assert_eq!(16, mat.bits());
        assert_eq!(4, mat.side());

        mat.grow();
        assert_eq!(256, mat.bits());
        assert_eq!(16, mat.side());

        mat.grow();
        assert_eq!(4096, mat.bits());
        assert_eq!(64, mat.side());
    }

    #[test]
    fn matrix_get_set_xy() {
        let mut mat = Matrix::new();
        mat.grow();
        mat.grow();
        mat.grow();

        let side = mat.side();

        for y in 0..side {
            for x in 0..side {
                assert!(!mat.get(x, y));
                mat.set(x, y);
                assert!(mat.get(x, y));
                mat.unset(x, y);
                assert!(!mat.get(x, y));
            }
        }
    }

    #[test]
    fn matrix_get_set_xy_dirty() {
        let mut mat = Matrix::new();
        mat.grow();
        mat.grow();
        mat.grow();

        let side = mat.side();

        for y in 0..side {
            for x in 0..side {
                assert!(!mat.get(x, y));
                mat.set(x, y);
                assert!(mat.get(x, y));
            }
        }
    }
}

// vim: foldmethod=marker
