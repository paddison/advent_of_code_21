use std::collections::HashMap;

use crate::parse_lines;

type PolyMap = HashMap<String, String>;
type GrowthMap = HashMap<String, u128>;
type CountMap = HashMap<String, u128>;

pub fn get_solution_1(is_test: bool) -> u128 {
    let file = if is_test {
        "data/day_14_test.txt"
    } else {
        "data/day_14.txt"
    };
    let lines = parse_lines(file);
    let mut poly = lines[0].to_string();
    let map = get_poly_map(&lines[2..]);

    for i in 0..10 {
        println!("Step: {i}");
        poly = grow(&map, poly);
    }

    let count_map = create_count_map(&poly);
    let most_common = get_most_common(&count_map);
    let least_common = get_least_common(&count_map);

    most_common - least_common
}

pub fn get_solution_2(is_test: bool, iterations: usize) -> u128 {
    let file = if is_test {
        "data/day_14_test.txt"
    } else {
        "data/day_14.txt"
    };
    let lines = parse_lines(file);
    let poly = lines[0].to_string();
    let poly_map = get_poly_map(&lines[2..]);
    let mut growth_map = poly_to_growth_map(&poly);
    let mut count_map = create_count_map(&poly);
    for _ in 0..iterations {
        growth_map = grow_map(growth_map, &poly_map, &mut count_map);
        // poly = grow(&map, poly);
    }



    // let count_map = count_map_from_growth_map(growth_map);
    let most_common = get_most_common(&count_map);
    let least_common = get_least_common(&count_map);

    most_common - least_common
}

fn get_poly_map(lines: &[String]) -> PolyMap {
    let mut map = HashMap::new();

    for line in lines {
        let key_value_pair: Vec<&str> = line.split("->").collect();
        assert_eq!(key_value_pair.len(), 2);
        map.insert(key_value_pair[0].trim().to_owned(), key_value_pair[1].trim().to_owned());    
    }
    map
}

// for solution 1
fn grow(poly_map: &PolyMap, poly: String) -> String {
    let mut new_poly = poly[0..1].to_string();
    for i in 0..poly.len() - 1 {
        let to_insert = poly_map.get(&poly[i..i + 2]).unwrap();
        new_poly += to_insert;
        new_poly += &poly[i + 1..i + 2];
    }

    new_poly
}

// checks the pairs contained in a polymer and creates a map contianing the pairs as keys and the 
// number of occurences of them as value
fn poly_to_growth_map(poly: &str) -> GrowthMap {
    let mut growth_map = HashMap::new();
    for i in 0..poly.len() - 1 {
        let entry = growth_map.entry(poly[i..i + 2].to_string()).or_insert(0);
        *entry += 1;
    }

    growth_map
}

// "grows" the growth map
fn grow_map(growth_map: GrowthMap, poly_map: &PolyMap, count_map: &mut CountMap) -> GrowthMap {
    let mut new_growth_map = growth_map.clone();
    for (k, v) in growth_map.iter() {
        if *v == 0 {
            continue;
        } 
        // determine letter to insert
        let to_insert = poly_map.get(k).unwrap();

        // update count of letters in count map
        let count_entry = count_map.entry(to_insert.to_owned()).or_insert(0);
        *count_entry += *v;

        // create new pairs (NN creates NC and CN for example)
        let first = k[0..1].to_string() + to_insert;
        // second new pair generated
        let second = to_insert.to_string() + &k[1..2];

        // if there are n pairs of NN in poly, n paris of NC and CN get generated
        let first_entry = new_growth_map.entry(first).or_insert(0);
        *first_entry += *v;
        let second_entry = new_growth_map.entry(second).or_insert(0);
        *second_entry += *v;

        // since NN -> NCN, need to delete old NN pairs
        let old_entry = new_growth_map.get_mut(k).unwrap();
        *old_entry -= *v;
    }

    new_growth_map
}

fn create_count_map(poly: &str) -> HashMap::<String, u128> {
    let mut map = HashMap::new();
    for c in poly.chars() {
        let entry = map.entry(c.to_string()).or_insert(0);
        *entry += 1;
    }
    map
}

fn get_most_common(count_map: &HashMap::<String, u128>) -> u128 {
    let mut max = 0;
    for v in count_map.values() {
        if *v > max {
            max = *v;
        }
    }

    max
}

fn get_least_common(count_map: &HashMap::<String, u128>) -> u128 {
    let mut min = u128::MAX;
    for v in count_map.values() {
        if *v < min {
            min = *v;
        }
    }

    min
}

#[cfg(test)]
mod tests {
    

    use crate::{parse_lines, day_14::create_count_map};

    use super::{get_poly_map, grow, get_solution_1, poly_to_growth_map, grow_map, PolyMap};
    
    fn get_test_data() -> (String, PolyMap) {
        let lines = parse_lines("data/day_14_test.txt");
        let poly = lines[0].to_string();
        let map = get_poly_map(&lines[2..]);
        (poly, map)
    }

    #[test]
    fn test_grow() {
        let (poly, poly_map) = get_test_data();
        println!("{:?}", poly_map);
        println!("{}", poly); 
        grow(&poly_map, poly);
    }

    #[test]
    fn test_solution_1() {
        let result = get_solution_1(true);
        assert_eq!(1588, result);
    }

    #[test]
    fn check_growth_map() {
        // let result = super::get_solution_2(true, 3);
        let lines = parse_lines("data/day_14_test.txt");
        let poly = lines[0].to_string();
        let poly_map = get_poly_map(&lines[2..]);
        let mut growth_map = poly_to_growth_map(&poly);
        let mut count_map = create_count_map(&poly);
        println!("Template:     NNCB");
        println!("{:?}", growth_map);
        growth_map = grow_map(growth_map, &poly_map, &mut count_map);
        println!("After step 1: NCNBCHB");
        println!("{:?}", growth_map);
        growth_map = grow_map(growth_map, &poly_map, &mut count_map);
        println!("After step 2: NBCCNBBBCBHCB");
        println!("{:?}", growth_map);
        growth_map = grow_map(growth_map, &poly_map, &mut count_map);
        println!("After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB");
        println!("{:?}", growth_map);
        growth_map = grow_map(growth_map, &poly_map, &mut count_map);
        println!("After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");
        println!("{:?}", growth_map);
    }

    #[test]
    fn test_solution_1_from_growth_map() {
        let result = super::get_solution_2(true, 10);
        assert_eq!(result, 1588);

        let result = super::get_solution_2(false, 10);
        assert_eq!(result, 3306)
    }
}