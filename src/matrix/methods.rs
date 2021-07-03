use super::{Data, Matrix};
use std::mem::transmute;

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
        let dat: &[D; M * N] = unsafe { transmute::<&[[D; N]; M], &[D; M * N]>(&data) };
        Matrix { dat: *dat }
    }

    // To create column vectors without having to pass in [[1], [2], [3], ... , [n]]
    // Doesn't return a result as the function can't fail as long as the passed arguments can be
    // passed
    pub const fn vector(dat: [D; M * N]) -> Self {
        Matrix { dat }
    }

    // For getting the raw data stored in the matrix struct
    pub const fn raw(self) -> [D; M * N] {
        self.dat
    }

    // Returns the data stored in the matrix struct in 2D array form
    // Same safety applied as when creating an array from a 2d array as seen in the new method -
    // this should be fine to do
    pub const fn data(self) -> [[D; N]; M] {
        *unsafe { transmute::<&[D; M * N], &[[D; N]; M]>(&self.dat) }
    }
}
