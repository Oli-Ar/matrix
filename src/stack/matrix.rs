// The initial generic matrix structure
// All fields are private as the struct will have methods to display the matrix
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct StackMatrix<D, const M: usize, const N: usize>
where
    [D; M * N]: Sized,
{
    pub(crate) buf: [D; M * N],
}
