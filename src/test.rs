use super::*;

// Test that creating new matrices with correct inputs work
#[test]
fn input() {
    let matrix1 = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    assert!(matrix1.is_ok());

    let matrix2 = Matrix::new([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15]]);
    assert!(matrix2.is_ok());

    // Matrix with single row should be created here
    let matrix3 = Matrix::new([1, 2, 3, 4, 5, 6]);
    assert!(matrix3.is_ok());

    let matrix4 = Matrix::new([[1], [2], [3]]);
    assert!(matrix4.is_ok());

    // Matrix lib only supports two dimensional matrices
    let matrix5 = Matrix::new([[1, 2, 3], [4, 5, 6], [[7, 8], 9]]);
    assert_eq!(matrix5, Err(MtxErr::InvalidInput));

    // All rows of a matrix should be the same length
    let matrix6 = Matrix::new([[1, 2], [3, 4, 5, 6], [7, 8, 9]]);
    assert_eq!(matrix6, Err(MtxErr::InvalidInput));
}

#[test]
fn fromType() {
    let matrix1 = Matrix::try_from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    assert!(matrix1.is_ok());

    let matrix2 = Matrix::try_from([[1, 2, 3, 4], [5, 6, 7, 8, 9]]);
    assert_eq!(matrix2, Err(MtxErr::InvalidInput));

    // TODO: Write test for when converting from other types than just [[T; K]; N]
}

// TODO: Matrix is implemented write tests for indexing and adition
