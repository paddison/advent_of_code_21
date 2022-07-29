// create a search frontier. A node is a touple of x and y coordinates, which represent index of map
// a path contains visited nodes and neighbors, that have not been visited
// for each new node, create visited path
// path should store its current value
// A* algorithm for traversal
// store paths in array, if a path has no more neighbors remove it from path array

use std::{collections::HashSet, ops::Index};

use crate::parse_lines;

pub fn get_solution_1(is_test: bool) {
    let file_name = match is_test {
        true => "data/day_15_test.txt",
        false => "data/day_15.txt",
    };
    let lines = parse_lines(file_name);
    let map = create_map(lines);
}

#[derive(PartialEq, Eq)]
struct Map {
    points: Vec<Vec<usize>>,
}

impl Index<Node> for Map {
    type Output = usize;

    fn index(&self, index: Node) -> &Self::Output {
        &self.points[index.points.0][index.points.1]
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Node {
    points: (usize, usize),
    cost: usize,
}

impl Node {
    fn new(map: &Map, coords) -> Self {
        // calculate cost of node with A*
        let cost = Map
    }
}

struct Path<'a> {
    visited: HashSet<Node>,
    neighbors: Vec<Node>,
    cost: usize,
    map: &'a [Vec<usize>]
}

fn create_map(lines: Vec<String>) -> Map {
    let mut points = vec![];
    for line in lines {
        let numbers: Vec<usize> = line
                                    .chars()
                                    .map(|c| c.to_digit(10))
                                    .filter(|d| d.is_some())
                                    .map(|d| d.unwrap() as usize)
                                    .collect();
        points.push(numbers);
    }

    Map { points }
}

#[cfg(test)]
mod tests {
    use crate::{parse_lines, day_15::{create_map, Map}};

    const TEST_MAP: [[usize; 10]; 10] = [
        [1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
        [1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
        [2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
        [3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
        [7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
        [1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
        [1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
        [3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
        [1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
        [2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
    ];

    #[test]
    fn test_create_map() {
        let points = TEST_MAP.iter().map(|l| l.to_vec()).collect();
        let expected = Map { points };
        let lines = parse_lines("data/day_15_test.txt");
        let map = create_map(lines);
        assert_eq!(map, expected);
    }
}

