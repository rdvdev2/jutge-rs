//! A library crate for easy and idiomatic interaction with
//! <https://jutge.org>

#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod client;
mod error;
mod problem;

pub use client::*;
pub use error::*;
pub use problem::*;
