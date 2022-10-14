use std::{ops::Deref, fmt::Display, collections::VecDeque, io::Write};

type Instruction = (IType, RType, Option<RType>);

pub fn get_solution_1() -> usize {
    let instructions = parse();
    // first three numbers don't seem to matter

    // always 18 instructions before next input
    // let mut n = 0;
    // let mut comp = 0;
    let inputs = get_first_five(); // 1 - 5
    let inputs = get_next_twos(inputs, 9, 17, 5); // 6-7
    let inputs = get_next(inputs, 8, 16, 7); // 8
    for (digit, z) in inputs.into_iter().rev() {
        println!("testing: {:?}", digit);
        if execute_n(z, 6, 8) {
            println!("found: {:?}", digit);
            return 0
        }
    }
    // let inputs = get_next(inputs, 9, 17, 8); // 9
    // let inputs = get_next_twos(inputs, 8, 16, 9); // 10-11
    // let inputs = get_next(inputs, 3, 11, 6); // 12
    // let inputs = get_next(inputs, 3, 11, 6); // 13
    // let inputs = get_next(inputs, 14, 22, 6); // 8
    // let inputs = get_next


        // let mut alu = Alu::new_with_inputs(VecDeque::from([comp]));
    // for i in 10..11 {
    //     let batch = instructions.clone().into_iter().skip(i * 18).take(18).collect::<Vec<Instruction>>();
    //     let mut valid_numbers = get_valid_numbers(&batch, comp);
    //     println!("{:?}", valid_numbers);
    //     comp = 0;
    // }
        // n += comp;
        // n *= 10;
    
    
    0
}

fn execute_n(z: isize, n: usize, skipped: usize) -> bool {
    let zs = [(9, 17), (isize::MIN, isize::MAX), (3, 11), (3, 11), (14, 22)];
    let instructions = parse().into_iter().skip(skipped * 18).take(n * 18).collect::<Vec<Instruction>>();
    let mut batches = Vec::new();
    for i in 0..n {
        let batch = parse().into_iter().skip((skipped + i) * 18).take(18).collect::<Vec<Instruction>>();
        batches.push(batch);
    }
    for n in (10_usize.pow(n as u32)..10usize.pow(n as u32 + 1)).rev() {
        let mut z_copy = z;
        let digits = deque_from_usize(n);
        if digits.iter().take(5).find(|e| e == &&0).is_some() {
            continue;
        }
        
        for (i, d) in digits.iter().enumerate() {
            let mut alu = Alu::new_with_inputs(VecDeque::from([*d]));
            alu.z = z_copy;
            for instr in &batches[i] {
                alu.exec_instruction(*instr);
            }
            if (alu.z % 26) >= zs[i].0 && (alu.z % 26) <= zs[i].1 {
                z_copy = alu.z;
                continue;
            } else {
                break
            }
        }
        if z_copy == 0 {
            println!("{:?}", digits);
            return true;
        }
    }

    false
}

