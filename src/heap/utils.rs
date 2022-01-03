use super::HeapMatrix;
use std::fmt::{self, Debug, Display, Formatter};
use std::vec::Vec;

impl<D: Clone> HeapMatrix<D> {
    // TODO: This function is shit, rewrite it later
    pub fn new(data: &[&[D]]) -> Self {
        let cols = data[0].len();
        if data.iter().any(|d| d.len() != cols) {
            panic!("All rows of matrix must have same amount of collumns")
        }
        HeapMatrix {
            buf: data
                .iter()
                .fold(Vec::with_capacity(data.len() * cols), |mut a, n| {
                    a.append(&mut n.to_vec());
                    a
                }),
            cols,
            rows: data.len(),
        }
    }

    pub fn raw(self) -> Vec<D> {
        self.buf
    }

    // These lifetimes work, don't touch
    pub fn data<'a>(&'a self) -> Vec<&'a [D]> {
        self.buf.chunks(self.cols).collect::<Vec<&'a [D]>>()
    }

    pub fn size(self) -> (usize, usize) {
        (self.cols, self.rows)
    }
}

impl<T: Clone + Default> Default for HeapMatrix<T> {
    fn default() -> Self {
        let mut tmp = HeapMatrix {
            buf: Vec::new(),
            rows: 3,
            cols: 3,
        };
        tmp.buf = std::vec![Default::default(); tmp.rows * tmp.cols];
        tmp
    }
}

impl<T: Clone> std::ops::Index<[usize; 2]> for HeapMatrix<T> {
    type Output = T;
    fn index(&self, index: [usize; 2]) -> &T {
        &self.buf[self.cols * index[0] + index[1]]
    }
}

impl<T: Clone> std::ops::IndexMut<[usize; 2]> for HeapMatrix<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut T {
        &mut self.buf[self.cols * index[0] + index[1]]
    }
}

impl<T: Debug> Display for HeapMatrix<T> {
    // Prints a two dimensional representaion of the matrix
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 0..self.cols {
            for col in 0..self.rows {
                write!(f, "{:>2?},", self.buf[self.cols * row + col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
