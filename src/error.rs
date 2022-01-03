#![cfg(feature = "heap")]
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    InvalidOrderMatrix((usize, usize), (usize, usize)),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            &Error::InvalidOrderMatrix(expected, given) => {
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
