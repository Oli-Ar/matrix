use super::HeapMatrix;
use alloc::vec::Vec;

impl<D: Clone> HeapMatrix<D> {
    // TODO: This function is shit, rewrite it later
    pub fn new(data: &[&[D]]) -> Self {
        let cols = data[0].len();
        if data.iter().any(|d| d.len() != cols) {
            panic!("All rows of matrix must have same amount of collumns")
        }
        HeapMatrix {
            dat: data
                .iter()
                .fold(Vec::with_capacity(data.len() * cols), |mut a, n| {
                    a.append(&mut n.to_vec());
                    a
                }),
            cols,
            rows: data.len(),
        }
    }

    #[inline(always)]
    pub fn raw(self) -> Vec<D> {
        self.dat
    }

    // These lifetimes work, don't touch
    #[inline(always)]
    pub fn data<'a>(&'a self) -> Vec<&'a [D]> {
        self.dat.chunks(self.cols).collect::<Vec<&'a [D]>>()
    }

    #[inline(always)]
    pub fn size(self) -> (usize, usize) {
        (self.cols, self.rows)
    }
}
