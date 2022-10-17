use std::collections::HashMap;

use crate::parse_lines;

struct Line {
    start: (u32, u32),
    end: (u32, u32),
}

impl Line {
    fn new(start: (u32, u32), end: (u32, u32)) -> Self {
        Line { start, end }
    }

    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn _print_map(map: &HashMap<(u32, u32), u32>, dim: u32) {
        for i in 0..dim {
            let mut row = String::new();
            for j in 0..dim {
                let val = match map.get(&(j, i)) {
                    Some(n) => n.to_string(),
                    None => String::from("."),
                };
                row += &val;
            }
            println!("{}", row);
        }
    }
}

impl From<String> for Line {
    fn from(line: String) -> Self {
        let touples: Vec<(u32, u32)> = line.split("->")
            .map(|touple| {
            let n: Vec<u32> = touple.split(",").map(|n| n.trim().parse::<u32>().unwrap()).collect();
            (n[0], n[1])
            })
            .collect();
        Line::new(touples[0], touples[1])
    }
}

impl From<&str> for Line {
    fn from(line: &str) -> Self {
        let touples: Vec<(u32, u32)> = line.split("->")
        .map(|touple| {
        let n: Vec<u32> = touple.split(",").map(|n| n.trim().parse::<u32>().unwrap()).collect();
        (n[0], n[1])
        })
        .collect();
    Line::new(touples[0], touples[1])
    }
}

impl IntoIterator for Line {
    type Item = (u32, u32);

    type IntoIter = Box<dyn Iterator<Item = (u32, u32)>>;

    fn into_iter(self) -> Self::IntoIter {
        if self.is_horizontal() || self.is_vertical() {
            let is_horizontal = self.is_horizontal();
            Box::new(LineIter { line: self, is_horizontal, depleted: false })
        } else {
            Box::new(LineIterDiag { line: self, depleted: false})
        }
    }
}

struct LineIter {
    line: Line,
    is_horizontal: bool,
    depleted: bool,
}

impl Iterator for LineIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.depleted {
            if self.line.start == self.line.end {
                self.depleted = true;
                Some((self.line.start.0, self.line.start.1))
            } else {
                let next = Some((self.line.start.0, self.line.start.1));
                if self.is_horizontal {
                    if self.line.start.0 > self.line.end.0 {
                        // left
                        self.line.start.0 -= 1;
                    } else {
                        // right
                        self.line.start.0 += 1;
                    }
                } else {
                    if self.line.start.1 > self.line.end.1 {
                        // down
                        self.line.start.1 -= 1;
                    } else {
                        // up
                        self.line.start.1 += 1;
                    }
                }
                next
            }
        } else {
            None
        }
        
    }
}

struct LineIterDiag {
    line: Line,
    depleted: bool,
}

impl Iterator for LineIterDiag {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.depleted {
            if self.line.start == self.line.end {
                self.depleted = true;
                Some((self.line.start.0, self.line.start.1))
            } else {
                let next = Some((self.line.start.0, self.line.start.1));
                if self.line.start.0 < self.line.end.0 {
                    self.line.start.0 += 1;
                } else {
                    self.line.start.0 -= 1;
                }
    
                if self.line.start.1 < self.line.end.1 {
                    self.line.start.1 += 1;
                } else {
                    self.line.start.1 -= 1;
                }
                next
            }
        } else {
            None
        }
    }
}

fn create_map(lines: Vec<Line>) -> HashMap<(u32, u32), u32> {
    let mut map: HashMap<(u32, u32), u32> = HashMap::new();
    for line in lines {
        // create the map by walking a line and storing the entries in a hash map
        for (x, y) in line {
            let count = map.entry((x, y)).or_insert(0);
            *count += 1;
        }
    }

    map
}

pub fn get_solution_1() -> u32 {
    let raw_lines = parse_lines("data/day_5.txt");
    let lines: Vec<Line> = raw_lines.into_iter()
        .map(
            Line::from)
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .collect();

    let map = create_map(lines);
    let mut count = 0;
    for (_, val) in map {
        if val >= 2 { count += 1 };
    }
    count
}

pub fn get_solution_2() -> u32 {
    let raw_lines = parse_lines("data/day_5.txt");
    let lines: Vec<Line> = raw_lines.into_iter()
        .map(Line::from)
        .collect();

    let map = create_map(lines);
    let mut count = 0;
    for (_, val) in map {
        if val >= 2 { count += 1 };
    }
    count
}


