pub trait Data = Into<f64> + Copy;

// The initial generic matrix structure
// All fields are private as the struct will have methods to display the matrix
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Matrix<D: Data, const M: usize, const N: usize>
where
    [D; M * N]: Sized,
{
    pub(crate) dat: [D; M * N],
}