fn get_first_five() -> Vec<(VecDeque<isize>, isize)> {
    let mut valids = Vec::new();
    for n in 111111..1000000 {
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

fn get_next_twos(inputs: Vec<(VecDeque<isize>, isize)>, z_lower: isize, z_upper: isize, skipped: usize) -> Vec<(VecDeque<isize>, isize)> {
    let mut valids = Vec::new();
    let instructions = parse().into_iter().skip(skipped * 18).take(2).collect::<Vec<Instruction>>();
    for (digit, z) in inputs {
        for i in 11..100 {
            if i % 10 == 0 {
                continue;
            }
            let inputs = deque_from_usize(i);
            let mut alu = Alu::new_with_inputs(inputs.clone());
            alu.z = z;
            for instruction in &instructions {
                alu.exec_instruction(*instruction);
            }
            let mod_z = alu.z % 26;
            if mod_z >= z_lower && mod_z <= z_upper {
                let mut clone = digit.clone();
                clone[5] = inputs[0];
                clone[6] = inputs[1];
                valids.push((clone, alu.z));
            }
        }
    }
    println!("{}", valids.len());

    valids
}

fn get_next(inputs: Vec<(VecDeque<isize>, isize)>, z_lower: isize, z_upper: isize, skipped: usize) -> Vec<(VecDeque<isize>, isize)> {
    let mut valids = Vec::new();
    let instructions = parse().into_iter().skip(skipped * 18).take(1).collect::<Vec<Instruction>>();
    for (digit, z) in inputs {
        for i in 1..10 {
            let inputs = VecDeque::from([i]);
            let mut alu = Alu::new_with_inputs(inputs.clone());
            alu.z = z;
            for instruction in &instructions {
                alu.exec_instruction(*instruction); 
            }
            let mod_z = alu.z % 26;
            if mod_z >= z_lower && mod_z <= z_upper {
                let mut clone = digit.clone();
                clone[7] = inputs[0];
                valids.push((clone, alu.z));
            }
        }
    }
    
    println!("{:?}, {:?}, {}", valids[0], valids.last(), valids.len());
    valids
}

// instructions repeat every 18 instructions, figure out numbers in reverse
// return all numbers that were valid, maximum number is at last position.
fn get_valid_numbers(instructions: &[Instruction], comp: isize) -> Vec<(isize, isize)> {
    let mut valid_numbers = Vec::new();
    for input in 1..10 {
        for z in 0..10000 {
            let mut alu = Alu {
                w: 0,
                x: 0,
                z,
                y: 0,
                inputs: VecDeque::from([input]),
            };
            for instruction in instructions {
                alu.exec_instruction(*instruction);
            }
            if alu.z == comp {
                valid_numbers.push((input, z));
            }
        }
    }

    valid_numbers
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

struct Alu {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
    inputs: VecDeque<isize>
}

impl Alu {

    fn new() -> Self {
        Alu { w: 0, x: 0, y: 0, z: 0, inputs: VecDeque::new() }
    }

    fn new_with_inputs(inputs: VecDeque<isize>) -> Self {
        Alu { w: 0, x: 0, y: 0, z: 0, inputs }
    }

    fn exec_instruction(&mut self, instruction: Instruction) {
        match instruction {
            (IType::Inp, lhs, None) => self.exec_inp(lhs),
            (IType::Add, lhs, Some(rhs)) => self.exec_add(lhs, rhs),
            (IType::Mul, lhs, Some(rhs)) => self.exec_mul(lhs, rhs),
            (IType::Div, lhs, Some(rhs)) => self.exec_div(lhs, rhs),
            (IType::Mod, lhs, Some(rhs)) => self.exec_mod(lhs, rhs),
            (IType::Eql, lhs, Some(rhs)) => self.exec_eql(lhs, rhs),
            _ => panic!("Got invalid instruction"),
        }
    }

    fn exec_inp(&mut self, lhs: RType) {
        let val = match self.inputs.pop_front() {
            Some(val) => val,
            None => 0,
        };
        self.write(lhs, val);
    }

    fn exec_add(&mut self, lhs: RType, rhs: RType) {
        self.write(lhs, self.load(lhs) + self.load(rhs));
    }

    fn exec_mul(&mut self, lhs: RType, rhs: RType) {
        self.write(lhs, self.load(lhs) * self.load(rhs));
    }

    fn exec_div(&mut self, lhs: RType, rhs: RType) {
        if self.load(rhs) < 0 {
            panic!("Cannot divide by 0");
        }
        self.write(lhs, self.load(lhs) / self.load(rhs));
    }

    fn exec_mod(&mut self, lhs: RType, rhs: RType) {
        let lhs_val = self.load(lhs);
        let rhs_val = self.load(rhs);
        if lhs_val < 0 || rhs_val <= 0 {
            panic!("a: {} needs to be < 0 and b: {} needs to be <= 0", lhs_val, rhs_val);
        }
        self.write(lhs, lhs_val % rhs_val);
    }

    fn exec_eql(&mut self, lhs: RType, rhs: RType) {
        let val = if self.load(lhs) == self.load(rhs) { 1 } else { 0 };
        self.write(lhs, val);
    }

    fn write(&mut self, register: RType, val: isize) {
        match register {
            RType::W(_) => self.w = val,
            RType::X(_) => self.x = val,
            RType::Y(_) => self.y = val,
            RType::Z(_) => self.z = val,
            RType::Const(_) => panic!("Const is not a valid register"),
        }
    }

    fn load(&self, register: RType) -> isize {
        match register {
            RType::W(_) => self.w,
            RType::X(_) => self.x,
            RType::Y(_) => self.y,
            RType::Z(_) => self.z,
            RType::Const(val) => val,
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

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::Alu;

    
    #[test]
    fn test_parse() {
        for line in super::parse() {
            println!("{} {} {:?}", line.0, line.1, line.2);
        }
    }

    #[test]
    fn test_execute_instruction() {
        let mut alu = Alu::new_with_inputs(VecDeque::from([10]));
        let instructions = super::parse();
        for instruction in instructions {
            alu.exec_instruction(instruction);
        }

        assert_eq!(alu.z, 0); // 1 
        assert_eq!(alu.y, 1); // 2
        assert_eq!(alu.x, 0); // 4
        assert_eq!(alu.w, 1); // 8
    }
}