#[test]
fn test_vertical_horizontal() {
    let horizontal = Line::new((0, 0), (5, 0));
    assert!(horizontal.is_horizontal());
    assert!(!horizontal.is_vertical());

    let vertical = Line::new((0, 0), (0, 5));
    assert!(vertical.is_vertical());
    assert!(!vertical.is_horizontal());
}

#[test]
fn test_from_string() {
    let line = Line::from(String::from("7,0 -> 7,4"));
    assert!(line.is_vertical());
}

#[test]
fn test_iterator_horizontal_asc() {
    let mut line = Line::from(String::from("1,5 -> 4,5")).into_iter();
    assert_eq!(line.next(), Some((1, 5)));
    assert_eq!(line.next(), Some((2, 5)));
    assert_eq!(line.next(), Some((3, 5)));
    assert_eq!(line.next(), Some((4, 5)));
    assert_eq!(line.next(), None);
}

#[test]
fn test_iterator_horizontal_desc() {
    let mut line = Line::from(String::from("3,4 -> 1,4")).into_iter();
    assert_eq!(line.next(), Some((3, 4)));
    assert_eq!(line.next(), Some((2, 4)));
    assert_eq!(line.next(), Some((1, 4)));
    assert_eq!(line.next(), None);
}

#[test]
fn test_iterator_vertical_asc() {
    let mut line = Line::from(String::from("7,0 -> 7,4")).into_iter();
    assert_eq!(line.next(), Some((7, 0)));
    assert_eq!(line.next(), Some((7, 1)));
    assert_eq!(line.next(), Some((7, 2)));
    assert_eq!(line.next(), Some((7, 3)));
    assert_eq!(line.next(), Some((7, 4)));
    assert_eq!(line.next(), None);
}

#[test]
fn test_iterator_vertical_desc() {
    let mut line = Line::from(String::from("7,4 -> 7,0")).into_iter();
    assert_eq!(line.next(), Some((7, 4)));
    assert_eq!(line.next(), Some((7, 3)));
    assert_eq!(line.next(), Some((7, 2)));
    assert_eq!(line.next(), Some((7, 1)));
    assert_eq!(line.next(), Some((7, 0)));
    assert_eq!(line.next(), None);
}

#[test]
fn test_iterator_diag_right_down() {
    let line = Line::from("0,1 -> 3,4");
    let mut iter = line.into_iter();
    assert_eq!(Some((0, 1)), iter.next());
    assert_eq!(Some((1, 2)), iter.next());
    assert_eq!(Some((2, 3)), iter.next());
    assert_eq!(Some((3, 4)), iter.next());
    assert_eq!(None, iter.next());
}

#[test]
fn test_iterator_diag_right_up() {
    let line = Line::from("0,4 -> 3,1");
    let mut iter = line.into_iter();
    assert_eq!(Some((0, 4)), iter.next());
    assert_eq!(Some((1, 3)), iter.next());
    assert_eq!(Some((2, 2)), iter.next());
    assert_eq!(Some((3, 1)), iter.next());
    assert_eq!(None, iter.next());
}

#[test]
fn test_iterator_diag_left_down() {
    let line = Line::from("3,1 -> 0,4");
    let mut iter = line.into_iter();
    assert_eq!(Some((3, 1)), iter.next());
    assert_eq!(Some((2, 2)), iter.next());
    assert_eq!(Some((1, 3)), iter.next());
    assert_eq!(Some((0, 4)), iter.next());
    assert_eq!(None, iter.next());
}

#[test]
fn test_iterator_diag_left_up() {
    let line = Line::from("3,4 -> 0,1");
    let mut iter = line.into_iter();
    assert_eq!(Some((3, 4)), iter.next());
    assert_eq!(Some((2, 3)), iter.next());
    assert_eq!(Some((1, 2)), iter.next());
    assert_eq!(Some((0, 1)), iter.next());
    assert_eq!(None, iter.next());
}


#[test]
fn test_create_map() {
    let raw_lines = parse_lines("data/day_5_test.txt");
    let lines: Vec<Line> = raw_lines.into_iter()
        .map(|line| Line::from(line))
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .collect();
    
    let map = create_map(lines);
    let mut count = 0;
    Line::_print_map(&map, 10);
    for (_, val) in map {
        if val >= 2 { count += 1 };
    }
    assert_eq!(count, 5);
}

#[test]
fn test_create_map_all() {
    let raw_lines = parse_lines("data/day_5_test.txt");
    let lines: Vec<Line> = raw_lines.into_iter()
        .map(|line| Line::from(line))
        .collect();
    
    let map = create_map(lines);
    let mut count = 0;
    Line::_print_map(&map, 10);
    for (_, val) in map {
        if val >= 2 { count += 1 };
    }
    assert_eq!(count, 12);
}