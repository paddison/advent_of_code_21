use std::fmt::Display;

use crate::parse_lines;

pub fn get_solution_1() -> usize{
    let lines = parse_lines("data/day_13.txt");
    let (points, folds) = prepare_input(lines);
    let p = Paper::new(points, folds);
    let mut p_iter = p.into_iter();
    let first_fold = p_iter.next().unwrap();
    let mut sum = 0;
    for line in first_fold {
        sum += line.iter().filter(|is_point| **is_point).count();
    }

    sum
}

pub fn get_solution_2() -> usize{
    let lines = parse_lines("data/day_13.txt");
    let (points, folds) = prepare_input(lines);
    let p = Paper::new(points, folds);
    let mut final_pattern = vec![];
    for points in p.into_iter() {
        final_pattern = points;
    }
    print_pattern(final_pattern);
    0
}

fn print_pattern(pattern: Vec<Vec<bool>>) {
    let mut s = String::new();
    for line in pattern {
        for point in line {
            if point {
                s += "#";
            } else {
                s += ".";
            }
        }
        s += "\n";
    }
    println!("{}", s);
}

fn prepare_input(lines: Vec<String>) -> (Vec<(usize, usize)>, Vec<Fold>) {
    let mut points = vec![];
    let mut folds = vec![];
    for line in lines {
        if line.contains(",") {
            let result: Vec<usize> = line.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
            assert_eq!(result.len(), 2);
            points.push((result[0], result[1]));
        } else if line.contains("=") {
            let result = line.split("=").collect::<Vec<&str>>();
            assert_eq!(result.len(), 2);
            let fold = match &result[0][result[0].len() - 1..] {
                "x" => Fold::Left(result[1].parse::<usize>().unwrap()),
                "y" => Fold::Up(result[1].parse::<usize>().unwrap()),
                _ => panic!("Invalid direction")
            };
            let line = result[1].parse::<usize>().unwrap();
            folds.push(fold);
        }
    }
    (points, folds)
} 

#[derive(Debug)]
enum Fold {
    Up(usize),
    Left(usize),
}

#[derive(Debug)]
struct Paper {
    points: Vec<Vec<bool>>,
    folds: Vec<Fold>,
}

impl Paper {
    pub fn new(points: Vec<(usize, usize)>, folds: Vec<Fold>) -> Self {
        // determine size of paper
        let mut x_dim = 0;
        let mut y_dim = 0;
        for i in 0..2 {
            match folds[i] {
                Fold::Up(l) => y_dim = l * 2 + 1,
                Fold::Left(l) => x_dim = l * 2 + 1,
            }
        }
        assert_ne!(x_dim, 0);
        assert_ne!(y_dim, 0);

        let mut m_points = vec![vec![false; x_dim]; y_dim];
        for (x, y) in points {
            m_points[y][x] = true;
        }

        Paper { points: m_points, folds }
    }
}

impl IntoIterator for Paper {
    type Item = Vec<Vec<bool>>;

    type IntoIter = PaperIter;

    fn into_iter(self) -> Self::IntoIter {
        PaperIter { points: self.points, folds: self.folds, index: 0 }
    }
}

struct PaperIter {
    points: Vec<Vec<bool>>,
    folds: Vec<Fold>,
    index: usize,
}

impl Iterator for PaperIter {
    type Item = Vec<Vec<bool>>;

    fn next(&mut self) -> Option<Self::Item> {
        // how to do fold
        // determine new dimensions based on direction
        // if folded vertically, copy backwards until fold position
        // if folded horizontally, copy with modulo of fold position
        if self.index >= self.folds.len() {
            return None;
        }
        let new_points = match self.folds[self.index] {
            Fold::Up(l) => { // get upper part of paper
                let mut v = vec![];
                for i in 0..l {
                    let mut new_line = vec![];
                    let upper = &self.points[i];
                    let lower = &self.points[self.points.len() - i - 1];
                    for (u, l) in upper.iter().zip(lower) {
                        new_line.push(*u || *l);
                    }
                    v.push(new_line);
                }
                
                v
            },
            Fold::Left(l) => { // get left part of paper
                let mut v = vec![];
                for line in &self.points {
                    let mut new_line = line[0..l].to_vec();
                    for (i, folded_point) in line[l + 1..].iter().rev().enumerate() {
                        if *folded_point {
                            new_line[i] = true;
                        }
                    }
                    v.push(new_line);
                }
                v
            },
        };
        self.points = new_points;
        self.index += 1;
        Some(self.points.clone())
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for line in &self.points {
            for point in line {
                if *point {
                    s += "#";
                } else {
                    s += ".";
                }
            }
            s += "\n";
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_lines;

    use super::{prepare_input, Fold, Paper};

    fn get_test_data() -> (Vec<(usize, usize)>, Vec<Fold>) {
        let lines = parse_lines("data/day_13_test.txt");
        prepare_input(lines)
    }

    #[test]
    fn test_prepare_input() {
        let (points, folds) = get_test_data();
        println!("{:?}", points);
        println!("{:?}", folds);
    }

    #[test]
    fn test_paper_new() {
        let (points, folds) = get_test_data();
        let paper = Paper::new(points, folds);
        let expected_layout = "...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
...........
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........
";

        // check dimensions
        assert_eq!(paper.points.len(), 15);
        assert_eq!(paper.points[0].len(), 11);

        // verify points are correct
        let s = format!("{}", paper);
        assert_eq!(expected_layout, s);
        let iter = paper.into_iter();
        // for f in iter {
        //     println!("{:?}", f);
        // }
    }

    #[test]
    fn test_fold_test_data() {
        let (points, folds) = get_test_data();
        let p = Paper::new(points, folds);
        let mut iter = p.into_iter();
        let points = iter.next().unwrap();
        let mut sum = 0;
        for line in points {
            sum += line.iter().filter(|p| **p).count();
        }

        assert_eq!(17, sum);
    }
}