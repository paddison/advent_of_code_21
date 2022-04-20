use crate::parse_lines;

fn calculate_position(input: Vec<String>) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for line in input {
        let line: Vec<&str> = line.split(" ").collect();
        let (direction, unit) = (line[0], line[1].parse::<u32>().expect("Parsing error"));
        match direction {
            "forward" => { horizontal += unit; },
            "up" => { depth -= unit; },
            "down" => { depth += unit; },
            direction => { eprintln!("Got invalid input {}", direction) },
        };
    }
    horizontal * depth
}

fn calculate_position_with_aim(input: Vec<String>) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input {
        let line: Vec<&str> = line.split(" ").collect();
        let (direction, unit) = (line[0], line[1].parse::<u32>().expect("Parsing error"));
        match direction {
            "forward" => { 
                horizontal += unit; 
                depth += unit * aim;
            },
            "up" => { aim -= unit; },
            "down" => { aim += unit; },
            direction => { eprintln!("Got invalid input {}", direction) },
        };
    }
    horizontal * depth
}

pub fn solve_day_2_1(file_name: &str) -> u32 {
    let input = parse_lines(file_name);
    calculate_position(input)
}

pub fn solve_day_2_2(file_name: &str) -> u32 {
    let input = parse_lines(file_name);
    calculate_position_with_aim(input)
}