use std::{ops::Deref, fmt::Display, collections::VecDeque, io::Write};

type Instruction = (IType, RType, Option<RType>);

pub fn get_solution_1() -> usize {

    0
}




fn deque_from_usize(mut n: usize) -> VecDeque<isize> {

    let mut deque = VecDeque::new();
    while n > 0 {
        deque.push_back((n % 10) as isize);
        n /= 10;
    }
    while deque.len() < 14 {
        deque.push_back(0);
    }
    deque
}

/// Builds a deque where every number is of value 'others', except the indexth value,
/// which will be of value 'digit'
fn deque_with_adjustable_digit(others: isize, digit: isize, index: usize) -> VecDeque<isize> {
    assert!(index < 18);
    let mut deque = VecDeque::new();

    for i in 0..18 {
        if i == index {
            deque.push_back(digit);
        } else {
            deque.push_back(others)
        }
    }

    deque
}

#[derive(Clone, Copy, Debug)]
enum RType {
    W(isize),
    X(isize),
    Y(isize),
    Z(isize),
    Const(isize),
}

impl From<RType> for String {
    fn from(input: RType) -> Self {
        match input {
            RType::W(val) => format!("w({})", val),
            RType::X(val) => format!("x({})", val),
            RType::Y(val) => format!("y({})", val),
            RType::Z(val) => format!("z({})", val),
            RType::Const(val) => format!("{}", val),
        }
    }
}

impl Display for RType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

impl Deref for RType {
    type Target = isize;

    fn deref(&self) -> &Self::Target {
        match self {
            RType::W(val) => val,
            RType::X(val) => val,
            RType::Y(val) => val,
            RType::Z(val) => val,
            RType::Const(val) => val,
        }
    }
}


impl From<&str> for RType {
    fn from(input: &str) -> Self {
        match input {
            "w" => RType::W(0),
            "x" => RType::X(0),
            "y" => RType::Y(0),
            "z" => RType::Z(0),
            n => RType::Const(n.parse::<isize>().unwrap()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum IType {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl Display for IType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let instr: &str = (*self).into();
        write!(f, "{}", instr)
    }
}

impl From<IType> for &str {
    fn from(input: IType) -> Self {
        match input {
            IType::Inp => "inp",
            IType::Add => "add",
            IType::Mul => "mul",
            IType::Div => "div",
            IType::Mod => "mod",
            IType::Eql => "eql",
        }
    }
}

impl From<&str> for IType {
    fn from(input: &str) -> Self {
        match input {
            "inp" => IType::Inp,
            "add" => IType::Add,
            "mul" => IType::Mul,
            "div" => IType::Div,
            "mod" => IType::Mod,
            "eql" => IType::Eql,
            _ => unreachable!(),
        }
    }
}

fn parse() -> Vec<Instruction> {
    include_str!("../data/day_24.txt")
        .split('\n')
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|parts| (parts[0].into(), parts[1].into(), parts.get(2).map(|r| (*r).into())))
        .collect()
}