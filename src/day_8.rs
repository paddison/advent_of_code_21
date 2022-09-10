use std::{collections::{HashSet, HashMap}, ops::Sub};

use crate::parse_lines;

#[derive(Debug, Eq, Hash)]
struct Digit {
    chars: String,
    len: usize,
    number: Option<u8>,
}

impl Sub for Digit {
    type Output = u64;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut chars = HashSet::new();
        let mut lhs_set = HashSet::new();
        self.chars.chars().for_each(| c | { lhs_set.insert(c); });
        let mut rhs_set = HashSet::new();
        rhs.chars.chars().for_each(| c | { rhs_set.insert(c); });
        
        for c in &rhs_set {
            if !lhs_set.contains(c) {
                chars.insert(*c);
            }
        }

        for c in &lhs_set {
            if !rhs_set.contains(c) {
                chars.insert(*c);
            }
        }

        chars.len() as u64
    }
}

impl Sub for &Digit {
    type Output = u64;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut chars = HashSet::new();
        let mut lhs_set = HashSet::new();
        self.chars.chars().for_each(| c | { lhs_set.insert(c); });
        let mut rhs_set = HashSet::new();
        rhs.chars.chars().for_each(| c | { rhs_set.insert(c); });
        
        for c in &rhs_set {
            if !lhs_set.contains(c) && !chars.contains(c){
                chars.insert(*c);
            }
        }

        for c in &lhs_set {
            if !rhs_set.contains(c) && !chars.contains(c) {
                chars.insert(*c);
            }
        }

        chars.len() as u64
    }
}

impl PartialEq for Digit {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }
        let mut count = 0;
        for self_char in self.chars.chars() {
            for other_char in other.chars.chars() {
                if self_char == other_char {
                    count += 1;
                }
            }
        }

        count == self.len
    }
}

impl From<String> for Digit {
    fn from(chars: String) -> Self {
        let number = match chars.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        };

        let len = chars.len();

        Digit { chars, len, number }
    }
}

impl From<&str> for Digit {
    fn from(chars: &str) -> Self {
        let number = match chars.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        };

        let len = chars.len();

        Digit { chars: chars.to_string(), len, number }
    }
}

pub fn get_solution_1() -> usize {
    let lines = parse_lines("data/day_8.txt");
    let digits: Vec<Vec<String>> = lines.into_iter().map(| l | {
        let split_line: Vec<String> = l.split('|').map(| s | s.to_string()).collect();
        split_line[1].split_whitespace().map(| s | s.to_string()).filter(| n | {
            let length = n.len();
            length == 2 || length == 3 || length == 4 || length == 7
        }).collect()
    }).collect();

    let mut count = 0;
    for line in digits {
        count += line.len();
    }
    count
}

pub fn get_solution_2() -> u32 {
    // idea:
    // for each digit, create a set out of its letters
    // now determine numbers in the following order n (number of letters):
    // 1 (2), 4(4), 7(3), 8(7) are clear since they are unique.
    // 3: (5) - 1 = (3),
    // 9: (6) - 3 = (1),
    // 6: (6) - 1 = (6),
    // 0: (6) - 3 = (3),
    // 5: (5) - 6 = (1)
    // the last remaining number in the set of sets will be 2

    // create a vector of sets containing each digits
    // create a map with key = number, and values = set
    // add numbers to set, by looping through the vector and testing if the above condition holds
    // for a given set size

    let lines = parse_lines("data/day_8.txt");
    let entries: Vec<Vec<String>> = lines.into_iter().map(|l| l.split('|').map(|n| n.to_string()).collect() ).collect();
    let mut sum = 0;
    for entry in entries {
        let (digits, number) = parse_to_digits(entry);
        let map = determine_numbers(digits);
        let result = calculate_result(map, number);
        sum += result;
    }
    sum
}

fn parse_to_digits(entries: Vec<String>) -> (Vec<Digit>, Vec<Digit>) {
    let mut digits = vec![];
    let mut number = vec![];
    for n in entries[0].split_whitespace(){
        digits.push(Digit::from(n));
    }

    for n in entries[1].split_whitespace() {
        number.push(Digit::from(n));
    }

    (digits, number)
}

