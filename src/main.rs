use std::time::Instant;

use advent_of_code::day_25;


fn main() {
    let start = Instant::now();
    let result = day_25::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("result is: {}, took {} micros", result, end);
    // 96918996924991
    // 91811241911641

}