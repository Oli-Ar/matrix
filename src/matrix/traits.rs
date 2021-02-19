use super::*;
use std::fmt::{Display, Formatter};

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
