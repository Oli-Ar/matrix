use super::Matrix;
use core::default::Default;
use core::fmt::{self, Debug, Display, Formatter};
use core::mem::transmute;
use core::ops::{Index, IndexMut};

impl<D: Copy, const M: usize, const N: usize> Matrix<D, M, N>
where
    [D; M * N]: Sized,
{
    // Compiler will assets that passed in array is correct size so returing a result is not
    // required
    pub const fn new(data: [[D; N]; M]) -> Self {
        // This should be safe as nested arrays are guaranteed to have no padding or reordering so
        // transmuting to flatten it is safe
        // This is done so the new function is const and doesn't required a for loop to copy the
        // data into the flattened array (O(1) instead of O(n))
        Matrix {
            buf: *unsafe { transmute::<&[[D; N]; M], &[D; M * N]>(&data) },
        }
    }

    // Returns the data stored in the matrix struct in 2D array form
    // Same safety applied as when creating an array from a 2d array as seen in the new method -
    // this should be fine to do (Again O(1) instead of O(N))
    pub const fn data(self) -> &'static [[D; N]; M] {
        unsafe { transmute::<&[D; M * N], &[[D; N]; M]>(self.buf()) }
    }

    // Returns the size of the matrix
    pub const fn size() -> (usize, usize) {
        (M, N)
    }
}

impl<D: Default + Copy, const M: usize, const N: usize> Default for Matrix<D, M, N>
where
    [D; M * N]: Sized,
{
    fn default() -> Matrix<D, M, N> {
        Matrix {
            buf: [Default::default(); M * N],
        }
    }
}

// Different impl block to make it so so the number of coluns can only be one
// Allow clippy indentity op here so clippy doesn't complain about how M * 1 can be reduced to M,
// it can't because the type of dat must be M * N and in this scenario N is always 1
#[allow(clippy::identity_op)]
impl<D, const M: usize> Matrix<D, M, 1>
where
    [D; M * 1]: Sized,
{
    // To create column vectors without having to pass in [[1], [2], [3], ... , [n]]
    // Doesn't return a result as the function can't fail as long as the passed arguments can be
    // passed
    pub const fn vector(buf: [D; M * 1]) -> Self {
        Matrix { buf }
    }
}

impl<D: Debug, const M: usize, const N: usize> Display for Matrix<D, M, N>
where
    [D; M * N]: Sized,
{
    // Prints a two dimensional representaion of the matrix
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 0..N {
            for col in 0..M {
                write!(f, "{:>2?},", self.buf()[M * row + col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// TODO: maybe handle some errors here, currently it panics as expected however the error message
// doesn't make sense because of the matrix being stored as a 1 dimensional array
impl<D, const M: usize, const N: usize> Index<[usize; 2]> for Matrix<D, M, N>
where
    [D; M * N]: Sized,
{
    type Output = D;
    fn index(&self, index: [usize; 2]) -> &D {
        // if index[0] >= M || index[1] >= N {
        //     // TODO: Write better error message
        //     panic!("Index out of bounds.");
        // }
        &self.buf()[index[0] * N + index[1]]
    }
}

impl<D, const M: usize, const N: usize> IndexMut<[usize; 2]> for Matrix<D, M, N>
where
    [D; M * N]: Sized,
{
    fn index_mut(&mut self, index: [usize; 2]) -> &mut D {
        // if index[0] >= M || index[1] >= N {
        //     // TODO: Write better error message
        //     panic!("Index out of bounds.");
        // }
        &mut self.buf_mut()[index[0] * N + index[1]]
    }
}
