pub fn get_solution_1() -> usize {
    let measurements = parse(include_str!("../data/day_1.txt"));
    let mut increases = 0;
    for i in 1..measurements.len() {
        if measurements[i] > measurements[i - 1] {
            increases += 1;
        }
    }
    increases
}

pub fn get_solution_2() -> usize {
    let measurements = parse(include_str!("../data/day_1.txt"));
    let mut increases = 0;
    let mut sum_two = measurements[0] + measurements[1] + measurements[2];
    for i in 3..measurements.len() {
        let sum_one = sum_two;
        sum_two = measurements[i - 0] + measurements[i - 1] + measurements[i - 2];
        if sum_two > sum_one {
            increases += 1;
        }
    }

    increases
}

pub fn parse(input: &str) -> Vec<i32>{
    input.split('\n').map(|n| n.trim().parse::<i32>().expect("unable to parse to number")).collect()
}