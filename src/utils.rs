use crate::{Data, Matrix};
use core::fmt::{self, Debug, Display, Formatter};
use core::mem::transmute;
use core::ops::Index;

impl<D, const M: usize, const N: usize> Matrix<D, M, N>
where
    D: Data,
    [D; M * N]: Sized,
{
    // Compiler will assets that passed in array is correct size so returing a result is not
    // required
    #[inline(always)]
    pub const fn new(data: [[D; N]; M]) -> Self {
        // This should be safe as nested arrays are guaranteed to have no padding or reordering so
        // transmuting to flatten it is safe
        // This is done so the new function is const and doesn't required a for loop to copy the
        // data into the flattened array
        Matrix {
            dat: *unsafe { transmute::<&[[D; N]; M], &[D; M * N]>(&data) },
        }
    }

    // For getting the raw data stored in the matrix struct
    #[inline(always)]
    pub const fn raw(self) -> [D; M * N] {
        self.dat
    }

    // Returns the data stored in the matrix struct in 2D array form
    // Same safety applied as when creating an array from a 2d array as seen in the new method -
    // this should be fine to do
    #[inline(always)]
    pub const fn data(self) -> [[D; N]; M] {
        *unsafe { transmute::<&[D; M * N], &[[D; N]; M]>(&self.dat) }
    }
}

// Different impl block to make it so so the number of coluns can only be one
// Allow clippy indentity op here so clippy doesn't complain about how M * 1 can be reduced to M,
// it can't because the type of dat must be M * N and in this scenario N is always 1
#[allow(clippy::identity_op)]
impl<D, const M: usize> Matrix<D, M, 1>
where
    D: Data,
    [D; M * 1]: Sized,
{
    // To create column vectors without having to pass in [[1], [2], [3], ... , [n]]
    // Doesn't return a result as the function can't fail as long as the passed arguments can be
    // passed
    #[inline(always)]
    pub const fn vector(dat: [D; M * 1]) -> Self {
        Matrix { dat }
    }
}

impl<T, const X: usize, const Y: usize> Display for Matrix<T, X, Y>
where
    [T; X * Y]: Sized,
    T: Data + Debug,
{
    // Prints a two dimensional representaion of the matrix
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
