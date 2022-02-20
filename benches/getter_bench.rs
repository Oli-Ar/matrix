#![feature(test)]
extern crate test;

use matrix::stack::Matrix;
use test::Bencher;

#[bench]
fn getter_const_ref_sml(b: &mut Bencher) {
    let matrix = Matrix::<i32, 10, 10>::default();
    b.iter(|| matrix.buf());
}

#[bench]
fn getter_const_ref_lrg(b: &mut Bencher) {
    let matrix = Matrix::<i32, 1000, 1000>::default();
    b.iter(|| matrix.buf());
}

#[bench]
fn getter_copy_sml(b: &mut Bencher) {
    let matrix = Matrix::<i32, 10, 10>::default();
    b.iter(|| matrix.buf_copy());
}

#[bench]
fn getter_copy_lrg(b: &mut Bencher) {
    let matrix = Matrix::<i32, 1000, 1000>::default();
    b.iter(|| matrix.buf_copy());
}
