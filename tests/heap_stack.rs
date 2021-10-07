use matrix::*;

#[test]
fn cross_mul() {
    const M: usize = 3;
    let stack_matrix = StackMatrix::<i32, M, M>::new([[5, 35, 62], [14, 54, 34], [23, 52, 8]]);
    let heap_matrix = HeapMatrix::<i32>::new(&[&[54, 62, 72], &[43, 5, 32], &[235, 24, 2]]);
    let heap_result = HeapMatrix::<i32>::new(&[
        &[16345, 1973, 1604],
        &[11068, 1954, 2804],
        &[5358, 1878, 3336],
    ]);
    assert_eq!((stack_matrix * heap_matrix).unwrap(), heap_result);
}
