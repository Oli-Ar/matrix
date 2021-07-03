// NOTE: For some reason the kernel panics if the return type is changed from the explicit type
// to 'Self::Output' DO NOT CHANGE THE RETURN TYPES output has to be set to satisfy the traits
use super::{Data, Matrix};
use std::ops::{Add, Index, Mul};
use std::{
    fmt::{Display, Formatter},
    usize,
};

impl<T, const X: usize, const Y: usize> Display for Matrix<T, X, Y>
where
    [T; X * Y]: Sized,
    T: Data + std::fmt::Debug,
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

impl<T: Data, const M: usize, const N: usize> Index<[usize; 2]> for Matrix<T, M, N>
where
    [T; M * N]: Sized,
{
    type Output = T;
    fn index(&self, index: [usize; 2]) -> &T {
        if index[0] >= M || index[1] >= N {
            // TODO: Write better error message
            panic!("Index out of bounds.");
        }
        &self.dat[index[0] * N + index[1]]
    }
}

impl<T, U, const X: usize, const Y: usize> Add<Matrix<U, X, Y>> for Matrix<T, X, Y>
where
    [T; X * Y]: Sized,
    T: Data + Add<U, Output = T>,
    U: Data,
{
    type Output = Self;
    fn add(self, other: Matrix<U, X, Y>) -> Self {
        // Clone data then add second array to each value
        let mut dat: [T; X * Y] = self.dat;
        for i in 0..X * Y {
            dat[i] = dat[i] + other.dat[i];
        }
        Matrix { dat }
    }
}

// Multiplication implementation for number * matrix
// TODO: Make this more generic eg allow multiplication of other types
impl<T, const X: usize, const Y: usize> Mul<T> for Matrix<T, X, Y>
where
    [T; X * Y]: Sized,
    T: Data + Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Matrix<T, X, Y> {
        let mut dat_copy = self.dat;
        for (i, e) in self.dat.iter().enumerate() {
            dat_copy[i] = rhs * *e;
        }
        Matrix { dat: dat_copy }
    }
}

// Multiplication between two matrices
// 2D Matrix multiplication requires for the Y dimension of the first and the X dimention of second
// to be the same to be valid this will result in matrix of size Y of the first and X of the
// second. Hence, matrix of size A by B multiplied by matix with dimensions B by C will always be
// valid and have a product of a matrix with dimensions A by C, meaining this condition can be
// verified purely in the decleration of this implementation.
impl<T, U, const A: usize, const B: usize, const C: usize> Mul<Matrix<U, B, C>> for Matrix<T, A, B>
where
    [T; A * B]: Sized, // Matrix one
    [U; B * C]: Sized, // Matrix two
    [T; A * C]: Sized, // Resulting matrix
    T: Data + Mul<U, Output = T> + Add<Output = T>,
    U: Data,
    u8: Into<T>, // u8 must implement into T so that an array can be constructed - this might be
                 // bad idk, TODO: Maybe fix that ^
{
    type Output = Matrix<T, A, C>;
    fn mul(self, rhs: Matrix<U, B, C>) -> Matrix<T, A, C> {
        let mut dat: [T; A * C] = [0.into(); A * C];
        // This block works - don't know the different in i, j, and k; I pray I don't need to touch
        // this again.
        for i in 0..A {
            for j in 0..C {
                for k in 0..B {
                    dat[C * i + j] = dat[C * i + j] + self.dat[B * i + k] * rhs.dat[A * k + j];
                }
            }
        }
        Matrix { dat }
    }
}
