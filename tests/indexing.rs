// Check that indexing works
use matrix::matrix;

fn main() {
    let x = matrix!([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    assert_eq!(x[[2, 1]], 6);
    assert_eq!(x[[0, 0]], 1);

    let y = matrix!([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
    assert_eq!(y[[3, 0]], 4);
    assert_eq!(y[[3, 2]], 12);
}
