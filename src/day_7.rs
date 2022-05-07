use crate::split_commas_32;

pub fn get_solution_1() -> u32 {
    let input = split_commas_32("data/day_7.txt");
    brute_force_fuel(input)
}

pub fn get_solution_2() -> u32 {
    let input = split_commas_32("data/day_7.txt");
    brute_force_fuel_2(input)
}

fn brute_force_fuel(input: Vec<u32>) -> u32 {
    let max = *input.iter().max().unwrap();
    let mut min_fuel = u32::MAX;
    for pos in 0..max {
        let mut cur_fuel = 0;
        for n in &input {
            cur_fuel += (pos).abs_diff(*n)
        }
        if cur_fuel < min_fuel {
            min_fuel = cur_fuel;
        }
    }

    min_fuel
}

fn brute_force_fuel_2(input: Vec<u32>) -> u32 {
    let max = *input.iter().max().unwrap();
    let mut min_fuel = u32::MAX;
    for pos in 0..max {
        let mut cur_fuel = 0;
        for n in &input {
            let steps = (pos).abs_diff(*n);
            cur_fuel += steps * (steps + 1) / 2;
        }
        if cur_fuel < min_fuel {
            min_fuel = cur_fuel;
        }
    }

    min_fuel
}

#[test]
fn test_sum() {
    let result = get_solution_1();
    assert_eq!(result, 37);
}