use matrix::matrix::Matrix;

const X: usize = 4;
const Y: usize = 6;
const LENGTH: usize = X * Y;

const X2: usize = 3;
const Y2: usize = 3;
const LENGTH2: usize = 9;

fn main() {
    let matrix_1 = Matrix::<i32, LENGTH>::new([
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
        [17, 18, 19, 20],
        [21, 22, 23, 24],
    ]);
    let matrix_2 = Matrix::<i32, LENGTH2>::new([[1, 2, 4], [4, 5, 6], [7, 8, 9]]);
    matrix_1 + matrix_2;
}
