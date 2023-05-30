#![deny(
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

pub mod bot;
pub mod global_variables;
pub mod helpers;

fn main() {
    crate::bot::start_bot();
}
