// parse input into range objects
use crate::cub;
use std::{ops::{Range, Deref, DerefMut}, fmt::Display};

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

        // let intersection = self.intersection(intersection).unwrap();

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
        // println!("{:?}", split_cuboids);
        split_cuboids
    }

    // fn does_not_overlap(&self, other: &Cuboid) -> bool {
    //     (self[X].1 < other[X].0 || self[X].0 > other[X].1) ||
    //     (self[Y].1 < other[Y].0 || self[Y].0 > other[Y].1) ||
    //     (self[Z].1 < other[Z].0 || self[Z].0 > other[Z].1)
    // }
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

// #[cfg(test)]
// mod tests {
//     use super::*;


//     fn get_test_input() -> Vec<Instruction> {
//         parse_data(include_str!("../data/day_22_test.txt"))
//     }


//     fn get_larger_test_input() -> Vec<Instruction> {
//         parse_data(include_str!("../data/day_22_test_large.txt"))
//     }

//     fn get_part_2_test_input() -> Vec<Instruction> {
//         parse_data(include_str!("../data/day_22_test_part_2.txt"))
//     }

//     #[test]
//     fn test_parse_data() {
//         let actual = get_test_input();
//         assert_eq!(actual.len(), 4);
//         assert_eq!(actual[0], (true, vec!((10,12), (10,12), (10,12))).into());
//         assert_eq!(actual[1], (true, vec!((11,13), (11,13), (11,13))).into());
//         assert_eq!(actual[2], (false, vec!((9,11), (9,11), (9,11))).into());
//         assert_eq!(actual[3], (true, vec!((10,10), (10,10), (10,10))).into());
//     }

//     #[test]
//     fn test_add_subtract() {
//         let instructions = get_test_input();
//         let mut cubes = HashSet::<Coordinates>::new();
        
//         let first = instructions[0].clone();
//         first.apply(&mut cubes);
//         assert_eq!(cubes.len(), 27);

//         let second = instructions[1].clone();
//         second.apply(&mut cubes);
//         assert_eq!(cubes.len(), 46);

//         let third = instructions[2].clone();
//         third.apply(&mut cubes);
//         assert_eq!(cubes.len(), 38);

//         let fourth = instructions[3].clone();
//         fourth.apply(&mut cubes);
//         assert_eq!(cubes.len(), 39);
//     }

//     #[test]
//     fn test_verify_range() {
//         let valid_instrucion = Instruction{ turns_on: true, cuboid: vec![(-50, 51), (-50, 51), (-50, 51)].into() };
//         assert!(valid_instrucion.verify());

//         let invalid_instruction = Instruction{ turns_on: true, cuboid: vec![(-50, 50), (-50, 50), (-51, 50)].into() };
//         assert!(!invalid_instruction.verify());

//         let invalid_instruction = Instruction{ turns_on: true, cuboid: vec![(-50, 50), (-50, 50), (-51, 50)].into() };
//         assert!(!invalid_instruction.verify());

//         let invalid_instruction = Instruction{ turns_on: true, cuboid: vec![(-50, 52), (-50, 52), (-51, 50)].into() };
//         assert!(!invalid_instruction.verify());

        

//     }

//     #[test]
//     fn test_result_with_test_data() {
//         assert_eq!(calculate_result(get_larger_test_input()), 590784);
//     }

//     #[test]
//     fn test_does_not_overlap() {
//         let compare_cub: Cuboid = [(9, 9), (9, 9), (9, 9)].into();

//         assert!(compare_cub.does_not_overlap(&[(10, 10), (9, 11), (9, 11)].into()));
//         assert!(compare_cub.does_not_overlap(&[(8, 11), (8, 11), (11, 11)].into()));
//         assert!(!compare_cub.does_not_overlap(&[(8, 11), (8, 11), (8, 11)].into()));
//         assert!(!compare_cub.does_not_overlap(&[(9, 9), (8, 11), (8, 11)].into()));
//     }

//     #[test]
//     fn test_split_cuboid() {
//         let cubs = cub![(8, 10), (8, 10), (8, 10)].split(&[(9, 9), (9, 9), (9, 9)].into());
//         for (i, cub_left) in cubs.iter().enumerate() {
//             // println!("{}", cub_left);
//             for cub_right in &cubs[i + 1..] {
//                 assert!(cub_left.does_not_overlap(cub_right));
//             }
//         }

//         let cubs = cub![(8, 10), (9, 10), (9, 10)].split(&[(9, 9), (9, 9), (9, 9)].into());
//         for (i, cub_left) in cubs.iter().enumerate() {
//             println!("{}", cub_left);
//             for cub_right in &cubs[i + 1..] {
//                 assert!(cub_left.does_not_overlap(cub_right));
//             }
//         }
//     }

//     #[test]
//     fn test_count_cubes() {
//         assert_eq!(cub![(8, 10), (8, 10), (8, 10)].count_cubes(), 27);
//         assert_eq!(cub![(9, 10), (8, 10), (8, 10)].count_cubes(), 18);
//     }

//     #[test]
//     fn test_one_split_count() {
//         let cubs = cub![(11, 13), (11, 13), (11, 13)].split(&[(10, 12), (10, 12), (10, 12)].into());
//         let mut count = 0;
//         for cub in cubs {
//             count += cub.count_cubes();
//         }

//         assert_eq!(count, 19);
//     }

//     #[test]
//     fn test_to_add() {
//         let instructions = get_larger_test_input()
//                                 .into_iter()
//                                 .take(20)
//                                 .collect::<Vec<Instruction>>();
//         let instrutctions_2 = instructions.clone();
//         let expected = calculate_result(instructions);

//         let mut grid = vec![];
//         for instruction in instrutctions_2 {
//             grid = parse_instruction(instruction, grid);
//         }

//         let actual = grid.iter().fold(0, |acc, cub| acc + cub.count_cubes());
        
//         assert_eq!(actual, expected);
//     }

//     #[test]
//     fn test_with_test_data() {
//         let instructions = get_part_2_test_input();

//         let mut grid = vec![];
//         for instruction in instructions {
//         grid = parse_instruction(instruction, grid);
//         }

//         let actual = grid.iter().fold(0, |acc, cub| acc + cub.count_cubes());

//         assert_eq!(actual, 2758514936282235);
//     }
// }