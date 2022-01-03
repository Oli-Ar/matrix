// TODO: This code's all kinda shit, this all needs to be fixed, make proper getters and setters as
// rename like the data method
use std::vec::Vec;

// Heap allocated generic matrix structure
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct HeapMatrix<D> {
    pub(crate) buf: Vec<D>,
    pub(crate) cols: usize,
    pub(crate) rows: usize,
}
