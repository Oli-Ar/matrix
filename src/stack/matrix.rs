use super::super::matrix::Matrix as MatrixTrait;
// The initial generic matrix structure
// All fields are private as the struct will have methods to display the matrix
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Matrix<D, const M: usize, const N: usize>
where
    [(); M * N]:,
{
    pub(super) buf: [D; M * N],
}

impl<D, const M: usize, const N: usize> MatrixTrait for Matrix<D, M, N> where [(); M * N]: {}

impl<D: Copy, const M: usize, const N: usize> Matrix<D, M, N>
where
    [(); M * N]:,
{
    // Getter method
    pub const fn buf(&self) -> &[D; M * N] {
        &self.buf
    }

    pub const fn buf_copy(&self) -> [D; M * N] {
        self.buf
    }

    // Returns a mutable reference to buffer so it can be edited, this function is only visible to
    // the module the matrix structure is declared in and sub modules, as it is not intended for
    // the user to edit the raw matrix buffer
    pub(super) fn buf_mut(&mut self) -> &mut [D; M * N] {
        &mut self.buf
    }
}
