use std::time::Instant;


use advent_of_code::*;


fn main() {
    let start_all = Instant::now();

    let start = Instant::now();
    let result = day_1::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 1.1:\t{:>7}us,\t{}", end, result);

    let start = Instant::now();
    let result = day_1::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 1.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_2::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 2.1:\t{:>7}us,\t{}", end, result);

    let start = Instant::now();
    let result = day_2::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 2.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_3::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 3.1:\t{:>7}us,\t{}", end, result);

    let start = Instant::now();
    let result = day_3::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 3.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_4::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 4.1:\t{:>7}us,\t{}", end, result);

    let start = Instant::now();
    let result = day_4::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 4.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_5::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 5.1:\t{:>7}us,\t{}", end, result);

    let start = Instant::now();
    let result = day_5::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 5.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_6::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 6.1:\t{:>7}us,\t{}", end, result);

    let start = Instant::now();
    let result = day_6::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 6.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_7::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 7.1:\t{:>7}us,\t{}", end, result);

    let start = Instant::now();
    let result = day_7::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 7.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_8::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 8.1:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_8::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 8.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_9::get_solution_1(false);
    let end = start.elapsed().as_micros();
    println!("Day 9.1:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_9::get_solution_2(false);
    let end = start.elapsed().as_micros();
    println!("Day 9.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_10::get_solution_1(false);
    let end = start.elapsed().as_micros();
    println!("Day 10.1:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_10::get_solution_2(false);
    let end = start.elapsed().as_micros();
    println!("Day 10.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_11::get_solution_1(false);
    let end = start.elapsed().as_micros();
    println!("Day 11.1:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_11::get_solution_2(false);
    let end = start.elapsed().as_micros();
    println!("Day 11.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_12::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 12.1:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_12::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 12.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_13::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 13.1:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_13::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 13.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_14::get_solution_1(false);
    let end = start.elapsed().as_micros();
    println!("Day 14.1:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_14::get_solution_2(false, 100);
    let end = start.elapsed().as_micros();
    println!("Day 14.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_15::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 15.1:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_15::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 15.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_16::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 16.1:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_16::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 16.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_17::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 17.1:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_17::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 17.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_18::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 18.1:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_18::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 18.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_19::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 19.1:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_19::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 19.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_20::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 20.1:\t{:>7}us\t{}", end, result);

    // let start = Instant::now();
    // let result = day_20::get_solution_1();
    // let end = start.elapsed().as_micros();
    // println!("Day 20.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_21::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 21.1:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_21::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 21.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_22::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 22.1:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_22::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 22.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_23::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 23.1:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_23::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 23.2:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_24::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 24.1:\t{:>7}us\t{}", end, result);

    let start = Instant::now();
    let result = day_24::get_solution_2();
    let end = start.elapsed().as_micros();
    println!("Day 24.2:\t{:>7}us\t{}", end, result);


    let result = day_25::get_solution_1();
    let end = start.elapsed().as_micros();
    println!("Day 25.1:\t{:>7}us\t{}", end, result);

    let end_all = start_all.elapsed().as_micros();
    println!("Day 1 to 25 took {} micros", end_all);

    // 96918996924991
    // 91811241911641

}