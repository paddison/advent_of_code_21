use std::{ops::{Deref, DerefMut}, fmt::Display};

pub fn get_solution_1() -> usize {
    let mut steps = 0;
    let mut g = parse(include_str!("../data/day_25.txt"));
    while g.do_move() {
        steps += 1;
    }
    steps + 1
}

#[derive(Clone, Copy, PartialEq)]
enum Cucumber {
    South,
    East,
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Option<Cucumber>>,
    dim: (usize, usize) // cols, rows
}

impl Deref for Grid {
    type Target = Vec<Option<Cucumber>>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for (i, ele) in self.grid.iter().enumerate() {
            if i % (self.dim.0) == 0 {
                string.push('\n');
            }
            match ele {
                Some(Cucumber::East) => string.push('>'),
                Some(Cucumber::South) => string.push('v'),
                None => string.push('.'),
            }
        }

        write!(f, "{}", &string[1..])
    }
}

impl Grid {
    fn get(&self, row: usize, col: usize) -> Option<&Cucumber> {
        let index = self.calculate_index(row, col);
        self.grid.get(index).unwrap_or(&None).as_ref()
    }

    fn calculate_index(&self, row: usize, col: usize) -> usize {
        self.dim.0 * row + col
    }

    fn calculate_pos(&self, index: usize) -> (usize, usize) {
        (index / self.dim.0, index % self.dim.0)
    }

    fn calculate_neighbour_position(&self, row: usize, col: usize, cuc: Cucumber) -> (usize, usize) {
        match cuc {
            Cucumber::East => (row, (col + 1) % self.dim.0),
            Cucumber::South => ((row + 1) % self.dim.1, col)
        }
    }

    fn try_move_cucumber(&mut self, row: usize, col: usize, cuc: Cucumber) {
        let (new_row, new_col) = self.calculate_neighbour_position(row, col, cuc);
        if self.get(new_row, new_col).is_none() {
            let i = self.calculate_index(row, col);
            let j = self.calculate_index(new_row, new_col);
            self[i] = None;
            self[j] = Some(cuc);
        }
    }

    fn do_move(&mut self) -> bool {
        // move east first
        let mut east_grid = self.clone();
        let mut south_facing = Vec::new();
        for (i, cuc) in self.grid.iter().enumerate() {
            let (row, col) = self.calculate_pos(i);
            // refactor moving into methods
            match cuc {
                Some(Cucumber::East) => east_grid.try_move_cucumber(row, col, Cucumber::East),
                Some(Cucumber::South) => south_facing.push((row, col)),
                None => (),
            }
        }

        // move south
        let mut south_grid = east_grid.clone();
        for (row, col) in south_facing {
            south_grid.try_move_cucumber(row, col, Cucumber::South)
        }
        if self.grid == south_grid.grid {
            self.grid = south_grid.grid;
            false
        } else {
            self.grid = south_grid.grid;
            true
        }
    }
}

fn parse(input: &str) -> Grid {
    let mut grid = Vec::new();
    let n_cols = input.find('\n').unwrap();
    let mut n_rows = 0;
    for line in input.split('\n') {
        for el in line.chars().map(|c| {
            match c {
                '.' => None,
                '>' => Some(Cucumber::East),
                'v' => Some(Cucumber::South),
                _ => panic!("invalid input"),
            }
        }) {
            grid.push(el);
        }
        n_rows += 1;
    }

    Grid{ grid, dim: (n_cols, n_rows) }
}

#[test]
fn test_stop() {
    let mut steps = 0;
    let mut g = parse(include_str!("../data/day_25_test.txt"));
    while g.do_move() {
        steps += 1;
        if steps % 10 == 0 {
            println!("{}\n", g);
        }
    }
    steps += 1;
    assert_eq!(steps, 58);
}