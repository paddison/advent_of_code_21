type Instruction = Vec<String>;

pub fn get_solution_1() -> usize {
    let batches = parse_to_batches().into_iter().skip(5).collect::<Vec<Vec<Instruction>>>();
    let digits = FirstFive::new();
    for (digit, z) in digits.into_iter() {
        if calculate_batch(&batches, z, vec![]) {
            println!("Digit: {:?}", digit);
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
    match (batch[5][2].parse::<isize>(), batch[15][2].parse::<isize>()) {
        (Ok(n),Ok(m)) => (n, m),
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

struct FirstFive {
    n: Numbers
}

impl FirstFive {
    fn new() -> Self {
        Self { n: Numbers::new() }
    }
}

impl Iterator for FirstFive {
    type Item = (usize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let n = self.n.next()?;
            let z = 26_isize.pow(4) * n[0] as isize + 
                    26_isize.pow(3) * n[1] as isize + 
                    26_isize.pow(2) * n[2] as isize +
                    26 * n[3] as isize +
                    n[4] as isize +
                    2373828;
            let mod_z = z % 26;
            if mod_z <= 10 && mod_z >= 2 {
                let digit = n[0] as usize * 10000 + 
                            n[1] as usize * 1000 + 
                            n[2] as usize * 100 + 
                            n[3] as usize * 10 + 
                            n[4] as usize;
                break Some((digit, z));
            }
        }
    }
}

struct Numbers {
    n: [u8; 5]
}

impl Numbers {
    fn new() -> Self {
        Self {n: [8, 0, 0, 0, 0]}
    }
}

impl Iterator for Numbers {
    type Item = [u8; 5];

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == [9, 9, 9, 9, 9] {
            None
        } else {
            for i in (0..5).rev() {
                self.n[i] = (self.n[i]) % 9 + 1;
                if self.n[i] != 1 {
                    break;
                }
            }
            Some(self.n)
        }
    }
}

fn parse() -> Vec<Instruction> {
    include_str!("../data/day_24.txt")
        .split('\n')
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|parts| {
            let mut instr = Vec::new();
            match (parts.get(0), parts.get(1), parts.get(2)) {
                (Some(op), Some(a), Some(b)) => {
                    instr.push(op.to_string());
                    instr.push(a.to_string());
                    instr.push(b.to_string());
                },
                (Some(op), Some(a), None) => {
                    instr.push(op.to_string());
                    instr.push(a.to_string());
                },
                _ => panic!("Invalid input."),
            }
            instr
        })
        .collect()
}

#[test]
fn test_number_iter() {
    let nums = Numbers::new();
    for n in nums {
        println!("{:?}", n);
    }
}