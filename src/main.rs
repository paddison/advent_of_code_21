use std::time::Instant;

use advent_of_code::day_24;


fn main() {
    let start = Instant::now();
    let result = day_24::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("result is: {}, took {} micros", result, end);
}