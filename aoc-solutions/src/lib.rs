#![feature(
    bool_to_option,
    once_cell,
    half_open_range_patterns,
    exclusive_range_pattern,
    vec_retain_mut
)]

pub mod year2020;
pub mod year2021;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
