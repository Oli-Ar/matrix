use super::*;

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
