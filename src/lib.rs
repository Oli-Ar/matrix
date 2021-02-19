#![allow(clippy::pedantic)]
#![feature(trait_alias)]
pub trait Data = std::convert::From<i32> + std::fmt::Display + std::fmt::Debug + Copy;

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
}

#[cfg(test)]
mod test;
