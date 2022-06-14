use std::{ops::Index, collections::HashSet};

use crate::parse_lines;

enum Edge {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    None,
}

enum Border {
    Top,
    Left,
    Right,
    Bottom,
    None,
}

#[derive(Debug, PartialEq)]
struct HeightMap<T: PartialOrd> {
    numbers: Vec<T>,
    width: usize,
}

impl<T: PartialOrd> Index<usize> for HeightMap<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.numbers[index]
    }
}

impl<T: PartialOrd> HeightMap<T> {
    // determine indices of low points
    fn determine_lows(&self) -> Vec<usize> {
        // case edge:
        // edges happen for i = 0, i = width - 1, i = total_size - width, i = total_size - 1;
        // case border: 
        //  top: for 0 < i < width - 1, 
        //  left: for i != 0 && i != total_size - width && i % width = 0
        //  right: for i != width - 1 && i != total_size - 1 && i % width = width - 1
        //  bottom: for total_size - width < i < total_size - 1
        // case default: if all other cases aren't true
    let mut lows = vec![];
    let width = self.width;
    let size = self.numbers.len();
    for (i, n) in self.numbers.iter().enumerate() {
        match determine_edge(i, width, size) {
            Edge::TopLeft => if self.get_right(i) > n && self.get_bottom(i) > n { lows.push(i); },
            Edge::TopRight => if self.get_left(i) > n && self.get_bottom(i) > n { lows.push(i); },
            Edge::BottomLeft => if self.get_right(i) > n && self.get_top(i) > n { lows.push(i); },
            Edge::BottomRight => if self.get_left(i) > n && self.get_top(i) > n { lows.push(i); },
            Edge::None => match determine_border(i, width, size) {
                Border::Top => if self.get_left(i) > n && self.get_right(i) > n && self.get_bottom(i) > n { lows.push(i) },
                Border::Left => if self.get_right(i) > n && self.get_top(i) > n && self.get_bottom(i) > n { lows.push(i) },
                Border::Right => if self.get_left(i) > n && self.get_top(i) > n && self.get_bottom(i) > n { lows.push(i) },
                Border::Bottom => if self.get_left(i) > n && self.get_right(i) > n && self.get_top(i) > n { lows.push(i) },
                Border::None => if self.get_left(i) > n && self.get_right(i) > n && self.get_bottom(i) > n  && self.get_top(i) > n {  lows.push(i) },
            }
        }
    }
    lows
    }
    
    #[inline(always)]
    fn get_top(&self, i: usize) -> &T {
        &self[i - self.width]
    }

    #[inline(always)]
    fn get_left(&self, i: usize) -> &T {
        &self[i - 1]
    }

    #[inline(always)]
    fn get_right(&self, i: usize) -> &T {
        &self[i + 1]
    }

    #[inline(always)]
    fn get_bottom(&self, i: usize) -> &T {
        &self[i + self.width]
    }

    fn get_unexplored_neighbours(&self, i: usize, explored: &HashSet<usize>) -> HashSet<usize> {

        let mut neighbours = HashSet::new();
        let top = i.wrapping_sub(self.width);
        let left = i.wrapping_sub(1);
        let right = i.wrapping_add(1);
        let bottom = i.wrapping_add(self.width);
        match determine_edge(i, self.width, self.numbers.len()) {
            Edge::TopLeft => {
                neighbours.insert(right);
                neighbours.insert(bottom);
            },
            Edge::TopRight => {
                neighbours.insert(left);
                neighbours.insert(bottom);
            },
            Edge::BottomLeft => {
                neighbours.insert(top);
                neighbours.insert(right);
            },
            Edge::BottomRight => {
                neighbours.insert(top);
                neighbours.insert(left);
            },
            Edge::None => match determine_border(i, self.width, self.numbers.len()) {
                Border::Top => {
                    neighbours.insert(left);
                    neighbours.insert(right);
                    neighbours.insert(bottom);
                },
                Border::Left => {
                    neighbours.insert(top);
                    neighbours.insert(right);
                    neighbours.insert(bottom);
                },
                Border::Right => {
                    neighbours.insert(top);
                    neighbours.insert(left);
                    neighbours.insert(bottom);
                },
                Border::Bottom => {
                    neighbours.insert(top);
                    neighbours.insert(left);
                    neighbours.insert(right);
                },
                Border::None => {                    
                    neighbours.insert(top);
                    neighbours.insert(left);
                    neighbours.insert(right);
                    neighbours.insert(bottom);
                },
            },
        }
        neighbours.difference(explored).map(|n| *n).collect()
    }

}

