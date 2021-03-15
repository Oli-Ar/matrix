pub mod error;
pub mod methods;
pub mod traits;

pub trait Data = Into<f64> + std::fmt::Display + std::fmt::Debug + Copy;

// The initial generic matrix structure
// All fields are private as the struct will have methods to display the matrix
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Matrix<D: Data, const N: usize> {
    dat: [D; N],
    x: usize,
    y: usize,
}

// TODO: add more errors as more possible errors are encountered
#[derive(Debug, PartialEq)]
pub enum MatrixError {
    // For operations such as trying to add two different sized matrices
    InvalidOperation(String),
}
