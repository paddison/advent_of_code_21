use std::fmt::Display;

static N_ENHANCEMENTS: isize = 50;
static USIZE_LEN: usize = std::mem::size_of::<usize>() * 8;
static FILL_BIT_MODULO: usize = 2;

pub fn get_solution_1() -> usize {
    let (algorithm, mut image) = get_input();

    for _ in 0..N_ENHANCEMENTS {
        image.enhance_image(&algorithm);
    }
    
    image.count_lit_pixels()
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
        let i = Self::get_actual_index(self.dim.0, x as usize, y as usize);
        
        // get number from vals field
        let n = *self.vals.get(i >> 6)?;
        let bit_index = i % USIZE_LEN;
        Some((n >> USIZE_LEN - 1 - bit_index) & 1)
    }

    /// Gets the value at (x, y), panics if index out of bounds
    fn _get_unchecked(&self, x: usize, y: usize) -> usize {
        // determine bit we need to get
        let i = Self::get_actual_index(self.dim.0, x, y);
    
        // get number from vals field
        let n = self.vals[i >> 6];
        let bit_index = i % USIZE_LEN;
        (n >> USIZE_LEN - 1 - bit_index) & 1
    }

    // returns three bits starting from (x, y)
    fn get_triple(&self, x: isize, y: isize) -> usize {
        let fill_bit = self.scale_factor % FILL_BIT_MODULO;

        // if we're above or below the image
        if y >= self.dim.1 as isize || y < 0 {
            return fill_bit * 7;
        } 

        // determine starting bit bit we need to get
        let i = Self::get_actual_index(self.dim.0, std::cmp::max(x, 0) as usize, y as usize);
        let n = *self.vals.get(i >> 6).unwrap(); // unwrap should be safe since we check for y >= self.dim.1 before
        let bit_index = i % USIZE_LEN;
        
        // if we're at the bit on the left of the image
        if x < 0 {
            return fill_bit * if x == -1 { 4 } else { 6 } | (n << bit_index >> USIZE_LEN - (3 + x) as usize )
        }      
        // if the last bit is to the right of the image
        if x >= self.dim.0 as isize - 2 {
            let shift = self.dim.0 - x as usize;

            return (n << bit_index >> USIZE_LEN - shift << 3 - shift) | fill_bit * if shift == 1 { 3 } else { 1 };
        }

        let rev_bit_index = USIZE_LEN - bit_index;
        
        // if we need to cross the usize boundary
        if rev_bit_index == 1 || rev_bit_index == 2 {
            // extract the last rev_bit_index bits of n
            let adj_n = *self.vals.get((i >> 6) + 1).unwrap();
            (n << bit_index >> USIZE_LEN - rev_bit_index << (3 - rev_bit_index)) | 
            // extract the first 3 - rev_bit_index bits of adjacent number (adj_n)
            (adj_n >> USIZE_LEN - (3 - rev_bit_index))
        } else {
            n >> USIZE_LEN - 3 - bit_index & 7
        } 
    }

    #[inline(always)]
    fn get_actual_index(dim: usize, x: usize, y: usize) -> usize {
        dim * y + x
    }

    fn count_lit_pixels(&self) -> usize {
        let mut count = 0;

        for val in &self.vals {
            let mut n = *val;
            while n > 0 {
                count += 1 & n;
                n >>= 1;
            }
        }
        
        count
    }

    fn enhance_image(&mut self, algorithm: &BitMatrix) {
        // new bitmatrix is going to be larger by two in each dimension
        let mut vals = vec![];
        let mut count = 0;
        let mut n = 0;

        for y in -1..self.dim.1 as isize + 1 {
            for x in -1..self.dim.0 as isize + 1 {
                let algorithm_index = self.calculate_algorithm_index(x, y);
                count += 1;
                n <<= 1;

                if let Some(val) = algorithm.get(algorithm_index as isize, 0) {
                    n += val;
                }
    
                if count == USIZE_LEN {
                    vals.push(n);
                    n = 0;
                    count = 0;
                } 
            }
        }
        let remaining_shift = USIZE_LEN - count;

        if remaining_shift != 0 {
            vals.push(n << remaining_shift);
        }

        self.vals = vals;
        self.dim = (self.dim.0 + 2, self.dim.1 + 2);
        self.scale_factor += 1;
    }


    // (x, y) = coordinate of center in window
    fn calculate_algorithm_index(&self, x: isize, y: isize) -> usize {
        let mut index = 0;

        for row in y - 1..y + 2 {
            index <<= 3;
            index += self.get_triple(x - 1, row);
        }
        index
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
        if low != lines.len() {
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

/// Returns (algorithm, Image)
fn get_input() -> (BitMatrix, BitMatrix) {
    let input = include_str!("../data/day_20.txt");
    let split_index = input.find('\n').unwrap();

    (input[..split_index + 1].into(), input[split_index + 2..].into())
}

#[cfg(test)]
mod tests {
    use super::{BitMatrix, USIZE_LEN};

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
        assert_eq!(bm.dim, (5, 5));
    }

    #[test]
    fn test_bit_matrix_get_is_some() {
        let bm: BitMatrix = get_test_string().into();
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
    fn test_calculate_window() {
        let image: BitMatrix = get_test_string().into();
        assert_eq!(image.calculate_algorithm_index(2, 2), 34);
    }

    #[test]
    fn test_image_count_lit_pixels() {
        let (_, image) = get_test_input();
        assert_eq!(image.count_lit_pixels(), 10);
    }

    #[test]
    fn test_get_triple() {

        let bm = BitMatrix { vals: vec![2, 2_usize.pow(63) + 2_usize.pow(62), 2_usize.pow(63) + 2_usize.pow(62) + 7, 2], dim: (128, 2), scale_factor: 1};

        assert_eq!(bm.get_triple(126, 1), 5);
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
        
        let index = 19;
        
        assert_eq!(n << index >> USIZE_LEN - 3, 2);

        let fill_bit = 1;
        let x_larger_by = 1;
        assert_eq!(fill_bit * ((1 << 3 - x_larger_by) - 1), 3);
        let x_larger_by = 2;
        assert_eq!(fill_bit * ((1 << 3 - x_larger_by) - 1), 1);
        
        let n: usize = 11116172303001903548; 
        let bit_index = 0; 
        let x: isize = 2; 
        let shift = 1;
        let fill_bit = 1;
        
        assert_eq!((fill_bit * ((1 << x as usize) - 1) << 3 - x) | 
                   (n << bit_index >> USIZE_LEN - shift), 7);
    }

}