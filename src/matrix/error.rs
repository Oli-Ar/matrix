use super::MatrixError;
use std::fmt::{Display, Formatter};

// This pattern is exhaustive so any new errors will have to be added here for this not to error
impl Display for MatrixError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            MatrixError::IncompatibleOperation(op) => {
                writeln!(f, "Incompatible operation: {}", op)?
            }
        };
        Ok(())
    }
}

// use std::error::Error;

// impl Error for MatrixError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         // NOTE: Implement later if necessary
//     }
// }
