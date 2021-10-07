#![cfg(feature = "heap")]
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum MatrixError {
    InvalidMulInput((usize, usize), (usize, usize)),
}

impl Display for MatrixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            &MatrixError::InvalidMulInput(expected, given) => {
                writeln!(
                    f,
                    "Invalid multiplication error expected size of matrix was {}x{} but was 
                    given matrix of size {}x{}.",
                    expected.0, expected.1, given.0, given.1
                )?;
            }
        }
        Ok(())
    }
}