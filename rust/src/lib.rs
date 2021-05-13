#![feature(const_fn_fn_ptr_basics)]
#![feature(generators, generator_trait)]
// #![feature(in_band_lifetimes)]
// #![feature(nll)]
#![feature(stmt_expr_attributes)]
// #![feature(tool_lints)]
// #![feature(try_from)]
#![warn(rust_2018_idioms)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::similar_names)]
#![allow(clippy::double_parens)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::comparison_chain)]

pub use failure::Error;

pub use crate::common::{
    Answer, Answers, Date, Day, Puzzle, Puzzles, Result, Solver, Year, YearPuzzles,
};
pub(crate) use crate::utils::OkOrFail;

pub const PUZZLES: Puzzles = Puzzles::new(PUZZLES_ARRAY);

#[rustfmt::skip]
const PUZZLES_ARRAY: &[YearPuzzles] = &[
    y2015::PUZZLES,
    y2016::PUZZLES,
];

#[macro_use]
mod macros;
#[macro_use]
mod parse;
mod common;
mod utils;
mod y2015;
mod y2016;
