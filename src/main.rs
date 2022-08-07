use std::time::Instant;

use advent_of_code::{day_14, day_16};


fn main() {
    let start = Instant::now();
    let result = day_16::get_solution_1();
    let end = start.elapsed().as_millis();
    println!("result is: {}, took {} millis", result, end); // sol is 150426
}