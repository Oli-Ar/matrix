// use matrix::matrix::Matrix;
//
// const X: usize = 5;
// const Y: usize = 3;
// const LENGTH: usize = X * Y;
//
// #[test]
// fn index() {
//     let matrix =
//         Matrix::<i32, LENGTH>::new([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15]]);
//     assert_eq!(matrix[[0, 2]], 11);
//     assert_eq!(matrix[[0, 0]], 1);
//     assert_eq!(matrix[[4, 1]], 10);
// }
//
// #[test]
// #[should_panic(expected = "Index out of bounds")]
// fn index_x_out_of_bounds() {
//     let matrix =
//         Matrix::<i32, LENGTH>::new([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15]]);
//     matrix[[5, 2]];
// }
//
// #[test]
// #[should_panic(expected = "Index out of bounds")]
// fn index_y_out_of_bounds() {
//     let matrix =
//         Matrix::<i32, LENGTH>::new([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15]]);
//     matrix[[3, 4]];
// }
