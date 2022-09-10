use std::{ops::{Deref, DerefMut, Add}, fmt::Display};
use std::fmt::Write as _;

pub fn get_solution_1() -> u32 {
    let mut lines = include_str!("../data/day_18.txt").lines();
    let mut sn: SnailNumber = lines.next().unwrap().into();

    for next in lines {
        sn += next.into();
    }

    sn.magnitude()
}

pub fn get_solution_2() -> u32 {
    let numbers = include_str!("../data/day_18.txt").lines().map(|line| line.into()).collect::<Vec<SnailNumber>>();
    let mut max = 0;
    for n_outer in &numbers {
        for n_inner in &numbers {
            if n_outer != n_inner {
                let n = n_inner + n_outer;
                let mag = n.magnitude();
                if mag > max {
                    max = mag;
                }
            }
        }
    }

    max
}

#[derive(Debug)]
struct SnailNumber {
    value: Vec<Number>
}

impl SnailNumber {

    fn new() -> Self {
        SnailNumber { value: vec![] }
    }

    fn reduce(&mut self) -> bool {
        for (i, n) in self.iter_mut().enumerate() {
            if n.level > 4 {
                self.explode(i);
                return false;
            } 
        }

        for (i, n) in self.iter_mut().enumerate() {
            if n.value >= 10 {
                self.split(i);
                return false;
            }
        }

        true
    }

    /*
    * index change while removing:    
    * [2, [[[1, 2], 3], 5]]
    * i = 1
    * remove left number and swap right with zero, and level - 1:
    * [2, [[0, 3], 5]], left = 1, right = 2
    * add left to i - 1
    * add right to i + 1
    * [3, [[0, 5], 5]]
    */
    fn explode(&mut self, i: usize) {

        // index should never be last element
        assert!(i < self.value.len() - 1);
        // get values of left and right regular number
        let left = self.remove(i).value;
        let right = self[i].value;
        
        // swap right number with zero and reduce level
        self[i].level -= 1;
        self[i].value = 0;

        // add regular number to regular number to the right
        if i < self.value.len() - 1 {
            self[i + 1].value += right;
        }

        // add regular number to regular number to the left
        if i > 0 {
            self[i - 1].value += left;
        }
    }

    fn split(&mut self, i: usize) {

        let left = Number { 
            value: self[i].value / 2,
            level: self[i].level + 1
        };

        let right = Number {
            value: if self[i].value % 2 == 1 { self[i].value / 2 + 1} else { self[i].value / 2 },
            level: self[i].level + 1
        };

        self[i] = left;
        self.insert(i + 1, right);
    }

    fn magnitude(mut self) -> u32 {
        while self.len() > 1 {
            // build a new number and assign it to the old one
            let mut i = 0;
            let mut sn = SnailNumber::new();
            let mut found_pair = false;
            while i < self.len() {
                let n = if !found_pair && i < self.len() - 1 && self[i].level == self[i + 1].level {
                    let n = Number { 
                        value: self[i].value * 3 + self[i + 1].value * 2,
                        level: self[i].level - 1,
                    };
                    i += 2;
                    found_pair = true;
                    n
                } else {
                    let n = self[i];
                    i += 1;
                    n
                };
                sn.push(n)
            }

            self = sn;
        }

        self[0].value
    }
}

impl Display for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut _level = 0;
        let mut string = String::new();
        for n in self.iter() {
            let _ = write!(string, "{:<2},", n.value);
        }
        string += "\n";
        for n in self.iter() {
            let _ = write!(string, "{:<2},", n.level);
        }

        write!(f, "{}\n", string)
    }
}

impl Add for &SnailNumber {
    type Output = SnailNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut value = vec![];
        value.extend(self.value.clone());
        value.extend(rhs.value.clone());

        let mut sn = SnailNumber { value };

        for n in sn.value.iter_mut() {
            n.level += 1;
        }
        
        while !sn.reduce() {}

        sn
    }
}

impl std::ops::AddAssign for SnailNumber {
    fn add_assign(&mut self, other: Self) {
        self.value.extend(other.value.into_iter());
        for n in self.value.iter_mut() {
            n.level += 1;
        }
        while !self.reduce() {}
    }
}

/// Works only on reduced snail numbers
impl From<&str> for SnailNumber {
    fn from(input: &str) -> Self {
        let mut value = Vec::new();
        let mut level = 0;
        for c in input.chars() {
            match c {
                '[' => level += 1,
                ']' => level -= 1,
                c => if let Some(n) = c.to_digit(10) {
                    value.push(Number { value: n, level })
                },
            }
        }
        SnailNumber{ value }
    }
}

impl Deref for SnailNumber {
    type Target = Vec<Number>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for SnailNumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl PartialEq for SnailNumber {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[derive(Debug, Clone, Copy)]
struct Number {
    value: u32,
    level: u8,
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.level == other.level
    }
}

#[cfg(test)]
pub mod tests {
    use crate::day_18::SnailNumber;

