use std::time::Instant;

use advent_of_code::day_14;


fn main() {
    let start = Instant::now();
    let result = day_14::get_solution_2(false, 41);
    let end = start.elapsed().as_millis();
    println!("result is: {}, took {} millis", result, end); // sol is 150426
}