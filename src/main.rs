#![feature(rustc_attrs)]

use std::env::args;

mod array;
mod astar;
mod bitset;
mod days;
mod grid;
mod iter;
mod matcher;
mod solution;
mod vec2;
mod or;
mod unzip;
pub fn main() {
    let num_to_run = args().last().and_then(|a|{
        a.parse::<usize>().ok()
    }).unwrap_or(days::SOLUTIONS.len()-1);
    let solution = days::SOLUTIONS[num_to_run-1];
    let input_number = num_to_run;
    println!("{}", (solution)(input_number));
}