    #[test]
    fn test_snail_number_from_str_ref() {
        let input = "[[1, 2], 3]";
        let sn: SnailNumber = input.into();
        assert_eq!(sn[0].level, 2);
        assert_eq!(sn[1].level, 2);
        assert_eq!(sn[2].level, 1);

        let input = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";
        let sn: SnailNumber = input.into();
        assert_eq!(sn[0].level, 4);
        assert_eq!(sn[1].level, 4);
        assert_eq!(sn[2].level, 4);
        assert_eq!(sn[3].level, 4);
        assert_eq!(sn[4].level, 4);
        assert_eq!(sn[5].level, 4);
        assert_eq!(sn[6].level, 4);
        assert_eq!(sn[7].level, 4);
        assert_eq!(sn[8].level, 1);
    }

    #[test]
    fn test_snail_number_add_assign() {
        let mut sn: SnailNumber = "[1, 2]".into();
        let other: SnailNumber = "[3, 4]".into();

        sn += other;
        assert_eq!(sn[0].level, 2);
        assert_eq!(sn[1].level, 2);
        assert_eq!(sn[2].level, 2);
        assert_eq!(sn[3].level, 2);

        let mut sn: SnailNumber = "[[[[4,3],4],4],[7,[[8,4],9]]]".into();
        let other: SnailNumber = "[1, 1]".into();
        sn += other;

        assert_eq!(sn[0].level, 4);
        assert_eq!(sn[0].value, 0);

        assert_eq!(sn[1].level, 4);
        assert_eq!(sn[1].value, 7);

        assert_eq!(sn[2].level, 3);
        assert_eq!(sn[2].value, 4);

        assert_eq!(sn[3].level, 4);
        assert_eq!(sn[3].value, 7);

        assert_eq!(sn[4].level, 4);
        assert_eq!(sn[4].value, 8);

        assert_eq!(sn[5].level, 4);
        assert_eq!(sn[5].value, 6);

        assert_eq!(sn[6].level, 4);
        assert_eq!(sn[6].value, 0);

        assert_eq!(sn[7].level, 2);
        assert_eq!(sn[7].value, 8);

        assert_eq!(sn[8].level, 2);
        assert_eq!(sn[8].value, 1);
    }

    #[test]
    fn test_snail_number_add_assign_several() {
        let mut sn: SnailNumber = "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]".into();

        sn += "[2, 9]".into();

        println!("{}", sn);
//         6 ,6 ,7 ,7 ,8 ,0 ,8 ,8 ,6 ,6 ,6 ,7 ,9 ,
//         4 ,4 ,4 ,4 ,4 ,4 ,4 ,4 ,4 ,4 ,4 ,4 ,2 ,
    }

    #[test]
    fn test_snail_number_explode() {
        let mut sn: SnailNumber = "[[[[[9,8],1],2],3],4]".into();

        sn.explode(0);

        assert_eq!(sn[0].level, 4);
        assert_eq!(sn[0].value, 0);
        assert_eq!(sn[1].level, 4);
        assert_eq!(sn[1].value, 9);

        let mut sn: SnailNumber = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".into();

        sn.explode(3);
        sn.explode(7);

        assert_eq!(sn[2].level, 4);
        assert_eq!(sn[2].value, 8);
        assert_eq!(sn[3].level, 4);
        assert_eq!(sn[3].value, 0);
        assert_eq!(sn[4].level, 2);
        assert_eq!(sn[4].value, 9);
        assert_eq!(sn[6].level, 4);
        assert_eq!(sn[6].value, 7);
        assert_eq!(sn[7].level, 4);
        assert_eq!(sn[7].value, 0);

    }

    #[test]
    fn test_snail_number_split() {
        let mut sn: SnailNumber = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".into();

        sn.explode(0);
        sn.explode(4);

        sn.split(3);

        assert_eq!(sn[3].level, 4);
        assert_eq!(sn[3].value, 7);
        assert_eq!(sn[4].level, 4);
        assert_eq!(sn[4].value, 8);

        sn.split(6);

        assert_eq!(sn[6].level, 5);
        assert_eq!(sn[6].value, 6);
        assert_eq!(sn[7].level, 5);
        assert_eq!(sn[7].value, 7);

    }

    #[test]
    fn test_snail_number_reduce() {
        let mut sn: SnailNumber = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".into();

        sn.reduce();

        assert_eq!(sn[0].level, 4);
        assert_eq!(sn[0].value, 0);

        assert_eq!(sn[1].level, 4);
        assert_eq!(sn[1].value, 7);

        assert_eq!(sn[2].level, 3);
        assert_eq!(sn[2].value, 4);

        assert_eq!(sn[3].level, 4);
        assert_eq!(sn[3].value, 7);

        assert_eq!(sn[4].level, 4);
        assert_eq!(sn[4].value, 8);

        assert_eq!(sn[5].level, 4);
        assert_eq!(sn[5].value, 6);

        assert_eq!(sn[6].level, 4);
        assert_eq!(sn[6].value, 0);

        assert_eq!(sn[7].level, 2);
        assert_eq!(sn[7].value, 8);

        assert_eq!(sn[8].level, 2);
        assert_eq!(sn[8].value, 1);

    }

