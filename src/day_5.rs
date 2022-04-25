use std::collections::HashMap;

use crate::parse_lines;

struct Line {
    start: (u32, u32),
    end: (u32, u32),
}

struct LineIter {
    line: Line,
    is_horizontal: bool,
    pos: u32,
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

    fn print_map(map: &HashMap<(u32, u32), u32>, dim: u32) {
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

impl IntoIterator for Line {
    type Item = (u32, u32);

    type IntoIter = LineIter;

    fn into_iter(self) -> Self::IntoIter {
        let is_horizontal = self.is_horizontal();
        LineIter { line: self, is_horizontal, pos: 0 }
    }
}

impl Iterator for LineIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.is_horizontal {
            true => {
                if self.line.start.0 > self.line.end.0 {
                    // descending line
                    if self.line.start.0 - self.pos + 1 <= self.line.end.0 {
                        None
                    } else {
                        Some((self.line.start.0 - self.pos, self.line.start.1))
                    }
                } else {
                    // ascending line
                    if self.line.start.0 + self.pos >= self.line.end.0 + 1 {
                        None
                    } else {
                        Some((self.line.start.0 + self.pos, self.line.start.1))
                    }
                }
            },
            false => {
                if self.line.start.1 > self.line.end.1 {
                    // descending line
                    if self.line.start.1 - self.pos + 1 <= self.line.end.1 {
                        None
                    } else {
                        Some((self.line.start.0, self.line.start.1 - self.pos))
                    }
                } else {
                    // ascending line
                    if self.line.start.1 + self.pos >= self.line.end.1 + 1 {
                        None
                    } else {
                        Some((self.line.start.0, self.line.start.1 + self.pos))
                    }
                }
            },
        };
        self.pos += 1;
        next
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

pub fn solve_5_1(file_name: &str) -> u32 {
    let raw_lines = parse_lines(file_name);
    let lines: Vec<Line> = raw_lines.into_iter()
        .map(|line| Line::from(line))
        .filter(|line| !(line.start == (0, 0) && line.end == (0, 0)))
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
fn test_create_map() {
    let raw_lines = parse_lines("data/day_5_test.txt");
    let lines: Vec<Line> = raw_lines.into_iter()
        .map(|line| Line::from(line))
        .filter(|line| line.is_horizontal() && line.is_vertical())
        .collect();
    
    let map = create_map(lines);
    let mut count = 0;
    Line::print_map(&map, 10);
    for (_, val) in map {
        if val >= 2 { count += 1 };
    }
    assert_eq!(count, 5);
}