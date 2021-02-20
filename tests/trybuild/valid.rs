use matrix::matrix::Matrix;

const X1: usize = 3;
const Y1: usize = 3;
const LENGTH1: usize = X1 * Y1;

const X2: usize = 5;
const Y2: usize = 3;
const LENGTH2: usize = X2 * Y2;

const X3: usize = 6;
const Y3: usize = 1;
const LENGTH3: usize = X3 * Y3;

fn main() {
    let _ = Matrix::<i32, LENGTH1>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let _ = Matrix::<i32, LENGTH2>::new([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15]]);
    let _ = Matrix::<i32, LENGTH3>::new([[1, 2, 3, 4, 5, 6]]);
}
