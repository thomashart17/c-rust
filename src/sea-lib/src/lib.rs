#![no_std]

pub extern crate sea_rs_common;

// #[cfg(not(feature = "no_std"))]
// #![no_std]
// #![cfg_attr(not(feature = "std"), no_std)]
// #[cfg(not(feature = "std"))]
// extern crate sea_rs_common;
#[cfg(not(feature = "std"))]
pub use sea_rs_common::CAllocator;

// extern crate bindings; 
// use bindings;
pub mod bindings;
pub mod seahorn;
pub use seahorn::*;
