use super::Matrix;
use core::ops::{Add, Mul, Sub};

impl<D, O: Copy, const M: usize, const N: usize> Add<Matrix<O, M, N>> for Matrix<D, M, N>
where
    [D; M * N]: Sized,
    D: Add<O, Output = D> + Copy,
{
    type Output = Self;
    fn add(self, other: Matrix<O, M, N>) -> Self {
        // Clone data then add second array to each value
        let mut buf = *self.buf();
        for (i, v) in buf.iter_mut().enumerate().take(M * N) {
            *v = *v + other.buf()[i];
        }
        Matrix { buf }
    }
}

impl<D, O: Copy, const M: usize, const N: usize> Sub<Matrix<O, M, N>> for Matrix<D, M, N>
where
    [D; M * N]: Sized,
    D: Sub<O, Output = D> + Copy,
{
    type Output = Self;
    fn sub(self, other: Matrix<O, M, N>) -> Self {
        // Clone data then add second array to each value
        let mut buf = *self.buf();
        for (i, v) in buf.iter_mut().enumerate().take(M * N) {
            *v = *v - other.buf()[i];
        }
        Matrix { buf }
    }
}

// Multiplication implementation for number * matrix
// TODO: Make this number * matrix not matrix * number
impl<D, O, const M: usize, const N: usize> Mul<O> for Matrix<D, M, N>
where
    [D; M * N]: Sized,
    D: Mul<O, Output = D> + Copy,
    O: Into<f64> + Mul<D, Output = D> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: O) -> Matrix<D, M, N> {
        let mut buf = *self.buf();
        for (i, e) in self.buf().iter().enumerate() {
            buf[i] = rhs * *e;
        }
        Matrix { buf }
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
    T: Mul<U, Output = T> + Add<Output = T> + Copy + Default,
    U: Copy,
{
    type Output = Matrix<T, A, C>;
    fn mul(self, rhs: Matrix<U, B, C>) -> Matrix<T, A, C> {
        let mut mat: Matrix<T, A, C> = Default::default();
        // This block works - don't know the different in i, j, and k; I pray I don't need to touch
        // this again.
        for i in 0..A {
            for j in 0..C {
                for k in 0..B {
                    mat[[i, j]] = mat[[i, j]] + self[[i, k]] * rhs[[k, j]];
                }
            }
        }
        mat
    }
}

impl<D, const M: usize, const N: usize> Matrix<D, M, N>
where
    D: Default + Copy,
    [D; M * N]: Sized,
    [D; N * M]: Sized,
{
    pub fn transpose(self) -> Matrix<D, N, M> {
        let mut mat: Matrix<D, N, M> = super::Matrix::default();
        for m in 0..M {
            for n in 0..N {
                mat[[n, m]] = self[[m, n]];
            }
        }
        mat
    }
}
