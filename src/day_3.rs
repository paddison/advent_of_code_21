use crate::parse_lines;

fn binary_diagnostic(input: Vec<String>) -> u32 {
    let line_length = input[0].len();
    let mut pos = vec![0 as i32; line_length];
    for line in input.iter().map(|line| line.as_bytes()) {
        for i in 0..line_length {
            if line[i] == 0x30 { pos[i] -= 1 } else { pos[i] += 1}; 
        }
    }

    let mut result = 0;
    pos.reverse();
    for i in 0..pos.len() {
        if pos[i] > 0 { result += 2_u32.pow(i as u32)}
    }
    result * (!result & (2_u32.pow(line_length as u32) - 1)) 
}

pub fn get_solution_1() -> u32 {
    let input = parse_lines("data/day_3.txt");
    binary_diagnostic(input)
}

fn determine_oxygen_rating(input: &Vec<String>) -> u32 {
    filter_values(input, '0', '1')
}

fn determine_o2_rating(input: &Vec<String>) -> u32 {
    filter_values(input, '1', '0')
}

fn filter_values(input: &Vec<String>, most_common_criteria_1: char, most_common_criteria_2: char) -> u32{
    let mut common_bit;
    let mut filtered = input.clone();
    for i in 0..input[0].len() {
        common_bit = if most_common_bit(i, &filtered) < 0 { most_common_criteria_1 } else { most_common_criteria_2 };
        filtered = filtered.into_iter().filter(|n| n.chars().nth(i).unwrap() == common_bit).collect();
        if filtered.len() == 1 {
            break;
        }
    }
    assert_eq!(filtered.len(), 1);
    u32::from_str_radix(&filtered[0], 2).unwrap()
}

fn most_common_bit(index: usize, input: &Vec<String>) -> i32 {
    let mut common_bit = 0;
    for line in input {
        // Unwrap is safe, since index is bound by line length
        if line.chars().nth(index).unwrap() == '0' {
            common_bit -= 1;
        } else {
            common_bit += 1;
        }
    }
    if common_bit == 0 {
        1      
    } else {
        common_bit
    }
}

pub fn get_solution_2() -> u32 {
    let input = parse_lines("data/day_3.txt");
    let oxygen_rating = determine_oxygen_rating(&input);
    let o2_rating = determine_o2_rating(&input);
    oxygen_rating * o2_rating
}

#[cfg(test)]
mod tests {
    use crate::day_3::binary_diagnostic;

    use super::{determine_oxygen_rating, determine_o2_rating};

    #[test]
    fn test_198() {
        let input: Vec<String> = vec!["00100",
                                "11110",
                                "10110",
                                "10111",
                                "10101",
                                "01111",
                                "00111",
                                "11100",
                                "10000",
                                "11001",
                                "00010",
                                "01010"].into_iter().map(|s| String::from(s)).collect();
        
        let result = binary_diagnostic(input);
        assert_eq!(result, 198)
    }

    #[test]
    fn test_o2_oxygen_rating() {
        let input: Vec<String> = vec!["00100",
                                "11110",
                                "10110",
                                "10111",
                                "10101",
                                "01111",
                                "00111",
                                "11100",
                                "10000",
                                "11001",
                                "00010",
                                "01010"].into_iter().map(|s| String::from(s)).collect();
        let oxygen_rating = determine_oxygen_rating(&input);
        assert_eq!(oxygen_rating, 23);
        let o2_rating = determine_o2_rating(&input);
        assert_eq!(o2_rating, 10);
    }
}