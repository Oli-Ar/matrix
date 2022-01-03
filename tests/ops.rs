use matrix::stack::Matrix;

#[test]
fn add() {
    const M: usize = 6;
    const N: usize = 4;

    let matrix_1 = Matrix::<i32, M, N>::new([
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
        [17, 18, 19, 20],
        [21, 22, 23, 24],
    ]);
    assert_eq!(matrix_1 + matrix_1, matrix_1 * 2);

    const O: usize = 3;

    let matrix_2 = Matrix::<i32, O, O>::new([[5, 8, 3], [2, 1, 9], [4, 6, 7]]);
    let matrix_3 = Matrix::<i32, O, O>::new([[7, 5, 7], [4, 2, 8], [4, 5, 2]]);
    let test_matrix_2 = Matrix::<i32, O, O>::new([[12, 13, 10], [6, 3, 17], [8, 11, 9]]);
    assert_eq!(matrix_2 + matrix_3, test_matrix_2);
}

#[test]
fn subtract() {
    const M: usize = 5;
    const N: usize = 2;

    let matrix_1 = Matrix::<i32, M, N>::new([[0, 1], [2, 3], [4, 5], [6, 7], [8, 9]]);
    let matrix_2 = Matrix::<i32, M, N>::new([[5, 6], [1, 6], [7, 23], [32, 4], [2, 4]]);

    let matrix_3 = Matrix::<i32, M, N>::new([[-5, -5], [1, -3], [-3, -18], [-26, 3], [6, 5]]);
    assert_eq!(matrix_1 - matrix_2, matrix_3);
}

#[test]
fn scalar_multiplication() {
    const M: usize = 3;
    const N: usize = 3;

    let matrix_1 = Matrix::<i32, M, N>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let test_matrix_1 = Matrix::<i32, M, N>::new([[2, 4, 6], [8, 10, 12], [14, 16, 18]]);
    assert_eq!(matrix_1 * 2, test_matrix_1);

    const O: usize = 4;
    const P: usize = 2;

    let matrix_2 = Matrix::<i32, O, P>::new([[4, 6], [7, 13], [12, 63], [3, 62]]);
    let test_matrix_2 = Matrix::<i32, O, P>::new([[12, 18], [21, 39], [36, 189], [9, 186]]);
    assert_eq!(matrix_2 * 3, test_matrix_2);
}

#[test]
fn matrix_multiplication() {
    const M: usize = 2;
    const N: usize = 3;

    let matrix_1 = Matrix::<i32, M, N>::new([[1, 2, 3], [4, 5, 6]]);
    let matrix_2 = Matrix::<i32, N, M>::new([[7, 8], [9, 10], [11, 12]]);
    let test_matrix = Matrix::<i32, M, M>::new([[58, 64], [139, 154]]);
    assert_eq!(matrix_1 * matrix_2, test_matrix);

    let matrix_3 = Matrix::<i32, 3, 3>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let unit_matrix = Matrix::<i32, 3, 3>::new([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    assert_eq!(matrix_3 * unit_matrix, matrix_3);
}

#[test]
fn index() {
    const M: usize = 3;
    const N: usize = 5;

    let matrix =
        Matrix::<i32, M, N>::new([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15]]);
    assert_eq!(matrix[[2, 0]], 11);
    assert_eq!(matrix[[0, 0]], 1);
    assert_eq!(matrix[[1, 4]], 10);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 15 but the index is 15")]
fn index_x_out_of_bounds() {
    const M: usize = 3;
    const N: usize = 5;

    let matrix =
        Matrix::<i32, M, N>::new([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15]]);
    matrix[[2, 5]];
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 15 but the index is 23")]
fn index_y_out_of_bounds() {
    const M: usize = 3;
    const N: usize = 5;

    let matrix =
        Matrix::<i32, M, N>::new([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15]]);
    matrix[[4, 3]];
}
