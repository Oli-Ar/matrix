use matrix::matrix::Matrix;

const LENGTH: usize = 9;

fn main() {
    let _ = Matrix::<i32, LENGTH>::new([[1, 2, 3], [4, 5, 6], [7]]);
}
