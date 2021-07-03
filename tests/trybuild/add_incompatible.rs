use matrix::matrix::Matrix;

const M: usize = 6;
const N: usize = 4;

const O: usize = 3;
const P: usize = 3;

fn main() {
    let matrix_1 = Matrix::<i32, M, N>::new([
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
        [17, 18, 19, 20],
        [21, 22, 23, 24],
    ]);
    let matrix_2 = Matrix::<i32, O, P>::new([[1, 2, 4], [4, 5, 6], [7, 8, 9]]);
    matrix_1 + matrix_2;
}
