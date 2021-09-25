use matrix::StackMatrix;

#[test]
fn new() {
    let matrix_1 = StackMatrix::<i32, 4, 2>::new([[1, 2], [3, 4], [5, 6], [7, 8]]);
    assert_eq!(matrix_1.data(), [[1, 2], [3, 4], [5, 6], [7, 8]]);
    assert_eq!(matrix_1.raw(), [1, 2, 3, 4, 5, 6, 7, 8]);
}

// We can verify that new works therefore we can verify vector works as it is a function for
// convenience which should be equal to a matrix which has 1 column
#[test]
fn column_vec() {
    let cv1 = StackMatrix::<i32, 10, 1>::new([[1], [2], [3], [4], [5], [6], [7], [8], [9], [10]]);
    let cv2 = StackMatrix::<i32, 10, 1>::vector([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(cv1, cv2);
}

#[test]
fn default_vec() {
    let matrix_1 = StackMatrix::<i32, 3, 4>::default();
    assert_eq!(matrix_1.raw(), [0; 12]);
}
