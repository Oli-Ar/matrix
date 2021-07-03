pub(crate) mod error;
pub(crate) mod methods;
pub(crate) mod traits;

pub trait Data = Into<f64> + Copy;

// The initial generic matrix structure
// All fields are private as the struct will have methods to display the matrix
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Matrix<D: Data, const M: usize, const N: usize>
where
    [D; M * N]: Sized,
{
    dat: [D; M * N],
}

// TODO: add more errors as more possible errors are encountered
#[derive(Debug, PartialEq)]
pub enum MatrixError {
    // For operations such as trying to add two different sized matrices
    InvalidOperation(String),
}
