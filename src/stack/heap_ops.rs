#![cfg(feature = "heap")]

use super::Matrix;
use core::ops::{Add, Mul};

use crate::error::Error;
use crate::heap::HeapMatrix;

// Matrix multiplication between a stack and heap matrix, this will result in a heap matrix
// as the size of a heap matrix cannot be asserted at compile time therefore the order of the
// resulting matrix cannot be known at compile time so a heap matrix must be returned
impl<T, U, const A: usize, const B: usize> Mul<HeapMatrix<U>> for Matrix<T, A, B>
where
    [T; A * B]: Sized,
    T: Mul<U, Output = T> + Add<Output = T> + Copy + Default,
    U: Copy,
{
    type Output = Result<HeapMatrix<T>, Error>;
    fn mul(self, rhs: HeapMatrix<U>) -> Result<HeapMatrix<T>, Error> {
        if B != rhs.rows {
            return Err(Error::InvalidOrderMatrix((A, B), (rhs.rows, rhs.cols)));
        }
        let mut mat = HeapMatrix::<T> {
            rows: A,
            cols: rhs.cols,
            ..Default::default()
        };
        for i in 0..A {
            for j in 0..rhs.cols {
                for k in 0..B {
                    mat[[i, j]] = mat[[i, j]] + self[[i, k]] * rhs[[k, j]];
                }
            }
        }
        Ok(mat)
    }
}
