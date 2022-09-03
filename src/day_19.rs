use std::{collections::{HashSet, HashMap}, mem::swap};

type Beacon = (i16, i16, i16);
type Position = (i16, i16, i16);
type Scanner = Vec<Beacon>;

pub fn get_solution_1() -> usize {
    let scanners = parse_input();
    let calculated_scanners = determine_beacons_and_scanners(scanners).iter().map(|(scanner, _)| scanner.clone()).collect();
    let beacons = collect_beacons(calculated_scanners);

    beacons.len()
}

pub fn get_solution_2() -> i16 {
    let scanners = parse_input();

    let scanner_positions: Vec<Position> = determine_beacons_and_scanners(scanners).iter().map(|(_, pos)| *pos).collect();
    let mut max = 0;
    for (i, pos_left) in scanner_positions.iter().enumerate() {
        for pos_right in scanner_positions.iter().skip(i + 1) {
            let distance = (pos_left.0 - pos_right.0).abs() + (pos_left.1 - pos_right.1).abs() + (pos_left.2 - pos_right.2).abs();
            if max < distance {
                max = distance;
            }
        }
    }

    max
}

fn determine_beacons_and_scanners(mut scanners: Vec<Scanner>) -> Vec<(Scanner, Position)> {
    let mut calculated_scanners = vec![(scanners.remove(0), (0, 0, 0))];

    // do slow solution first (compare everything with everything)
    while scanners.len() > 0 {
        let mut found_index = None;

        for (left, _) in &mut calculated_scanners {
            'outer: for (i, right) in scanners.iter_mut().enumerate() {
                for j in 1..24 {
                    let positions_map = create_positions_map(&left, right);
                    if let Some(pos) = try_get_position(positions_map) {
                        calculate_absolute_beacon_positions(right, pos);
                        found_index = Some((i, pos));
                        break 'outer;
                    }
                    rotate(right, j);
                }
            }
            if let Some((i, pos)) = found_index {
                calculated_scanners.push((scanners.remove(i), pos));
                break;
            }
        }
    }
    
    calculated_scanners
}

fn parse_input() -> Vec<Scanner> {
    let mut scanners = vec![];
    let mut cur_scanner = vec![];
    for line in include_str!("../data/day_19.txt").split('\n').skip(1) {
        if line.len() == 0 {
            continue;
        }
        if line.starts_with("---") {
            scanners.push(cur_scanner);
            cur_scanner = vec![];
        } else {
            let beacon: Vec<i16> = line.split(',').map(|n| n.parse().unwrap()).collect();
            cur_scanner.push((beacon[0], beacon[1], beacon[2]));
        }
    }
    scanners.push(cur_scanner);

    scanners
}

fn rotate(scanner: &mut Scanner, i: usize) {
    for (x, y, z) in scanner {
        if i % 4 == 0 {
            *x *= -1;
            *z *= -1;
        }
        swap(y, z);
        *y *= -1;
        
        if i % 8 == 0 {
            swap(x, y);
            swap(y, z);
        }
    }
}

fn create_positions_map(left: &Scanner, right: &Scanner) -> HashMap<Position, usize>{
    let mut positions_map = HashMap::new();
    for left_beacon in left {
        for right_beacon in right {
            let key = (left_beacon.0 - right_beacon.0, left_beacon.1 - right_beacon.1, left_beacon.2 - right_beacon.2);
            let entry = positions_map.entry(key).or_insert(0);
            *entry += 1;
        }
    }
    positions_map
}

fn try_get_position(positions_map: HashMap<Position, usize>) -> Option<Position> {
    for (position, count) in positions_map {
        if count >= 12 {
            return Some(position);
        }
    }

    None
}

fn calculate_absolute_beacon_positions(scanner: &mut Scanner, pos: Position) {
    for (x, y, z) in scanner { 
        *x += pos.0; 
        *y += pos.1; 
        *z += pos.2; 
    }
}

fn collect_beacons(scanners: Vec<Scanner>) -> HashSet<Beacon> {
    let mut beacons = HashSet::new();

    for scanner in scanners {
        for beacon in scanner {
            beacons.insert(beacon);
        }
    }

    beacons
}

#[cfg(test)]
mod tests {
    use crate::day_19::try_get_position;

    use super::{parse_input, create_positions_map, rotate};

    #[test]
    fn test_parse_input() {
        let scanners = parse_input();
        assert_eq!(scanners.len(), 5);
        assert_eq!(scanners[0][0], (404, -588, -901));
        assert_eq!(scanners[1][1], (605, 423, 415));
        assert_eq!(scanners[2][2], (-784, 533, -524));
        assert_eq!(scanners[3][3], (-660, 373, 557));
        assert_eq!(scanners[4][25], (30, -46, -14));
    }

    #[test]
    fn test_positions_map() {
        let mut scanners = parse_input();
        scanners[1].iter_mut().for_each(|(x, _, z)| { *x *= -1; *z *= -1 });

        let map = create_positions_map(&scanners[0], &scanners[1]);
        let actual = try_get_position(map);

        assert!(actual.is_some());
        assert_eq!(actual.unwrap(), (68, -1246, -43));
    }

    #[test]
    fn test_rotate() {
        let mut scanner = vec![(1, 2, 3)];
        for i in 1..23 {
            rotate(&mut scanner, i);
            println!("{:?}", scanner[0]);
        }
    }
}