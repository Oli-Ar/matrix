use super::{Data, Matrix};

impl<D, const X: usize, const Y: usize> Matrix<D, X, Y>
where
    D: Data + From<i32>,
    [D; X * Y]: Sized,
{
    // Result return is not necessary as rust will require the matrix to be X by Y at compile time
    pub fn new(data: [[D; X]; Y]) -> Self {
        // Fill array with whatever is at index 0, 0 of the passed in matrix, the value doens't
        // matter as it is overwritten
        let mut dat: [D; X * Y] = [0.into(); X * Y];
        for i in 0..dat.len() {
            dat[i] = data[i / X][i % X];
        }
        Matrix { dat }
    }

    pub fn dimensions() -> (usize, usize) {
        (X, Y)
    }
}

// impl<D, const X: usize, const Y: usize> Matrix<D, X, Y>
// where
//     D: Data + From<i32>,
//     [D; X * Y]: Sized,
// {
//     // To create column vectors without having to pass in [[1], [2], [3], ... , [n]]
//     // Doesn't return a result as the function can't fail as long as the passed arguments can be
//     // passed
//     pub fn vector(dat: [D; X * Y]) -> Self {
//         Matrix { dat }
//     }
// }

impl<D, const X: usize, const Y: usize> Matrix<D, X, Y>
where
    D: Data + From<i32>,
    [D; X * Y]: Sized,
{
    // To create column vectors without having to pass in [[1], [2], [3], ... , [n]]
    // Doesn't return a result as the function can't fail as long as the passed arguments can be
    // passed
    pub fn vector(dat: [D; X * Y]) -> Self {
        Matrix { dat }
    }
}

// TODO: Write method to create a random matrix of a given size
