use matrix::stack::Matrix;

const M: usize = 3;
const N: usize = 3;

fn main() {
    let _ = Matrix::<i32, M, N>::new([[1, 2, 3], [4, 5, 6], [7]]);
}
