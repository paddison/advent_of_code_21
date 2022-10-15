use std::{ops::Deref, fmt::Display, collections::VecDeque};

type Instruction = (IType, RType, Option<RType>);

pub fn get_solution_1() -> usize {
    let batches = parse_to_batches().into_iter().skip(5).collect::<Vec<Vec<Instruction>>>();
    let candidates = get_first_five();
    for (digit, z) in candidates.into_iter() {
        println!("Digit: {:?}", digit);
        if calculate_batch(&batches, z, vec![]) {
            break;
        }
    }
    0
}

fn calculate_batch(batches: &[Vec<Instruction>], z: isize, others: Vec<isize>) -> bool {
    if batches.len() == 0 {
        println!("z: {}", z);
        println!("Digits: {:?}", others);
        return true;
    }
    for w in 1..10 {
        // println!("Batch {} of 14", 15 - batches.len());
        let mut new_others = others.clone();
        new_others.push(w);
        let (n, m) = get_variables(&batches[0]);
        if let Some(z) = try_digit(z, n, m, w) {
            if calculate_batch(&batches[1..], z, new_others.clone()) {
                return true;
            }
        }
    }

    false
}

fn parse_to_batches() -> Vec<Vec<Instruction>> {
    let instructions = parse();
    let mut batches = Vec::new();
    for n in 0..14 {
        batches.push(instructions.iter().skip(n * 18).take(18).cloned().collect::<Vec<Instruction>>());
    }
    batches
}

fn get_variables(batch: &[Instruction]) -> (isize, isize) {
    match (batch[5].2.unwrap(), batch[15].2.unwrap()) {
        (RType::Const(n), RType::Const(m)) => (n, m),
        _ => panic!("Invalid indices"),
    }
}

fn try_digit(z: isize, n: isize, m: isize, w: isize) -> Option<isize> {
    if n < 0 {
        if (z % 26) + n == w {
            Some(z / 26)
        } else {
            None
        }
    } else {
        Some(26 * z + w + m)
    }
}

fn get_first_five() -> Vec<(VecDeque<isize>, isize)> {
    let mut valids = Vec::new();
    for n in 11111..100000 {
        let digits = deque_from_usize(n);
        if digits.iter().take(5).find(|e| e == &&0).is_some() {
            continue;
        }
        
        let z = 26_isize.pow(4) * digits[0] + 
                    26_isize.pow(3) * digits[1] + 
                    26_isize.pow(2) * digits[2] +
                    26 * digits[3] +
                    digits[4] +
                    2373828;
        let mod_z = z % 26;
        if mod_z <= 10 && mod_z >= 2 {
            valids.push((digits, z));
        }
    }
    println!("{}", valids.len());
    valids
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