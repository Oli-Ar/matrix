// NOTE: For some reason the kernel panics if the return type from traits is changed from
// the explicit type to 'Self::Output' DO NOT CHANGE THE RETURN TYPES output has to be explicit
// to stop a kernel panic

#![no_std]
#![allow(clippy::pedantic, incomplete_features)]
#![feature(trait_alias)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(const_fn_trait_bound)]
#![feature(const_refs_to_cell)]

pub mod arithmetic;
pub mod matrix;
pub mod utils;

pub use crate::matrix::{Data, Matrix};
