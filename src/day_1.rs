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

#[cfg(test)]
mod tests {
    use crate::day_1::get_solution_1;
    use crate::day_1::get_solution_2;

    #[test]
    fn four_increases() {
        let measurements = vec![1, 2, 1, 3, 4, 2, 5];
        let result = get_solution_1();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_one_increase() {
        let measurements = vec![199, 200, 208, 210];
        let result = get_solution_2();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_two_increase() {
        let measurements = vec![1, 1, 1, 2, 1, 3];
        let result = get_solution_2();
        assert_eq!(result, 2);
    }
}