use std::{ops::Deref, fmt::{Display, Formatter, Write}};

const BOARD_WIDTH: usize = 13;

pub fn get_solution_1() -> usize {
    0
}

fn get_input() -> &'static str {
    include_str!("../data/day_23.txt")
}

struct Burrow {
    board: Vec<Vec<Tile>>,
    pods: Vec<Amphipod>,
}

impl Display for Burrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for row in &self.board {
            for tile in row {
                let _ = match tile {
                    Tile::Empty => write!(board, " "),
                    Tile::Wall =>  write!(board, "#"),
                    _ =>  write!(board, "."),
                };
            }
            let _ = write!(board, "\n");
        }

        for pod in &self.pods {
            let index = pod.pos.0 + pod.pos.1 * BOARD_WIDTH + pod.pos.1; // y position to account for \n chars
            board.replace_range(index..index + 1, pod.p_type.into())
        }

        write!(f, "{}", board)
    }
}

impl From<&str> for Burrow {
    fn from(input: &str) -> Self {
        let mut room_count = -1;
        // let mut amphipods = vec![];
        let char_to_tile = move |c| match c {
            '#' => Tile::Wall,
            '.' => Tile::Hallway,
            c => if c.is_alphabetic() {
                    room_count = (room_count + 1) % 4;
                    match room_count {
                        0 => Tile::AmberRoom,
                        1 => Tile::BronzeRoom,
                        2 => Tile::CopperRoom,
                        3 =>Tile::DesertRoom,
                        _ => panic!("room_count cannot be more than 3."),
                    }
                } else {
                    Tile::Empty
                }
            ,
        };

        fn update_position(position: &mut (usize, usize)) -> (usize, usize) {
            let old_position = position.clone();
            position.0 = (position.0 / 2 % 4) * 2 + 3;
            if position.0 == 3 {
                position.1 += 1;
            }
            old_position
        }

        let board = input.split('\n')
                         .map(|s| s.chars()
                                   .map(char_to_tile) 
                                   .collect::<Vec<Tile>>())
                         .collect::<Vec<Vec<Tile>>>();
        let mut position = (3, 2);
        let amphipods = input.split('\n').skip(2).take(2)
                             .map(|line| line.chars().skip(3).step_by(2).take(4)
                                             .map::<Amphipod, _>(|c| Amphipod::new(c.into(), update_position(&mut position)))
                                             .collect::<Vec<Amphipod>>())
                             .flatten()
                             .collect::<Vec<Amphipod>>();

        
        Burrow { board, pods: amphipods }
    }
}

#[derive(Debug)]
enum Tile {
    Empty,
    Wall,
    Hallway,
    AmberRoom,
    BronzeRoom,
    CopperRoom,
    DesertRoom,
}

struct Amphipod {
    p_type: PodType,
    pos: (usize, usize),
    has_moved: bool,
}

impl Amphipod {
    fn new(p_type: PodType, position: (usize, usize)) -> Self {
        Self {p_type, pos: position, has_moved: false }
    }

    fn cost(&self) -> usize {
        **self as usize
    }

    fn can_move(&self) -> bool {
        false
    }

    fn is_home(&self, burrow: &Burrow) -> bool {
        if self.pos.1 == 1 {
            return false;
        }

        // get all pods below our pod
        for pod in burrow.pods
                            .iter()
                            .filter(|p| p.pos.0 == self.pos.0 && self.pos.1 > self.pos.1 && self.pos.1 > burrow.board.len()) {
            if pod.p_type != self.p_type {
                return false;
            }
        }

        true
    }           
}

impl Deref for Amphipod {
    type Target = PodType;

    fn deref(&self) -> &Self::Target {
        &self.p_type
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       let pod = match self.p_type {
        PodType::Amber => "A",
        PodType::Bronze => "B",
        PodType::Copper => "C",
        PodType::Desert => "D",
    };

        write!(f, "{}", pod)
    }
}

#[derive(Copy, Clone, PartialEq)]
enum PodType {
    Amber = 1,
    Bronze = 10,
    Copper = 100,
    Desert = 1000,
}

impl From<char> for PodType {
    fn from(c: char) -> Self {
        match c {
            'A' => PodType::Amber,
            'B' => PodType::Bronze,
            'C' => PodType::Copper,
            'D' => PodType::Desert,
            _ => panic!("Invalid Char for Podtype")
        }
    }
}

impl From<PodType> for &str {
    fn from(pod: PodType) -> Self {
        match pod {
            PodType::Amber => "A",
            PodType::Bronze => "B",
            PodType::Copper => "C",
            PodType::Desert => "D",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Amphipod;
    use super::Burrow;
    use super::PodType::*;

    fn get_test_input() -> &'static str{
        include_str!("../data/day_23_test.txt")
    }

    #[test]
    fn test_amphi_cost() {
        let (amber, bronze, copper, desert) = (Amphipod::new(Amber, (0, 0)), Amphipod::new(Bronze, (0, 0)), Amphipod::new(Copper, (0, 0)), Amphipod::new(Desert, (0, 0)));
        assert_eq!(amber.cost(), 1);
        assert_eq!(bronze.cost(), 10);
        assert_eq!(copper.cost(), 100);
        assert_eq!(desert.cost(), 1000);
    }

    #[test]
    fn test_burrow_from_str() {
        let burrow: Burrow = get_test_input().into();
        for p in burrow.pods.iter().map(|ap| ap.pos) {
            println!("{:?}", p);
        }
        println!("{}", burrow);
    }
}