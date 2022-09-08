use std::fmt::Display;

static N_ENHANCEMENTS: isize = 2;
static USIZE_LEN: usize = std::mem::size_of::<usize>() * 8;
static FILL_BIT_MODULO: usize = 1;

pub fn get_solution_1() -> usize {
    let (algorithm, mut image) = get_test_input();
    image.fit_for_enhancement();

    println!("{:064b}", image.vals[0]);
    // for _ in 0..N_ENHANCEMENTS {
    //     image.enhance_image(&algorithm);

    // }
    // println!("{}", image);
    
    image.count_pixels()
}

// Implement map as bitmatrix
#[derive(Debug)]
struct BitMatrix {
    vals: Vec<usize>,
    dim: (usize, usize), // (width, height)
    scale_factor: usize
}

impl BitMatrix {
    // get should be able to be called with negative numbers
    fn get(&self, x: isize, y: isize) -> Option<usize> {
        // check if we're indexing out of bounds
        if x < 0 || y < 0 || x as usize >= self.dim.0 || y as usize >= self.dim.1{
            return None
        }
         
        // determine bit we need to get
        let i = Self::get_actual_index(self.dim.0, y as usize, x as usize);
        
        // get number from vals field
        let n = *self.vals.get(i >> 6)?;
        let bit_index = i % USIZE_LEN;
        Some((n << bit_index) >> USIZE_LEN - 1)
    }

    // returns three bits starting from (x, y)
    fn get_triple(&self, x: isize, y: isize) -> usize {
        let fill_bit = self.scale_factor % FILL_BIT_MODULO;

        // if we're above or below the image
        if y >= self.dim.1 as isize || y < 0 {
            return fill_bit * 7;
        } 
        // determine starting bit bit we need to get
        
        // if we're at the bit on the left of the image
        if x < 0 {
            let i = Self::get_actual_index(self.dim.0, (x + 1) as usize, y as usize);
            let n = *self.vals.get(i >> 6).unwrap(); // unwrap should be safe since we check for y >= self.dim.1 before
            return fill_bit * 4 + (n >> USIZE_LEN - 2);
        }
        let i = Self::get_actual_index(self.dim.0, x as usize, y as usize);
        let n = *self.vals.get(i >> 6).unwrap(); // unwrap should be safe since we check for y >= self.dim.1 before
        
        // if the last bit is to the right of the image
        if x >= self.dim.0 as isize - 2{ 
            return fill_bit + (n & 3);
        }

        let bit_index = i % USIZE_LEN;
        let rev_bit_index = USIZE_LEN - bit_index;
        
        // if we need to cross the usize boundary
        if rev_bit_index == 1 || rev_bit_index == 2 {
            // extract the last rev_bit_index bits of n
            let adj_n = *self.vals.get((i >> 6) + 1).unwrap();
            (n << bit_index >> USIZE_LEN - rev_bit_index << (3 - rev_bit_index)) | 
            // extract the first 3 - rev_bit_index bits of adjacent number (adj_n)
            (adj_n >> USIZE_LEN - (3 - rev_bit_index))
        } else {
            let first = n << bit_index;
            println!("{}", first);
            println!("{}", first  >> (USIZE_LEN - 2));
            first  >> (USIZE_LEN - 2)
        } 
    }

    #[inline(always)]
    fn get_actual_index(dim: usize, x: usize, y: usize) -> usize {
        dim * y as usize + x as usize
    }

    fn fit_for_enhancement(&mut self) {
        let mut vals = vec![];
        let mut count = 0;
        let mut n = 0;
        let range = N_ENHANCEMENTS;
        
        for y in -range..self.dim.1 as isize + range {
            for x in -range..self.dim.0 as isize + range {
                count += 1;
                
                n <<= 1;
                if let Some(val) = self.get(x, y) {
                    n += val;
                }
                
                if count == USIZE_LEN {
                    vals.push(n);
                    n = 0;
                    count = 0;
                } 
            }
        }
        let remaining_shift = count % USIZE_LEN;
        if remaining_shift != 0 {
            vals.push(n << remaining_shift); 
        }
    
        self.vals = vals;
        self.dim = (self.dim.0 + range as usize * 2 , self.dim.1 + range as usize * 2 );
    }

