use crate::parse_lines;

type BoardMap = (i32, bool);

fn parse_board(input: Vec<String>) -> (Vec<i32>, Vec<Vec<BoardMap>>) {
    let numbers: Vec<i32> = input[0].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    let mut boards: Vec<Vec<BoardMap>> = vec![];
    let mut board: Vec<BoardMap> = vec![];
    for i in 2..input.len() {
        if input[i].len() == 0 {
            boards.push(board.clone());
            board = vec![];
        } else {
            board.append(&mut input[i].split_whitespace().map(|x| (x.parse::<i32>().unwrap(), false)).collect::<Vec<_>>()); 
        }
    }
    boards.push(board.clone());

    (numbers, boards)
}

fn determine_winner(numbers: &Vec<i32>, boards: &mut Vec<Vec<BoardMap>>) -> i32 {

    for n in numbers {
        for board in boards.iter_mut(){
            let index = index_of(*n, &board); 
            if index >= 0 {
                board[index as usize].1 = true
            }
            if  is_row_complete(&board) || is_column_complete(&board) {
                let sum: i32 = board.into_iter()
                    .filter(|(_, marked)| !marked)
                    .map(|(n, _)| *n)
                    .sum();

                return sum * n;
            }
        }
    }
    -1
}    

fn is_row_complete(board: &Vec<BoardMap>) -> bool {
    for i in 0..5 {
        if board[i * 5..i * 5 + 5].iter().all(|(_, is_marked)| *is_marked) {
            return true;
        }
    }
    false
}

fn is_column_complete(board: &Vec<BoardMap>) -> bool {
    for i in 0..5 {
        let mut is_winner = true;
        for j in 0..5 {
            if !board[i + 5 * j].1 {
                is_winner = false;
            }
        }
        if is_winner {
            return true;
        } 
    }
    false
}

fn index_of(item: i32, container: &Vec<BoardMap>) -> i32 
{
    for (i, (n, _)) in container.into_iter().enumerate() {
        if *n == item {
            return i as i32;
        }
    }
    -1
}

fn determine_last_winner(numbers: Vec<i32>, mut boards: Vec<Vec<BoardMap>>) -> i32 {

    for n in &numbers {
        if n == &16 {
            println!("");
        }
        for board in boards.iter_mut() {
            let index = index_of(*n, board); 
            if index >= 0 {

                board[index as usize].1 = true
            }
        }
        if boards.len() > 1 {
            boards.retain(|b| !(is_row_complete(b) || is_column_complete(b)));
        } else if is_row_complete(&boards[0]) || is_column_complete(&boards[0]) {
            let sum: i32 = boards[0].iter()
                    .filter(|(_, marked)| !marked)
                    .map(|(n, _)| *n)
                    .sum();

            return sum * n;
        }
    }
    -1
}


fn print_board(board: &Vec<BoardMap>) {
    let mut board_string = String::new();
    for (i, (n, marked)) in board.iter().enumerate() {
        if i != 0 && i % 5 == 0 {
            board_string += "\n";
        }
        
        if *marked {
            board_string += &format!("{}. ", n);
        } else {
            board_string += &format!("{} ", n);
        }

    }
    println!("{}", board_string);
}


pub fn solve_4_1(file_name: &str) -> i32 {
    let input = parse_lines(file_name);
    let (numbers,  mut boards) = parse_board(input);
    determine_winner(&numbers, &mut boards)
}

pub fn solve_4_2(file_name: &str) -> i32 {
    let input = parse_lines(file_name);
    let (numbers,  boards) = parse_board(input);
    determine_last_winner(numbers, boards)
}

#[cfg(test)]
mod tests {
    use super::{parse_board, determine_last_winner};
    use crate::{parse_lines, day_4::determine_winner};

    #[test]
    fn test_parse_board() {
        let input = parse_lines("data/day_4.txt");
        let (numbers, mut boards) = parse_board(input);
        // println!("{:?}", numbers);
        println!("{:?}", boards.len());
        let result = determine_winner(&numbers, &mut boards);
        println!("{}", result);
    }

    #[test]
    fn test_determine_last_winner() {
        let input = parse_lines("data/day_4_test.txt");
        let (numbers, boards) = parse_board(input);
        let result = determine_last_winner(numbers, boards);
        assert_eq!(1924, result);
    }
}