    #[test]
    fn test_snail_number_magnitude() {
        let sn: SnailNumber = "[[1,2],[[3,4],5]]".into();
        assert_eq!(sn.magnitude(), 143);

        let sn: SnailNumber = "[[[[3,0],[5,3]],[4,4]],[5,5]]".into();
        assert_eq!(sn.magnitude(), 791);

        let sn: SnailNumber = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".into();
        assert_eq!(sn.magnitude(), 3488);

        let sn: SnailNumber = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".into();
        assert_eq!(sn.magnitude(), 4140);
    }

    #[test]
    fn test_with_test_data() {
        let mut lines = include_str!("../data/day_18_test.txt").lines();
        let mut sn: SnailNumber = lines.next().unwrap().into();

        while let Some(next) = lines.next() {
            sn += next.into();
        }

        assert_eq!(sn.magnitude(), 4140);
    }
}

mod binary_tree_implementation_not_finished {
    use std::ops::Add;

    struct _SnailNumber {
        root: NumberType,
    }
    
    #[derive(Debug)]
    enum NumberType {
        Regular(u32),
        Pair(Box<(NumberType, NumberType)>)
    }
    
    impl NumberType {
        fn parse_str(mut i: usize, input: &str) -> (Self, usize) {
            let mut cur_pair = (NumberType::Regular(0), NumberType::Regular(0));
            while i < input.len() {
                match &input[i..i + 1] {
                    "[" => (cur_pair.0, i) = Self::parse_str(i + 1, input),
                    "," => (cur_pair.1, i) = Self::parse_str(i + 1, input),
                    "]" => return (NumberType::Pair(Box::new(cur_pair)), i + 1),
                    n => {
                        // check if n is two characters long (useful for debugging splits)
                        if let Some(m) =  input.get(i + 1..i + 2) {
                            if m.chars().next().unwrap().is_numeric() { 
                                return (NumberType::Regular(input[i..i + 2].parse::<u32>().unwrap()), i + 2) 
                            }
                        }
                        return (NumberType::Regular(n.parse::<u32>().unwrap()), i + 1)
                    }
                }
            }
            unreachable!()
        }
    }
    
    impl Add<NumberType> for NumberType {
    
        type Output = NumberType;
    
        fn add(self, rhs: NumberType) -> Self::Output {
            (self, rhs).into()
        }
    }
    
    impl From<&str> for NumberType {
        fn from(input: &str) -> Self {
            Self::parse_str(0, input).0
        }
    }
    
    impl From<u32> for NumberType {
        fn from(val: u32) -> Self {
            NumberType::Regular(val)
        }
    }
    
    impl From<(u32, u32)> for NumberType {
        fn from((l, r): (u32, u32)) -> Self {
            NumberType::Pair(Box::new((l.into(), r.into())))
            
        }
    }
    
    impl From<(NumberType, NumberType)> for NumberType {
        fn from(other: (NumberType, NumberType)) -> Self {
            Self::Pair(Box::new(other))
        }
    }
    
    
    impl PartialEq for NumberType {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Regular(n), Self::Regular(m)) => n == m,
                (Self::Pair(pair), Self::Pair(other_pair)) => pair.0 == other_pair.0 && pair.1 == other_pair.1,
                _ => false,
            }
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::{ NumberType::{Regular, self}, NumberType::Pair };
    
    
        #[test]
        fn test_number_type_partial_eq() {
            let actual = Pair(Box::new((Pair(Box::new((Regular(1), Regular(2)))), Regular(3))));
    
            let expected_true = Pair(Box::new((Pair(Box::new((Regular(1), Regular(2)))), Regular(3))));
            assert_eq!(actual, expected_true);
    
            let expected_false = Pair(Box::new((Pair(Box::new((Regular(2), Regular(2)))), Regular(3))));
            assert_ne!(actual, expected_false);
    
            let expected_false = Pair(Box::new((Pair(Box::new((Pair(Box::new((Regular(1), Regular(1)))), Regular(2)))), Regular(3))));
            assert_ne!(actual, expected_false);
    
            let expected_false = Pair(Box::new((Regular(1), Regular(3))));
            assert_ne!(actual, expected_false);   
        }
    
        #[test]
        fn test_number_type_parse_str() {
            let actual = NumberType::parse_str(0, "[[1,2],3]").0;
            assert_eq!(actual, Pair(Box::new((Pair(Box::new((Regular(1), Regular(2)))), Regular(3)))));
    
            let actual = NumberType::parse_str(0, "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]").0;
    
            let p1: NumberType = (1, 2).into();
            let p2: NumberType = (3, 4).into();
            let p3: NumberType = (5, 6).into();
            let p4: NumberType = (7, 8).into();
            let p5: NumberType = (p1, p2).into();
            let p6: NumberType = (p3, p4).into();
            let p7: NumberType = (p5, p6).into();
            let expected: NumberType = (p7, Regular(9)).into();
    
            assert_eq!(actual, expected);
    
        }
    }
    
}

