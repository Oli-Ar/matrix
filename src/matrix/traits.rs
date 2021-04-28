use super::{Data, Matrix};
use std::fmt::{Display, Formatter};
use std::ops::{Index, Mul};

impl<T: Data, const X: usize, const Y: usize> Display for Matrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    // Prints a two dimensional representaion of the matrix
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..Y {
            for col in 0..X {
                write!(f, "{:>2?},", self.dat[X * row + col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Data, const X: usize, const Y: usize> Index<[usize; 2]> for Matrix<T, X, Y>
where
    [T; X * Y]: Sized,
{
    type Output = T;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        if index[0] >= X || index[1] >= Y {
            // TODO: Write better error message
            panic!("Index out of bounds.");
        }
        &self.dat[index[1] * X + index[0]]
    }
}

// Multiplication implementation for number * matrix
impl<T, const X: usize, const Y: usize> Mul<T> for Matrix<T, X, Y>
where
    T: Data + Mul<Output = T> + From<i32>,
    [T; X * Y]: Sized,
{
    // This does nothing but I can't delete it - opened issue with return type on rust github
    // https://github.com/rust-lang/rust/issues/84631
    type Output = Self;
    fn mul(self, rhs: T) -> Matrix<T, X, Y> {
        let mut dat_copy = [0.into(); X * Y];
        for (i, e) in self.dat.iter().enumerate() {
            dat_copy[i] = rhs * *e;
        }
        Matrix { dat: dat_copy }
    }
}
