use crate::parse_lines;

pub fn get_solution_1(is_test: bool) -> usize {
    // build a stack of opening brackets
    // push opening brackets, if closing bracket is found, compare
    // if valid pop of stack, otherwise return
    let file_name = if is_test { "data/day_10_test.txt" } else { "data/day_10.txt" };
    let lines = parse_lines(file_name);
    let mut score = 0;
    for line in lines {
        let mut stack = vec![];
        for bracket in line.chars() {
            if is_opening(bracket) {
                stack.push(bracket);
            } else if is_valid_closing(stack[stack.len() - 1], bracket){
                let _ = stack.pop();
            } else {
                score += get_bracket_score(bracket);
                break;
            }
        }
    }
    score
}

pub fn get_solution_2(is_test: bool) -> usize {
    let file_name = if is_test { "data/day_10_test.txt" } else { "data/day_10.txt" };
    let lines = parse_lines(file_name);
    let mut sums = vec![];
    for stack in discard_corrupted(lines) {
        sums.push(get_stack_score(stack));
    }
    sums.sort();
    sums[sums.len() / 2]
}

fn is_opening(bracket: char) -> bool {
    bracket == '(' || bracket == '[' || bracket == '{' || bracket == '<'
}

fn is_valid_closing(opening: char, closing: char) -> bool {
    match opening {
        '(' => closing == ')',
        '[' => closing == ']',
        '{' => closing == '}',
        '<' => closing == '>',
        invalid => panic!("Got invalid bracket: {}", invalid)
    }
}

fn get_bracket_score(bracket: char) -> usize {
    match bracket {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        invalid => panic!("Got invalid bracket: {}", invalid)
    }
}

// discard corrputed lines and return stacks of each line
fn discard_corrupted(lines: Vec<String>) -> Vec<Vec<char>> {
    let mut incomplete = vec![];
    for line in &lines {
        let mut stack = vec![];
        let mut is_corrupted = false;
        for bracket in line.chars() {
            if is_opening(bracket) {
                stack.push(bracket);
            } else if is_valid_closing(stack[stack.len() - 1], bracket){
                let _ = stack.pop();
            } else {
                is_corrupted = true;
            }
        }

        if !is_corrupted {
            incomplete.push(stack);
        }
    } 

    incomplete
}


fn get_stack_score(stack: Vec<char>) -> usize {
    stack.iter().rev().fold(0, |sum, bracket| {
        sum * 5 + match bracket {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            invalid => panic!("Got invalid bracket: {}", invalid)
        }
    })

}

#[cfg(test)]
mod tests {
    use crate::{parse_lines, day_10::get_stack_score};

    use super::{get_solution_1, discard_corrupted, get_solution_2};
    #[test]
    fn test_get_solution_1() {
        let solution = get_solution_1(true);
        assert_eq!(solution, 26397);
    }

    #[test]
    fn test_get_stack_score() {
        let stack = "[({([[{{".chars().into_iter().collect::<Vec<char>>();
        assert_eq!(get_stack_score(stack), 288957);
    }

    #[test]
    fn test_get_solution_2() {
        let solution = get_solution_2(true);
        assert_eq!(solution, 288957);
    }
}