fn determine_numbers(digits: Vec<Digit>) -> HashMap<i8, Digit> {
    let mut map: HashMap<i8, Digit> = HashMap::new();
    let mut undefined = vec![];
    for digit in digits {
        match digit.number {
            Some(n) => { map.insert(n as i8, digit); },
            None => { undefined.push(digit); },
        }
    }

    // determine 3
    determine_digit(&mut undefined, &mut map, 1, 3, 3, 5);
    
    // determine 9
    determine_digit(&mut undefined, &mut map, 3, 1, 9, 6);
    
    // determine 6
    determine_digit(&mut undefined, &mut map, 1, 6, 6, 6);
    
    // determine 0
    determine_digit(&mut undefined, &mut map, 3, 3, 0, 6);
    
    // determine 5
    determine_digit(&mut undefined, &mut map, 6, 1, 5, 5);
    
    // insert last digit(2)
    assert_eq!(undefined.len(), 1);
    let mut digit = undefined.remove(0);
    digit.number = Some(2);
    map.insert(2, digit);
    
    map
}

fn calculate_result(map: HashMap<i8, Digit>, number: Vec<Digit>) -> u32 {
    let mut result = 0;
    for (i, digit) in number.iter().rev().enumerate() {
        for (n, d) in &map {
            if digit == d {
                result += *n as u32 * 10_u32.pow(i as u32);
            }
        }
    }

    result as u32
}

fn determine_digit(undefined: &mut Vec<Digit>, map: &mut HashMap<i8, Digit>, comparator: i8, compare_result: u64, number: u8, char_length: usize) {
    let mut index = 0;
    for (i, digit) in undefined.iter().enumerate() {
        let comp = map.get(&comparator).unwrap();
        if  char_length == digit.len && (digit - comp) == compare_result {

            index = i;
            break;
        }
    }
    let mut digit = undefined.remove(index);
    digit.number = Some(number);
    map.insert(number as i8, digit);
}

#[test]
fn test_digit_sub() {
    let d1 = Digit { chars: "bacd".to_string(), len: 4, number: None };
    let d2 = Digit { chars: "bc".to_string(), len: 2, number: None};
    assert_eq!(2, d1 - d2);
}

#[test]
fn test_digit_sub_0_3() {
    let zero = Digit { chars: "cagedb".to_string(), len: 6, number: None };
    let three = Digit { chars: "fbcad".to_string(), len: 5, number: None };
    assert_eq!(3, three - zero);
}

#[test]
fn test_digit_sub_6_3() {
    let three = Digit { chars: "fbcad".to_string(), len: 5, number: None };
    let six = Digit { chars: "cdfgeb".to_string(), len: 6, number: None };
    assert_eq!(3, six - three);
}

#[test]
fn test_digit_sub_6_1() {
    let one = Digit { chars: "ab".to_string(), len: 2, number: None };
    let six = Digit { chars: "cdfgeb".to_string(), len: 6, number: None };
    assert_eq!(6, six - one);
}

#[test]
fn test_digit_sub_0_1() {
    let one = Digit { chars: "ab".to_string(), len: 2, number: None };
    let six = Digit { chars: "cagedb".to_string(), len: 6, number: None };
    assert_eq!(4, six - one);
}

#[test]
fn test_digit_sub_9_1() {
    let one = Digit { chars: "ab".to_string(), len: 2, number: None };
    let six = Digit { chars: "cefabd".to_string(), len: 6, number: None };
    assert_eq!(4, six - one);
}

#[test]
fn test_digit_from_string() {
    let d = Digit::from("bacd".to_string());

    assert_eq!(4, d.len);
    assert_eq!(Some(4), d.number);
}

#[test]
fn test_digit_partial_eq_true() {
    let d1 = Digit::from("bacd".to_string());
    let d2 = Digit::from("abcd".to_string());
    assert!(d1 == d2);
}

#[test]
fn test_digit_partial_eq_false() {
    let d1 = Digit::from("bacd".to_string());
    let d2 = Digit::from("abc".to_string());
    let d3 = Digit::from("abce".to_string());
    assert!(d1 != d2);
    assert!(d1 != d3);
    assert!(d2 != d3);
}