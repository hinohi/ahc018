//! A procedural noise generation library for Rust.
//!
//! # Example
//!
//! ```rust
//! use noise::{NoiseFn, Perlin, Seedable};
//!
//! let perlin = Perlin::new(1);
//! let val = perlin.get([42.4, 37.7, 2.8]);
//! ```

extern crate alloc;

pub use crate::noise_fns::*;

pub mod core;
mod noise_fns;
pub mod permutationtable;
mod s_curve;
mod vectors;
