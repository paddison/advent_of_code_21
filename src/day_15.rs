// create a search frontier. A node is a touple of x and y coordinates, which represent index of map
// a path contains visited nodes and neighbors, that have not been visited
// for each new node, create visited path
// path should store its current value
// A* algorithm for traversal
// store paths in array, if a path has no more neighbors remove it from path array

use crate::parse_lines;

const SCALE_FACTOR: usize = 5;

type Node = (u32, (usize, usize));

pub fn get_solution_1(is_test: bool) -> u32 {
    let file_name = match is_test {
        true => "data/day_15_test.txt",
        false => "data/day_15.txt",
    };
    let map: CaveMap = parse_lines(file_name).into();
    let mut path_frontier = CavePathFrontier::new(map);

    let result = loop {
        if let Some(path) = path_frontier.advance() {
            break path;
        }
    };

    return result.cost
}

pub fn get_solution_2(is_test: bool) -> u32 {
    let file_name = match is_test {
        true => "data/day_15_test.txt",
        false => "data/day_15.txt",
    };
    let map: CaveMap = parse_lines(file_name).into();
    let map = map.enlarge_map(SCALE_FACTOR);
    let mut path_frontier = CavePathFrontier::new(map);

    let result = loop {
        if let Some(path) = path_frontier.advance() {
            break path;
        }
    };

    return result.cost
}


#[derive(PartialEq, Eq)]
struct CaveMap {
    vals: Vec<Option<u32>>,
    dim: (usize, usize),
}

impl CaveMap {

    fn get(&self, (x, y): (usize, usize)) -> Option<&u32> {
        
        if let Some(Some(val)) = self.vals.get(x * self.dim.0 + y) {
            Some(val)
        } else {
            None
        }
    }

    fn get_mut(&mut self, (x, y): (usize, usize)) -> Option<&mut u32> {
        if let Some(Some(val)) =  self.vals.get_mut(x * self.dim.0 + y) {
            Some(val)
        } else {
            None
        }
    }

    fn set_to_visited(&mut self, (x, y): (usize, usize)) {
        self.vals[x * self.dim.0 + y] = None;
    }

    fn enlarge_map(&self, scale_factor: usize) -> Self {
        // transfrom map.vals to normal numbers:
        let base_block: Vec<u32> = self.vals.iter().map(|val| val.unwrap()).collect();

        let mut new_vals = vec![];

        for i in 0..scale_factor {
            for y in 0..self.dim.1 {
                for j in 0..scale_factor {
                    for x in 0..self.dim.0 {
                        let new_val = (base_block[y * self.dim.0 + x] + i as u32 + j as u32 - 1) % 9 + 1;
                        new_vals.push(new_val);
                    }
                }
            }
        }

        let vals = new_vals.into_iter().map(|val| Some(val)).collect();

        CaveMap { vals, dim: (self.dim.0 * SCALE_FACTOR, self.dim.1 * SCALE_FACTOR) }
    }
}

impl From<Vec<String>> for CaveMap {
    fn from(input: Vec<String>) -> Self {
        let dim = (input[0].len(), input.len());
        let mut vals = vec![];
        for line in input {
            for c in line.chars() {
                vals.push(Some(c.to_digit(10).unwrap()));
            }
        }

        CaveMap { vals, dim }
    }
}

#[derive(PartialEq, Debug)]
struct CavePath {
    pos: (usize, usize),
    cost: u32,
}

impl CavePath {
    fn copy_update(&self, n: Node) -> Self {
        CavePath { pos: n.1, cost: n.0 + self.cost}
    }
}

struct CavePathFrontier {
    paths: Vec<CavePath>,
    map: CaveMap,
    exit: (usize, usize)
}

impl CavePathFrontier {
    fn new(map: CaveMap) -> Self{
        let exit = (map.dim.0 - 1, map.dim.1 - 1);
        let paths = vec![CavePath { pos: (0, 0), cost: 0 }];
        CavePathFrontier { paths, map, exit }
    }

    fn is_complete(&self, path: &CavePath) -> bool {
        self.exit == path.pos
    }

    fn get_neighbors(&self, path: &CavePath) -> Vec<Node> {
        let mut neighbors = vec![];
        let pos = path.pos;

        // calculate new positions
        if pos.1 > 0 {
            let upper = (pos.0, pos.1 - 1); // overflow gets checked via get function
            if let Some(val) = self.map.get(upper) { neighbors.push((*val, upper)) };
        }
        
        if pos.0 > 0 {
            let left = (pos.0 - 1 , pos.1);
            if let Some(val) = self.map.get(left) { neighbors.push((*val, left)) };
        }

        let right = (pos.0 + 1, pos.1);
        if let Some(val) = self.map.get(right) { neighbors.push((*val, right)) };
        
        let lower = (pos.0, pos.1 + 1);
        if let Some(val) = self.map.get(lower) { neighbors.push((*val, lower)) };

        neighbors
    }

