use std::time::Instant;

use advent_of_code::day_23;


fn main() {
    let start = Instant::now();
    let result = day_23::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("result is: {}, took {} micros", result, end);
}