    fn count_pixels(&self) -> usize {
        let mut count = 0;

        for x in 0..self.dim.0 as isize {
            for y in 0..self.dim.1 as isize {
                count += match self.get(x, y) {
                    Some(n) => n as usize,
                    None => 0,
                }
            }
        }
        
        count
    }

    fn enhance_image(&mut self, algorithm: &BitMatrix) {
        // new bitmatrix is going to be larger by two in each dimension
        if self.scale_factor as isize >= N_ENHANCEMENTS {
            eprintln!("Enhancing image further will lead to incorrect results");
            std::process::exit(1);
        }

        let mut vals = vec![];
        let mut count = 0;
        let mut n = 0;
        for y in 0..self.dim.1 as isize {
            for x in 0..self.dim.0 as isize {
                count += 1;
      
                n <<= 1;
                let algorithm_index = calculate_window(&self, (x, y));
                if let Some(val) = algorithm.get(algorithm_index, 0) {
                    n += val;
                }
    
                if count == USIZE_LEN {
                    vals.push(n);
                    n = 0;
                    count = 0;
                } 
            }
        }
        let remaining_shift = count % USIZE_LEN;
        if remaining_shift != 0 {
            vals.push(n << remaining_shift); // TODO verify this
        }
        self.vals = vals;
        self.scale_factor += 1;
    }
}

impl From<&str> for BitMatrix {
    fn from(input: &str) -> Self {
        let mut vals = vec![];
        let width = input.find('\n').unwrap();
        let lines: String = input.split('\n').collect();
        
        let mut low = 0;
        while low < lines.len() {
            let high = std::cmp::min(low + USIZE_LEN, lines.len());
            let num = lines[low..high]
                .chars()
                .fold(0, |acc, c| (acc << 1) + if c == '#' { 1 } else { 0 });
            vals.push(num);
            low += USIZE_LEN;
        }
        // shift the last entry so it fits to 8 bit
        if !(low == lines.len()) {
            let last = vals.last_mut().unwrap();
            let shift = lines.len() % USIZE_LEN;
            *last <<= USIZE_LEN - shift;
        } 

        BitMatrix { vals, dim: (width, lines.len() / width ), scale_factor: 0}
    }
}

impl Display for BitMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display = String::new();
        for y in 0..self.dim.1 {
            for x in 0..self.dim.0 {
                if let Some(n) = self.get(x as isize, y as isize) {
                    display += match n {
                        0 => ".",
                        _ => "#",
                    }
                }
            }
            display += "\n";
        }

        write!(f, "{}", display)
    }
}

// (x, y) = coordinate of center in window
fn calculate_window(image: &BitMatrix, (x, y): (isize, isize)) -> isize {
    let mut index = 0;
    let fill_bit = (image.scale_factor % FILL_BIT_MODULO) as isize; // on even enhancement steps everything outside the cave will be '.', on uneven '#'


    // for row in y - 1..y + 2 {
    //     for col in x - 1..x + 2 {
    //         index <<= 1;
    //         index += match image.get(col, row) {
    //             Some(bit) => bit as isize,
    //             None => fill_bit,
    //         }
    //     }
    // }

    let mut index = 0;
    for row in y - 1..y + 2 {
        index <<= 3;
        index += image.get_triple(x - 1, row);
    }
        // println!("{}", index);
    // algorithm_index
    index as isize
}

/// Returns (algorithm, Image)
fn get_input() -> (BitMatrix, BitMatrix) {
    let input = include_str!("../data/day_20.txt");
    let split_index = input.find('\n').unwrap();

    (input[..split_index + 1].into(), input[split_index + 2..].into())
}

