# ksq - k-2 tree library for rust

`ksq` is an implementation of a K2 tree (k²-tree), which, when storing sparse
bits, is a very space-effective matrix. This library implements the tree as a
flat 1-d array, rather than explicitly encoding dimensionality.

Unlike some other k2 trees, I've opted to use a `u16`, not a `u8`. This means
that the tree will grow by `N<<4` each layer -- and each cell can represent a
maximum of 16 other cells, not 8. This may change in the future.