    fn heuristic(&self, path: &CavePath) -> f64 {
        let a = path.pos.0 as f64;
        let b = path.pos.1 as f64;
        let dist = f64::sqrt(a * a + b * b);

        path.cost as f64 + dist

    }

    fn advance(&mut self) -> Option<CavePath> {

        let mut candidates = vec![];

        for path in &self.paths {
            let mut neighbors: Vec<CavePath> = vec![];
            
            for neighbor in self.get_neighbors(&path) {
                neighbors.push(path.copy_update(neighbor));
            }

            let cmp_heuristic = |lhs: &CavePath, rhs: &CavePath| self.heuristic(lhs).total_cmp(&self.heuristic(rhs));
            // determine cheapest neighbor
            if let Some(min) = neighbors
                                .into_iter()
                                .min_by(cmp_heuristic) {
                candidates.push(min);
            };
        }

        // determine cheapest path
        let min = candidates.into_iter().min_by(|lhs, rhs| lhs.cost.cmp(&rhs.cost)).unwrap();
        if self.is_complete(&min) {
            Some(min)
        } else {
            self.map.set_to_visited(min.pos);
            self.paths.push(min);
            None
        }
    } 
}

#[cfg(test)]
mod tests {
    use crate::{day_15::{CaveMap, CavePath}, parse_lines};

    use super::{CavePathFrontier, SCALE_FACTOR};

    fn create_test_data() -> Vec<String> {
        vec!["12345".to_string(), "54321".to_string(), "31245".to_string()]
    }

    fn create_mini_test_data() -> CaveMap {
        vec!["12".to_string(), "34".to_string()].into()
    }

    fn create_online_test_data() -> CaveMap {
        parse_lines("data/day_15_test.txt").into()
    }

    #[test]
    fn test_create_map() {
        let map: CaveMap = create_test_data().into();
        let expected_vals: Vec<Option<u32>> = vec![
            1, 2, 3, 4, 5, 
            5, 4, 3, 2, 1, 
            3, 1, 2, 4, 5].into_iter().map(|x| Some(x)).collect();
        
        assert_eq!(map.dim, (5, 3));
        assert_eq!(map.vals, expected_vals);
    }

    #[test]
    fn test_frontier_new() {
        let map: CaveMap = create_test_data().into();
        let frontier = CavePathFrontier::new(map);

        assert_eq!(frontier.paths[0], CavePath { cost: 0, pos: (0, 0) });
        assert_eq!(frontier.exit, (4, 2));
    }

    #[test]
    fn test_frontier_advance() {
        let mut frontier = CavePathFrontier::new(create_online_test_data());
        let result = frontier.advance();
        assert!(result.is_none());
        // should go left first
        assert!(frontier.map.get((1, 0)).is_none());

        let result = frontier.advance();
        assert!(result.is_none());
        // should go left first
        assert!(frontier.map.get((0, 1)).is_none());
    }

    #[test]
    fn test_frontier_advance_result() {
        let mut frontier = CavePathFrontier::new(create_online_test_data());
        
        let result = loop {
            if let Some(path) = frontier.advance() {
                break path;
            }
        };
    
        assert_eq!(result.cost, 40);
    }

    #[test]
    fn test_enlarge_map() {

        let map = create_mini_test_data();
        let actual = map.enlarge_map(3);
        let expected: Vec<Option<u32>> = vec![
            1, 2, 2, 3, 3, 4, 
            3, 4, 4, 5, 5, 6, 
            2, 3, 3, 4, 4, 5, 
            4, 5, 5, 6, 6, 7, 
            3, 4, 4, 5, 5, 6, 
            5, 6, 6, 7, 7, 8].into_iter().map(|x| Some(x)).collect();

        assert_eq!(actual.vals, expected);
    }

    #[test]
    fn test_enlarge_map_result() {
        let map = create_online_test_data();
        let large_map = map.enlarge_map(SCALE_FACTOR);

        let mut frontier = CavePathFrontier::new(large_map);

        let result = loop {
            if let Some(path) = frontier.advance() {
                break path;
            }
        };
    
        assert_eq!(result.cost, 315);
    }
}

