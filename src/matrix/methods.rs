use super::{Data, Matrix};

impl<D: Data, const N: usize> Matrix<D, { N }> {
    // Result return is not necessary as rust will require the matrix to be X by Y at compile time
    pub fn new<const X: usize, const Y: usize>(data: [[D; X]; Y]) -> Self {
        let mut dat: [D; N] = [0.into(); N];
        for i in 0..dat.len() {
            dat[i] = data[i / X][i % X];
        }
        Matrix { dat, x: X, y: Y }
    }

    // To create column vectors without having to pass in [[1], [2], [3], ... , [n]]
    // Doesn't return a result as the function can't fail as long as the passed arguments can be
    // passed
    pub fn vector(dat: [D; N]) -> Self {
        Matrix { dat, x: 1, y: N }
    }

    // TODO: Write method to create a random matrix of a given size
}
