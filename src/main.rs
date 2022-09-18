use std::time::Instant;

use advent_of_code::day_22;


fn main() {
    let start = Instant::now();
    let result = day_22::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("result is: {}, took {} micros", result, end);
}