fn get_test_input() -> (BitMatrix, BitMatrix) {
    let input = include_str!("../data/day_20_test.txt");
    let split_index = input.find('\n').unwrap();

    (input[..split_index + 1].into(), input[split_index + 2..].into())
}

#[cfg(test)]
mod tests {
    use super::{BitMatrix, calculate_window, USIZE_LEN};

    fn get_test_string() -> &'static str {
"#..#.
#....
##..#
..#..
..###"
    }

    fn get_test_input() -> (BitMatrix, BitMatrix) {
        let input = include_str!("../data/day_20_test.txt");
        let split_index = input.find('\n').unwrap();
    
        (input[..split_index + 1].into(), input[split_index + 2..].into())
    }

    #[test]
    fn test_bit_matrix_from_string() {
        let bm: BitMatrix = get_test_string().into();

        println!("{:?}", bm);
        assert_eq!(bm.dim, (5, 5));
    }

    #[test]
    fn test_bit_matrix_get_is_some() {
        let bm: BitMatrix = get_test_string().into();
        println!("{}", bm);
        assert_eq!(bm.get(0, 0), Some(1));
        assert_eq!(bm.get(1, 0), Some(0));
        assert_eq!(bm.get(4, 2), Some(1));
        assert_eq!(bm.get(4, 0), Some(0));
        assert_eq!(bm.get(4, 4), Some(1));
    }

    #[test]
    fn test_bit_matrix_get_is_none() {
        let bm: BitMatrix = get_test_string().into();

        assert!(bm.get(4,5).is_none());
        assert!(bm.get(5,0).is_none());
    }

    #[test]
    fn test_get_input() {
        let (algorithm, mut image) = get_test_input();
        image.fit_for_enhancement();
        image.enhance_image(&algorithm);
        // println!("{:?}", algorithm);
        println!("{}", image);
    }

    #[test]
    fn test_calculate_window() {
        let image: BitMatrix = get_test_string().into();

        assert_eq!(calculate_window(&image, (2, 2)), 34);
    }

    #[test]
    fn test_image_count_lit_pixels() {
        let (_, image) = get_test_input();
        assert_eq!(image.count_pixels(), 10);
    }

    // #[test]
    // fn test_enhance_image() {
    //     let (algorithm, image) = get_test_input();
    //     println!("{}", algorithm.get(256, 0).unwrap());
    //     println!("{}", image);

    //     let image = enhance_image(&algorithm, image);
    //     println!("{}", image);

    //     let image = enhance_image(&algorithm, image);
    //     println!("{}", image);

    //     assert_eq!(image.count_pixels(), 35);
    // }

    #[test]
    fn test_get_triple() {
        let bm = BitMatrix { vals: vec![2, 2_usize.pow(63) + 2_usize.pow(62), 2_usize.pow(63) + 2_usize.pow(62) + 7, 2], dim: (128, 2), scale_factor: 1};

        // cross number 
        assert_eq!(bm.get_triple(126, 1), 3);
        assert_eq!(bm.get_triple(-1, 1), 7);
        assert_eq!(bm.get_triple(62, 0), 5);
        assert_eq!(bm.get_triple(63, 0), 3);
        assert_eq!(bm.get_triple(64, 0), 6);
        assert_eq!(bm.get_triple(0, -1), 7);
        assert_eq!(bm.get_triple(0, 129), 7);
    }

    #[test]
    fn test_shifts() {
        let n: usize = 0b000000000_000000000_001001000_001000000_001100100_000010000_000011100_0;
        // let n2: usize = 0b11100000000100000000011000010001000000101000
        assert_eq!(n, 15397464117288);
        println!("{}", n);
        let index = 19;
        
        assert_eq!(n << index >> USIZE_LEN - 3, 2);
    }
}