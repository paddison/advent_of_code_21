use std::fmt::Display;

static N_ENHANCEMENTS: isize = 50;

pub fn get_solution_1() -> usize {
    let (algorithm, mut image) = get_input();
    image.fit_for_enhancement();
    // println!("{}", image);
    
    for _ in 0..N_ENHANCEMENTS {
        image.enhance_image(&algorithm);
        // println!("{}", image);
    }
    // println!("{}", image);
    
    image.count_pixels()
}

// Implement map as bitmatrix
#[derive(Debug)]
struct BitMatrix {
    vals: Vec<u8>,
    dim: (usize, usize), // (width, height)
    scale_factor: usize
}

impl BitMatrix {
    // get should be able to be called with negative numbers
    fn get(&self, x: isize, y: isize) -> Option<u8> {
        // check if we're indexing out of bounds
        if x < 0 || y < 0 {
            return None
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.dim.0 || y >= self.dim.1 {
            return None;
        }
         
        // determine bit we need to get
        let i = self.dim.0 * y + x;
        
        // get number from vals field
        let n = *self.vals.get(i / 8)?;
        let bit_index = i % 8;
        Some((n << bit_index) >> 7)
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
                
                if count == 8 {
                    vals.push(n);
                    n = 0;
                    count = 0;
                } 
            }
        }
        let remaining_shift = count % 8;
        if remaining_shift != 0 {
            vals.push(n << remaining_shift); 
        }
    
        self.vals = vals;
        self.dim = (self.dim.0 + range as usize * 2 , self.dim.1 + range as usize * 2 );
    }

    fn count_pixels(&self) -> usize {
        let mut count = 0;
        
        for n in &self.vals {
            let mut n = n.clone();
            while n > 0 {
                if n % 2 == 1 {
                    count += 1;
                }
                n >>= 1;
            }
        }

        // for x in 0..self.dim.0 as isize {
        //     for y in 0..self.dim.1 as isize {
        //         count += match self.get(x, y) {
        //             Some(n) =>  n as usize,
        //             None => 0,
        //         }
        //     }
        // }
        
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
    
                if count == 8 {
                    vals.push(n);
                    n = 0;
                    count = 0;
                } 
            }
        }
        let remaining_shift = count % 8;
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
            let high = std::cmp::min(low + 8, lines.len());
            let num = lines[low..high]
                .chars()
                .fold(0, |acc, c| (acc << 1) + if c == '#' { 1 } else { 0 });
            vals.push(num);
            low += 8;
        }
        // shift the last entry so it fits to 8 bit
        if !(low == lines.len()) {
            let last = vals.last_mut().unwrap();
            let shift = lines.len() % 8;
            *last <<= 8 - shift;
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
    let mut algorithm_index = 0;
    let is_even = image.scale_factor % 2 == 0; // on even enhancement steps everything outside the cave will be '.', on uneven '#'

    for row in y - 1..y + 2 {
        for col in x - 1..x + 2 {
            algorithm_index <<= 1;
            algorithm_index += match image.get(col, row) {
                Some(bit) => bit as isize,
                None => if is_even { 0 } else { 1 },
            }
        }
    }

    algorithm_index
}

// fn enhance_image(algorithm: &BitMatrix, image: BitMatrix) -> BitMatrix {
//     // new bitmatrix is going to be larger by two in each dimension
//     let mut vals = vec![];
//     let mut count = 0;
//     let mut n = 0;
//     for y in -ENHANCE_RANGE..image.dim.1 as isize + ENHANCE_RANGE {
//         for x in -ENHANCE_RANGE..image.dim.0 as isize + ENHANCE_RANGE {
//             count += 1;
  
//             n <<= 1;
//             let algorithm_index = calculate_window(&image, (x, y));
//             if let Some(val) = algorithm.get(algorithm_index, 0) {
//                 n += val;
//             }

//             if count % 8 == 0 {
//                 vals.push(n);
//                 n = 0;
//             } 
//         }
//     }
//     let remaining_shift = count % 8;
//     if remaining_shift != 0 {
//         vals.push(n << remaining_shift); // TODO verify this
//     }

//     BitMatrix { vals , dim: (image.dim.0 + 2 * ENHANCE_RANGE as usize, image.dim.1 + 2 * ENHANCE_RANGE as usize), scale_factor: image.scale_factor + 1 }
// }

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
    use super::{BitMatrix, calculate_window};

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
        let (algorithm, image) = get_test_input();
        println!("{:?}", algorithm);
        println!("{:?}", image);
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
}