impl HeightMap<u8> {
    fn calculate_result(&self, lows: Vec<usize>) -> usize {
        let mut sum = 0;
        for low in lows {
            sum += self[low] as usize + 1;
        }
        sum
    }

    // builds a search frontier and adds neighbours to it until all neighbours have been explored
    fn determine_basin_size(&self, low: usize) -> usize {
        let mut explored = HashSet::new();
        let mut frontier = vec![low];
        while frontier.len() > 0 {
            let next = frontier.pop().unwrap(); // unwrap is safe, since for loop doesn't run if frontier is empty
            explored.insert(next);
            let neighbours = self.get_unexplored_neighbours(next, &explored);
            for n in neighbours {
                if self[n] != 9 {
                    frontier.push(n);
                }
            }
        }

        explored.len()
    }
}

// edge and border are called consequtively, so border doesnt need to check if it's not an edge
#[inline(always)]
fn determine_edge(i: usize, width: usize, size: usize) -> Edge {
    if i == 0 {
        Edge::TopLeft
    } else if i == width - 1 {
        Edge::TopRight
    } else if i == size - width {
        Edge::BottomLeft
    } else if i == size - 1 {
        Edge::BottomRight
    } else {
        Edge::None
    }
}
// should only be called if determine border has been called before
#[inline(always)]
fn determine_border(i: usize, width: usize, size: usize) -> Border {
    if 0 < i && i < width - 1 {
        Border::Top
    } else if i % width == 0 {
        Border::Left
    } else if i % width == width - 1 {
        Border::Right
    } else if size - width < i && i < size - 1 {
        Border::Bottom
    } else {
        Border::None 
    }
}

pub fn get_solution_1(is_test: bool) -> usize {
    let file_name = if is_test { "data/day_9_test.txt" } else { "data/day_9.txt" };
    let lines = parse_lines(file_name);
    let map = parse_numbers(lines);
    let lows = map.determine_lows();
    map.calculate_result(lows) 
}

pub fn get_solution_2(is_test: bool) -> usize {
    // build a search frontier:
    // two sets, one with unexplored neighbours, one with explored
    // write a get_neighbors function, which returns indices of all neighbors of a given node
    // a valid neighbor is, if it is not in explored, and it is not 9
    let file_name = if is_test { "data/day_9_test.txt" } else { "data/day_9.txt" };
    let lines = parse_lines(file_name);
    let map = parse_numbers(lines);
    let lows = map.determine_lows();
    let mut basins = vec![];
    for low in lows {
        basins.push(map.determine_basin_size(low));
    } 
    basins.sort_by(|a, b| b.cmp(a));
    basins[0] * basins[1] * basins[2]
}

