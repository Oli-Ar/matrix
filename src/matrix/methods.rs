use super::{Data, Matrix};

impl<D: Data, const M: usize, const N: usize> Matrix<D, M, N>
where
    D: From<i32>,
    [D; M * N]: Sized,
{
    // Compiler will assets that passed in array is correct size so returing a result is not
    // required
    pub const fn new(data: [[D; N]; M]) -> Self {
        // This should be safe as nested arrays are guaranteed to have no padding or reordering so
        // transmuting to flatten it is safe
        // This is done so the new function is const and doesn't required a for loop to copy the
        // data into the flattened array
        let dat: &[D; M * N] = unsafe { std::mem::transmute::<&[[D; N]; M], &[D; M * N]>(&data) };
        Matrix { dat: *dat }
    }

    // To create column vectors without having to pass in [[1], [2], [3], ... , [n]]
    // Doesn't return a result as the function can't fail as long as the passed arguments can be
    // passed
    pub const fn vector(dat: [D; M * N]) -> Self {
        Matrix { dat }
    }
}
