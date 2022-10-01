// parse input into range objects
use std::{ops::{Deref, DerefMut}, fmt::Display};

#[macro_export]
macro_rules! cub {
    ($x:expr, $y:expr, $z:expr) => {
        Cuboid([$x, $y, $z])
    };
}


type Instruction = (bool, Cuboid);
type Grid = Vec<Cuboid>;

const X: usize = 0;
const Y: usize = 1;
const Z: usize = 2;
const S: usize = 0;
const E: usize = 1;

pub fn get_solution_1() -> isize {
    let mut grid = vec![];
    for (turns_on, cuboid) in get_input().into_iter().take(20) {
    grid = parse_instruction(turns_on, cuboid, grid);
    }
    println!("Total of {} cuboids.", grid.len());
    grid.iter().fold(0, |acc, cub| acc + cub.count_cubes())
}

pub fn get_solution_2() -> isize {
    let mut grid = vec![];
    for (turns_on, cuboid) in get_input() {
    grid = parse_instruction(turns_on, cuboid, grid);
    }
    println!("Total of {} cuboids.", grid.len());
    grid.iter().fold(0, |acc, cub| acc + cub.count_cubes())
}

#[derive(PartialEq, Debug, Clone)]
struct Cuboid([[isize; 2]; 3]);

impl Deref for Cuboid {
    type Target = [[isize; 2]; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Cuboid {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Cuboid {

    fn count_cubes(&self) -> isize {
        (self[X][E] - self[X][S] + 1) *
        (self[Y][E] - self[Y][S] + 1) *
        (self[Z][E] - self[Z][S] + 1)
    }

    // TODO: Rewrite this so it is not ugly anymore
    /// Splits this cuboid into smaller cuboids, that do not overlap with other
    fn split(self, intersection: &Cuboid) -> Vec<Cuboid> {
        if self.intersection(intersection).is_none() {
            return vec![self];
        }

        let mut split_cuboids = vec![];

        let min_x = if self[X][S] < intersection[X][S] {
            split_cuboids.push(Cuboid([[self[X][S], intersection[X][S] - 1], self[Y], self[Z]]));
            intersection[X][S]
        } else {
            self[X][S]
        };

        let min_y = if self[Y][S] < intersection[Y][S] {
            split_cuboids.push(Cuboid([[min_x, self[X][E]], [self[Y][S], intersection[Y][S] - 1], self[Z]]));
            intersection[Y][S]
        } else {
            self[Y][S]
        };

        let min_z = if self[Z][S] < intersection[Z][S] {
            split_cuboids.push(Cuboid([[min_x, self[X][E]], [min_y, self[Y][E]], [self[Z][S], intersection[Z][S] - 1]]));
            intersection[Z][S]
        } else {
            self[Z][S]
        };
 
        let max_x = if self[X][E] > intersection[X][E] {
            split_cuboids.push(Cuboid([[intersection[X][E] + 1, self[X][E]], [min_y, self[Y][E]], [min_z, self[Z][E]]]));
            intersection[X][E]
        } else {
            self[X][E]
        };

        let max_y = if self[Y][E] > intersection[Y][E] {
            split_cuboids.push(Cuboid([[min_x, max_x], [intersection[Y][E] + 1, self[Y][E]], [min_z, self[Z][E]]]));
            intersection[Y][E]
        } else {
            self[Y][E]
        };

        if self[Z][E] > intersection[Z][E] {
            split_cuboids.push(Cuboid([[min_x, max_x], [min_y, max_y], [intersection[Z][E] + 1, self[Z][E]]]));     
        }

        split_cuboids
    }

    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        Some([
            [self[X][S].max(other[X][S]), self[X][E].min(other[X][E])],
            [self[Y][S].max(other[Y][S]), self[Y][E].min(other[Y][E])],
            [self[Z][S].max(other[Z][S]), self[Z][E].min(other[Z][E])]
        ]).filter(|c| c.iter().all(|[start, end]| start <= end))
          .map(|cuboid| Cuboid(cuboid))
    }
}

impl Display for Cuboid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}-{}, {}-{}, {}-{})", self[X][S], self[X][E], self[Y][S], self[Y][E], self[Z][S], self[Z][E])
    }
}

impl From<[[isize; 2]; 3]> for Cuboid {
    fn from(vals: [[isize; 2]; 3]) -> Self {
        Cuboid([vals[0], vals[1], vals[2]])
    }
}

fn parse_instruction(turns_on: bool, cuboid: Cuboid, mut grid: Grid) -> Grid{
    if turns_on {
        let mut added = vec![];
        add_to_grid(cuboid, &mut grid, &mut added);
        grid.append(&mut added);
        grid
    } else {
        let mut new_grid = vec![];
        remove_from_grid(cuboid, grid, &mut new_grid);
        new_grid
    }
}

fn add_to_grid(cuboid: Cuboid, grid: &mut [Cuboid], added: &mut Vec<Cuboid>) {
    match grid.get(0) {
        Some(other_cuboid) => cuboid.split(other_cuboid) 
                                    .into_iter()
                                    .for_each(|cuboid| add_to_grid(cuboid, &mut grid[1..], added)),
        None => added.push(cuboid)
    }
}

fn remove_from_grid(cuboid: Cuboid, grid: Grid, new_grid: &mut Grid) {
    for grid_cuboid in grid {
        new_grid.append(&mut grid_cuboid.split(&cuboid));
    }
}

fn get_input() -> Vec<Instruction> {
    parse_data(include_str!("../data/day_22.txt"))
}

fn parse_data(input: &str) -> Vec<Instruction> {
    input
        .split('\n')
        .map(|line| {
            let cuboid = line[line.find('x').unwrap()..]
                            .split(',')
                            .map(|range| {
                                let delim = range.find('.').unwrap();
                                let low = range[2..delim].parse::<isize>().unwrap();
                                let high = range[delim + 2..].parse::<isize>().unwrap();
                                if low < high {
                                    [low, high]
                                } else {
                                    [high, low]
                                }
                            })
                            .collect::<Vec<[isize; 2]>>();
            (line.starts_with("on "), [cuboid[X], cuboid[Y], cuboid[Z]].into())
        })
        .collect::<Vec<Instruction>>()
}