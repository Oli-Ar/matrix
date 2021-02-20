use matrix::matrix::Matrix;

const X: usize = 5;
const Y: usize = 2;
const LENGTH: usize = X * Y;

// We can verify that make works therefore we can verify vector works as it is a function for
// convenience which should be equal to a matrix where X == 1
#[test]
fn test() {
    let cv1 = Matrix::<i32, LENGTH>::new([[1], [2], [3], [4], [5], [6], [7], [8], [9], [10]]);
    let cv2 = Matrix::<i32, LENGTH>::vector([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(cv1, cv2);
}