fn parse_numbers(lines: Vec<String>) -> HeightMap<u8> {

    let mut numbers = vec![];
    let width = lines[0].len();
    for line in lines {
        for n in line.as_bytes() {
            numbers.push(*n - 48);
        }
    }
    HeightMap { numbers, width }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use crate::parse_lines;
    use super::{HeightMap, parse_numbers, get_solution_1, get_solution_2};

    #[test]
    fn test_parse_numbers() {
        let lines = parse_lines("data/day_9_test.txt");
        let numbers = parse_numbers(lines);
        assert_eq!(numbers, HeightMap { numbers: 
            vec![
                2,1,9,9,9,4,3,2,1,0,
                3,9,8,7,8,9,4,9,2,1,
                9,8,5,6,7,8,9,8,9,2,
                8,7,6,7,8,9,6,7,8,9,
                9,8,9,9,9,6,5,6,7,8
            ], 
            width: 10 }
        );
    }
    
    #[test]
    fn test_if_edge() {
        let map = HeightMap { numbers: 
            vec![
                2,1,9,9,9,4,3,2,1,0,
                3,9,8,7,8,9,4,9,2,1,
                9,8,5,6,7,8,9,8,9,2,
                8,7,6,7,8,9,6,7,8,7,
                9,8,9,9,9,6,5,6,7,8
            ], 
            width: 10 };
    
        assert_eq!(map[0], 2);
        assert_eq!(map[map.width - 1], 0);
        assert_eq!(map[map.numbers.len() - map.width], 9);
        assert_eq!(map[map.numbers.len() - 1], 8);
    }
    
    #[test]
    fn test_if_border() {
        let map = HeightMap { numbers: 
            vec![
                2,1,9,9,9,4,3,2,1,0,
                3,9,8,7,8,9,4,9,2,1,
                9,8,5,6,7,8,9,8,9,2,
                8,7,6,7,8,9,6,7,8,7,
                9,8,9,9,9,6,5,6,7,8
            ], 
            width: 10 };
        let mut tops = vec![];
        let mut lefts = vec![];
        let mut rights = vec![];
        let mut bottoms = vec![];
        for (i, val) in map.numbers.iter().enumerate() {
            if 0 < i && i < map.width - 1 as usize{
                tops.push(*val);
            } else if i != 0 && i != map.numbers.len() - map.width && i % map.width == 0 {
                lefts.push(*val);
            } else if i != map.width - 1 && i != map.numbers.len() - 1 && i % map.width == map.width - 1 {
                rights.push(*val);
            } else if map.numbers.len() - map.width < i && i < map.numbers.len() - 1 {
                bottoms.push(*val);
            }
        }
    
        assert_eq!(tops, vec![1, 9, 9, 9, 4, 3, 2, 1]);
        assert_eq!(lefts, vec![3, 9, 8]);
        assert_eq!(rights, vec![1, 2, 7]);
        assert_eq!(bottoms, vec![8, 9, 9, 9, 6, 5, 6, 7]);
    }
    
    #[test]
    fn test_get_elements() {
        let map = HeightMap { numbers: 
            vec![
                2,1,9,9,9,4,3,2,1,0,
                3,9,7,7,8,9,4,9,2,1,
                9,8,5,6,7,8,9,8,9,2,
                8,7,6,7,8,9,6,7,8,7,
                9,8,9,9,9,6,5,6,7,8
            ], 
            width: 10 }; 
        // get numbers around (1, 1) == 9
        let i = 11;
        assert_eq!(map.get_top(i), &1);
        assert_eq!(map.get_left(i), &3);
        assert_eq!(map.get_right(i), &7);
        assert_eq!(map.get_bottom(i), &8);
    }
    
    #[test]
    fn test_get_solution_1() {
        let result = get_solution_1(true);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_get_unexplored_neighbours() {
        let map = HeightMap { numbers: 
            vec![
                2,1,9,9,9,4,3,2,1,0,
                3,9,7,7,8,9,4,9,2,1,
                9,8,5,6,7,8,9,8,9,2,
                8,7,6,7,8,9,6,7,8,7,
                9,8,9,9,9,6,5,6,7,8
            ], 
            width: 10 }; 
        // test top
        let explored = HashSet::from([1]);
        let diff = map.get_unexplored_neighbours(2, &explored);
        assert_eq!(diff, HashSet::from([3, 12]));

        // test center
        let explored = HashSet::from([0, 1, 2, 21]);
        let diff = map.get_unexplored_neighbours(11, &explored);
        assert_eq!(diff, HashSet::from([10, 12]));
    }

    #[test]
    fn test_determine_basin_size() {
        let map: HeightMap<u8> = HeightMap { numbers: 
            vec![
                2,1,9,9,9,4,3,2,1,0,
                3,9,8,7,8,9,4,9,2,1,
                9,8,5,6,7,8,9,8,9,2,
                8,7,6,7,8,9,6,7,8,9,
                9,8,9,9,9,6,5,6,7,8
            ], 
            width: 10 };
        
        let size = map.determine_basin_size(1);
        assert_eq!(size, 3);
        
        let size = map.determine_basin_size(9);
        assert_eq!(size, 9);

        let size = map.determine_basin_size(22);
        assert_eq!(size, 14);
        
        let size = map.determine_basin_size(46);
        assert_eq!(size, 9);
    }

    #[test]
    fn test_get_solution_2() {
        let solution = get_solution_2(true);
        assert_eq!(solution, 1134);
    }
}

