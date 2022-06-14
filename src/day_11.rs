use crate::parse_lines;

pub fn get_solution_1(is_test: bool) -> usize {
    let file_name = if is_test { "data/day_11_test.txt" } else { "data/day_11.txt" };
    let lines = parse_lines(file_name);
    let octos = parse_numbers(lines);
    <[[u8; 10]; 10] as OctoMap<u8, 10>>::run(octos, 100)
}

pub fn get_solution_2(is_test: bool) -> usize {
    let file_name = if is_test { "data/day_11_test.txt" } else { "data/day_11.txt" };
    let lines = parse_lines(file_name);
    let octos = parse_numbers(lines);
    <[[u8; 10]; 10] as OctoMap<u8, 10>>::run_sync(octos)
}

#[inline(always)]
fn parse_numbers(lines: Vec<String>) -> [[u8; 10]; 10] {
    let mut octos = [[0; 10]; 10];
    for (i, line) in lines.iter().enumerate() {
        for (j, n) in line.as_bytes().iter().enumerate() {
            octos[i][j] = *n - 48;
        }
    }

    octos
}

trait OctoMap<T: Copy, const S: usize> {
    fn get(&self, i: isize, j: isize) -> Option<&T>;
    fn get_mut(&mut self, i: isize, j: isize) -> Option<&mut T>;
    fn run(octos: [[T; S]; S], repeats: usize) -> usize;
    fn run_sync(octos: [[T; S]; S]) -> usize;
    fn flash(octos: [[T; S]; S], i: isize, j: isize, flashes: &mut usize) -> [[T; S]; S];
}

impl<const S: usize> OctoMap<u8, S> for [[u8; S]; S] {
    fn get(&self, i: isize, j: isize) -> Option<&u8> {
        if i < 0 || j < 0 || i as usize >= S || j as usize >= S {
            None
        } else {
            Some(&self[i as usize][j as usize])
        }
    }

    #[inline(always)]
    fn get_mut(&mut self, i: isize, j: isize) -> Option<&mut u8> {
        if i < 0 || j < 0 || i as usize >= S || j as usize >= S {
            None
        } else {
            Some(&mut self[i as usize][j as usize])
        }
    }

    fn run(mut octos: Self, repeats: usize) -> usize {
        // increase all energy levels by one
        // loop through octos again 
        // if octo > 9, set octo = 0 and call flash()
        // since all energy levels were increased already, octos that flashed can be determined by checking
        // octo == 0
        // if so, don't increase energy level in flash() function
        let mut flashes = 0;
        for _ in 0..repeats {
            // increase energy level by one
            for line in octos.iter_mut() {
                for octo in line.iter_mut() {
                    *octo += 1;
                }
            }
    
            for i in 0..S {
                for j in 0..S {
                    let octo = octos.get_mut(i as isize, j as isize).unwrap();
                    if *octo > 9 {
                        *octo = 0;
                        octos = Self::flash(octos, i as isize, j as isize, &mut flashes);
                    }
                }
            }
        }

        flashes
    }

    fn run_sync(mut octos: [[u8; S]; S]) -> usize {
        let size = S * S;
        let mut is_sync = false;
        let mut count = 0;
        while !is_sync {
            let mut flashed = 0;
            // increase energy level by one
            for line in octos.iter_mut() {
                for octo in line.iter_mut() {
                    if *octo == 0 {
                        flashed += 1;
                    } 
                    *octo += 1;
                }
            }

            is_sync = size == flashed;
            count += 1;
    
            for i in 0..S {
                for j in 0..S {
                    let octo = octos.get_mut(i as isize, j as isize).unwrap();
                    if *octo > 9 {
                        *octo = 0;
                        octos = Self::flash(octos, i as isize, j as isize, &mut 0); // use dummy value, so i don't have to change implementation
                    }
                }
            }
        }

        count - 1
    }


    fn flash(octos: [[u8; S]; S], i: isize, j: isize, flashes: &mut usize) -> [[u8; S]; S] {
        *flashes += 1;
        let mut octos = octos;
        for k in i - 1..i + 2 {
            for l in j - 1..j + 2 {
                if k == i && l == j {
                    continue;
                }
    
                if let Some(n) = octos.get_mut(k, l) {
                    if *n == 0 {
                        continue;
                    }
                    *n += 1;
                    if *n > 9 {
                        *n = 0;
                        octos = Self::flash(octos, k, l, flashes);
                    }
                }
            }
        }  
    
        octos
    }
}



#[cfg(test)]
mod tests {
    use crate::parse_lines;
    use super::{OctoMap, parse_numbers};

    fn create_map() -> [[u8; 10]; 10] {
        let lines = parse_lines("data/day_11_test.txt");
        parse_numbers(lines)
    }

    #[test]
    fn test_octo_get_some() {
        let octos = create_map();
        assert_eq!(octos.get(2, 2), Some(&6));
    }

    #[test]
    fn test_octo_get_none() {
        let octos = create_map();
        assert!(octos.get(-1, 0).is_none());
        assert!(octos.get(0, -1).is_none());
        assert!(octos.get(10, 0).is_none());
        assert!(octos.get(0, 10).is_none());
    }

    #[test]
    fn test_one_flash() {
        const SIZE: usize = 5;
        let octos = [
            [1, 1, 1, 1, 1],
            [1, 9, 9, 9, 1],
            [1, 9, 1, 9, 1],
            [1, 9, 9, 9, 1],
            [1, 1, 1, 1, 1],
        ];
        // let expected_one = [
        //     [3, 4, 5, 4, 3],
        //     [4, 0, 0, 0, 4],
        //     [5, 0, 0, 0, 5],
        //     [4, 0, 0, 0, 4],
        //     [3, 4, 5, 4, 3],
        // ];

        // let expected_two = [
        //     [4, 5, 6, 5, 4],
        //     [5, 1, 1, 1, 5],
        //     [6, 1, 1, 1, 6],
        //     [5, 1, 1, 1, 5],
        //     [4, 5, 6, 5, 4],
        // ];
        
        let result = <[[u8; SIZE]; SIZE] as OctoMap<u8, SIZE>>::run(octos, 1);
        assert_eq!(result, 9)  
    }
    
    #[test]
    fn test_day_11_test() {
        assert_eq!(super::get_solution_1(true), 1656);
    }

    #[test]
    fn test_run_sync() {
        assert_eq!(super::get_solution_2(true), 195);
    }
}

