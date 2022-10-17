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

pub fn get_solution_1() -> u32 {
    let input = parse_lines("data/day_2.txt");
    calculate_position(input)
}

pub fn get_solution_2() -> u32 {
    let input = parse_lines("data/day_2.txt");
    calculate_position_with_aim(input)
}