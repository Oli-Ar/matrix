use std::fmt::{Display, Formatter};

pub trait Data = std::convert::From<i32> + Display + std::fmt::Debug + Copy;

// The initial generic matrix structure
// All fields are private as the struct will have methods to display the matrix
#[derive(Debug, Eq, PartialEq)]
pub struct Matrix<D: Data, const N: usize> {
    dat: [D; N],
    x: usize,
    y: usize,
}

impl<D: Data, const N: usize> Matrix<D, { N }> {
    // TODO: error should be an enum not an int
    pub fn new<const X: usize, const Y: usize>(data: [[D; X]; Y]) -> Result<Self, i32> {
        for sub in data.iter() {
            if sub.len() != X {
                return Err(0);
            }
        }
        let mut dat: [D; N] = [0.into(); N];
        for i in 0..dat.len() {
            dat[i] = data[i / Y][i % X];
        }
        Ok(Matrix { dat, x: X, y: Y })
    }

    // To create column vectors without having to pass in [[1], [2], [3], ... , [n]]
    // Doesn't return a result as the function can't fail as long as the passed arguments can be
    // passed
    pub fn vector(dat: [D; N]) -> Self {
        Matrix { dat, x: 1, y: N }
    }
}

impl<T: Data, const N: usize> Display for Matrix<T, { N }> {
    // Prints a two dimensional representaion of the vector
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.y {
            for col in 0..self.x {
                write!(f, "{:>2?},", self.dat[self.x * row + col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
