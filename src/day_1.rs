pub fn count_increases(measurements: Vec<i32>) -> u32 {
    let mut increases = 0;
    for i in 1..measurements.len() {
        if measurements[i] > measurements[i - 1] {
            increases += 1;
        }
    }
    increases
}

pub fn count_increases_consecutive(measurements: Vec<i32>) -> u32 {
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

pub fn parse_measurements_to_int(measurements: Vec<String>) -> Vec<i32>{
    measurements.into_iter().map(|n| n.parse::<i32>().expect("unable to parse to number")).collect()
}

#[cfg(test)]
mod tests {
    use crate::day_1::count_increases;
    use crate::day_1::count_increases_consecutive;

    #[test]
    fn four_increases() {
        let measurements = vec![1, 2, 1, 3, 4, 2, 5];
        let result = count_increases(measurements);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_one_increase() {
        let measurements = vec![199, 200, 208, 210];
        let result = count_increases_consecutive(measurements);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_two_increase() {
        let measurements = vec![1, 1, 1, 2, 1, 3];
        let result = count_increases_consecutive(measurements);
        assert_eq!(result, 2);
    }
}