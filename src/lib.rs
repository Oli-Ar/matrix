// NOTE: For some reason the kernel panics if the return type from traits is changed from
// the explicit type to 'Self::Output' DO NOT CHANGE THE RETURN TYPES output has to be explicit
// to stop a kernel panic

#![no_std]
#![allow(clippy::pedantic, incomplete_features)]
#![feature(
    trait_alias,
    generic_const_exprs,
    const_fn_trait_bound,
    const_refs_to_cell
)]

#[cfg(feature = "heap")]
extern crate std;
#[cfg(feature = "heap")]
mod heap;
#[cfg(feature = "heap")]
pub use heap::HeapMatrix;
#[cfg(feature = "heap")]
mod error;
#[cfg(feature = "heap")]
pub use error::MatrixError;

mod stack;
pub use stack::StackMatrix;
