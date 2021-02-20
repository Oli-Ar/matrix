use super::{Data, Matrix};
use std::fmt::{Display, Formatter};
use std::ops::Index;

impl<T: Data, const N: usize> Display for Matrix<T, { N }> {
    // Prints a two dimensional representaion of the matrix
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.y {
            for col in 0..self.x {
                write!(f, "{:>2?},", self.dat[self.x * row + col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Data, const N: usize> Index<[usize; 2]> for Matrix<T, { N }> {
    type Output = T;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        if index[0] >= self.x || index[1] >= self.y {
            // TODO: Write better error message
            panic!("Index out of bounds.");
        }
        &self.dat[index[1] * self.x + index[0]]
    }
}
