use matrix::matrix;

fn main() {
    let matrix = matrix!([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
    let output = format!("{}", matrix);
    let expected = "[  1,  2,  3,  4,\n   \
                       5,  6,  7,  8,\n   \
                       9, 10, 11, 12,]";
    assert_eq!(output, expected);
}
