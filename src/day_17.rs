use std::{collections::{HashSet, HashMap}, ops::Range};

pub fn get_solution_1() -> i32 {
    
    let x_range = (211..232).collect::<HashSet<_>>();
    let y_range = (-124..-69).collect::<HashSet<_>>();
    let min_y = y_range.iter().min().unwrap();
    let max_y_vel = (min_y + 1) * -1;
    min_y * (min_y + 1) / 2
}

pub fn get_solution_2() -> usize{
    let min_x = 211;
    let max_x = 232;
    let min_y = -124;
    let max_y = -69;
    let solve_vel_x_less_n = |delim: f64| (-0.5 + f64::sqrt(0.25 + 4. * delim * 0.5));
    let x_range_any = (solve_vel_x_less_n(min_x.into()).ceil(), solve_vel_x_less_n((max_x + 1).into()).ceil());
    let max_y_vel = (min_y + 1) * -1;
    // since 124 - 69 = 55 < 69, there is a range of y_vel values between 123 and -123 where we overshoot after the last time before the target area.

    // calculate all y_vels where it is possible to hit the target area:
    let mut vel_y_map = HashMap::new();
    let mut range_map = HashMap::new();
    for vel in get_y_vels(min_y, max_y) {
        let mut x_set = HashSet::new();
        for n in n_steps_to_target(vel.into(), max_y.into(), min_y.into()) {
            let x_vals = range_map.entry(n).or_insert(determine_n_x_vels(n.into(), min_x.into(), max_x.into(), x_range_any).collect::<HashSet<_>>());
            x_set = x_set.union(&x_vals).map(|x| *x).collect();
        }
        vel_y_map.insert(vel, x_set);
    }
    let mut count = 0;
    for (vel_y, vel_xs) in vel_y_map {
        count += vel_xs.len();
        for vel_x in vel_xs {
            match simulate((vel_x, vel_y), min_x..max_x + 1, min_y..max_y + 1) {
                0 => eprintln!("Didn't hit target with velocity: ({}, {})", vel_x, vel_y),
                steps => (), //println!("Hit target with velocity: ({}, {}) after {} steps.", vel_x, vel_y, steps),
            }
        }
    }

    count
}

// TODO: Ignore all y velocities which end up in target are y range after first step
// all combinations that exist in that case can be calculated simply by 
// (max_x_range - min_x_range) * (max_y_range - min_y_range)
// these are all the y_velocities which are in min_y_range to max_y_range
fn get_y_vels(min_y_range: i32, max_y_range: i32) -> Vec<i32> {
    // since 124 - 69 = 55 < 69, there is a range of y_vel values between 123 and -123 where we overshoot after the last time before the target area.
    let min_y_range = min_y_range.abs();
    let max_y_range = max_y_range.abs();
    let max_y_vel = min_y_range;
    let range_y_max = max_y_range - 2; // end up one before target area
    let range_y_min = (min_y_range - 1) / 2; // end up before target are so that |y_vel| + 2 + |y_vel| + 1 = |min_y_range| + 1

    // todo, verify if this is correct
    (min_y_range * -1..max_y_vel) // ((max_y_range - 2) * -1..max_y_vel) for optimizing if we really only calculate the number of combinations
        .collect::<Vec<i32>>()
        // .difference(&((range_y_min - 1..=range_y_max - 1).collect()))
        // .map(|x| *x)
        // .collect::<HashSet<i32>>()
        // .difference(&(range_y_max * -1..=range_y_min * - 1).collect())
        // .map(|x| *x)
        // .collect()
}

/// Calculates the number of steps to hit the target area, for a given velocity in y direction,
/// which is known to be able to hit the target area
fn n_steps_to_target(vel_y: f64, max_y: f64, min_y: f64) -> Range<i32> {

    let calc_high_to_target = |vel: f64, max: f64| {
        let high_y = if vel > 0. { vel * (vel + 1.) / 2.} else { (vel.abs() - 1.) * vel.abs() / 2. };
        let neg_offset = if vel >= 0. { 0. } else { vel.abs() - 1.};
        // let max = if vel < 0. { max.abs() + (vel.abs() - 1.) * vel.abs() / 2. } else { max.abs() };
        f64::sqrt(0.25 - 2. * -(high_y + max.abs())) - 0.5 - neg_offset
    };

    let start_to_high = if vel_y < 0. { 0 } else { vel_y as i32 + 1};
    let steps_min = start_to_high + calc_high_to_target(vel_y, max_y).ceil() as i32;
    let steps_max = start_to_high + calc_high_to_target(vel_y, min_y - 1.).ceil() as i32;
    steps_min..steps_max
}

fn x_distance_after_n_steps(n: i32, x_vel: i32) -> i32 {
    n * x_vel - (n.pow(2) - n) / 2
} 

fn determine_n_x_vels(n: f64, min_x: f64, max_x: f64, range_any: (f64, f64)) -> Range<i32> {
    if n >= range_any.1 {
        return range_any.0 as i32..range_any.1 as i32; // maybe only call function if n < range_any.1 ?
    }
    fn solve_vel_x(delim: f64, n: f64) -> f64 { // find out difference between inner function and closure
        (2. * delim + n.powi(2) - n) / (2. * n) 
    }

    let max_vel_x = solve_vel_x(max_x + 1., n).ceil() as i32;

    if n >= range_any.0 {
        return range_any.0 as i32..max_vel_x
    }

    let min_vel_x = solve_vel_x(min_x, n).ceil() as i32;

    min_vel_x..max_vel_x
}

fn simulate((mut vel_x, mut vel_y): (i32, i32), range_x: Range<i32>, range_y: Range<i32>) -> usize {
    let mut pos = (0, 0);
    let mut steps = 0;
    if vel_y == -68 {
        println!("vel is 68");
    }
    while pos.0 < range_x.end || pos.1 > range_y.end {
        pos = (pos.0 + vel_x, pos.1 + vel_y);
        steps += 1;
        if range_x.contains(&pos.0) && range_y.contains(&pos.1) {
            return steps
        }
        vel_y -= 1;
        if vel_x > 0 { 
            vel_x -= 1;
        }
    }

    0
}


#[cfg(test)]
pub mod tests {
    use std::{collections::HashSet, ops::Range};

    use super::{get_y_vels, n_steps_to_target, x_distance_after_n_steps, determine_n_x_vels};

    fn create_test_range() -> (HashSet<i32>, HashSet<i32>) {
        ((20..30).collect::<HashSet<_>>(), (-10..-8).collect::<HashSet<_>>())
    }

    #[test]
    fn test_get_y_vels() {
        let y_range = create_test_range().1;
        let min_y_range = *y_range.iter().min().unwrap();
        let max_y_range = *y_range.iter().max().unwrap();

        let mut actual = get_y_vels(min_y_range, max_y_range);
        actual.sort();
        let expected = vec![-10, -9, -8, -3, -2, -1, 0, 1, 2, 7, 8, 9];

        assert_eq!(actual, expected);

        let mut actual_test = get_y_vels(-10, -5);
        actual_test.sort();
        let expected_test = vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(actual_test, expected_test);

    }

    #[test]
    fn test_x_vels() {
        let split_to_tuple = |s: &str| { 
            let split: Vec<&str> = s.split(',').collect(); 
            assert_eq!(split.len(), 2);
            (split[0].parse::<i32>().unwrap(), split[1].parse::<i32>().unwrap())
        };
        let mut tuples: Vec<(i32, i32)> = include_str!("../data/day_17_sol.txt")
                                    .split_ascii_whitespace()
                                    .map(split_to_tuple)
                                    .collect();
        tuples.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        tuples.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        println!("{:?}", tuples)
    }

    #[test]
    fn test_n_steps_to_target() {
        let max_y = -5.;
        let min_y = -10.;

        let y_vel = -10.;
        let actual = n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 1..2);

        let y_vel = -5.;
        let actual = n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 1..2);

        let y_vel = -4.;
        let actual = n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 2..3);

        let y_vel = -3.;
        let actual = n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 2..3);

        let y_vel = -2.;
        let actual = n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 2..4);

        let y_vel = -1.;
        let actual = n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 3..5);

        let y_vel = 0.;
        let actual = n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 4..6);

        let y_vel = 1.;
        let actual = n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 5..7);

        let y_vel = 2.;
        let actual = n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 7..8);

        let y_vel = 3.;
        let actual = n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 9..10);

        let y_vel = 4.;
        let actual = n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 10..11);

        let y_vel = 5.;
        let actual = n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 12..13);

        let y_vel = 6.;
        let actual = n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 14..15);

        let y_vel = 7.;
        let actual = n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 16..17);

        let y_vel = 8.;
        let actual = n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 18..19);

        let y_vel = 9.;
        let actual = n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 20..21);

    }

    #[test]
    fn test_x_distance_after_n_steps() {
        let actual = x_distance_after_n_steps(2, 7);
        assert_eq!(actual, 7 + 6);

        let actual = x_distance_after_n_steps(3, 5);
        assert_eq!(actual, 5 + 4 + 3);
    
    }

    #[test]
    fn test_determine_n_x_vels() {
        let solve_vel_x_less_n = |delim: f64| (-0.5 + f64::sqrt(0.25 + 4. * delim * 0.5));
    
        let min_x = 20.;
        let max_x = 30.;

        let min_vel_x_less = solve_vel_x_less_n(min_x.into()).ceil();
        let max_vel_x_less = solve_vel_x_less_n(max_x + 1.).ceil();    
        let range_any = (min_vel_x_less, max_vel_x_less);
        
        let one_actual = determine_n_x_vels(1., min_x, max_x, range_any).collect::<HashSet<_>>();
        let two_actual = determine_n_x_vels(2., min_x, max_x, range_any).collect::<HashSet<_>>();
        let three_actual = determine_n_x_vels(3., min_x, max_x, range_any).collect::<HashSet<_>>();
        let four_actual = determine_n_x_vels(4., min_x, max_x, range_any).collect::<HashSet<_>>();
        let five_actual = determine_n_x_vels(5., min_x, max_x, range_any).collect::<HashSet<_>>();
        let six_actual = determine_n_x_vels(6., min_x, max_x, range_any).collect::<HashSet<_>>();
        let seven_actual = determine_n_x_vels(7., min_x, max_x, range_any).collect::<HashSet<_>>();
        let nine_actual = determine_n_x_vels(9., min_x, max_x, range_any).collect::<HashSet<_>>();
        let ten_actual = determine_n_x_vels(10., min_x, max_x, range_any).collect::<HashSet<_>>();
        let twelve_actual = determine_n_x_vels(12., min_x, max_x, range_any).collect::<HashSet<_>>();

        
        // n = 1
        assert_eq!(one_actual, HashSet::from([20, 21, 22, 23, 24 ,25, 26, 27, 28, 29, 30]));
        // n = 2
        assert_eq!(two_actual, HashSet::from([11, 12, 13, 14, 15]));
        // n = 2, 3
        assert_eq!(two_actual.union(&three_actual).map(|x| *x).collect::<HashSet<_>>(), HashSet::from([8, 9, 10, 11, 12, 13, 14, 15]));
        // n = 3, 4
        assert_eq!(three_actual.union(&four_actual).map(|x| *x).collect::<HashSet<_>>(), HashSet::from([7, 8, 9, 10, 11]));
        // n = 4, 5
        assert_eq!(four_actual.union(&five_actual).map(|x| *x).collect::<HashSet<_>>(), HashSet::from([6, 7, 8, 9]));
        // n = 5, 6
        assert_eq!(five_actual.union(&six_actual).map(|x| *x).collect::<HashSet<_>>(), HashSet::from([6, 7, 8]));
        // n = 7
        assert_eq!(seven_actual, HashSet::from([6, 7]));
        // n = 9
        assert_eq!(nine_actual, HashSet::from([6, 7]));
        // n = 10
        assert_eq!(ten_actual, HashSet::from([6, 7]));

        assert_eq!(twelve_actual, HashSet::from([6, 7]));

    }

    #[test]
    fn test_correct_x_vels() {
        fn determine_n_x_vels_dbg(n: f64, min_x: f64, max_x: f64) -> (Range<i32>, Range<i32>) {
            let solve_vel_x = |delim: f64, n: f64| (2. * delim + n.powi(2) - n) / (2. * n);
            let solve_vel_x_less_n = |delim: f64| (-0.5 + f64::sqrt(0.25 + 4. * delim * 0.5));
            let min_vel_x = solve_vel_x(min_x, n).ceil() as i32;
            let max_vel_x = solve_vel_x(max_x + 1., n).ceil() as i32;
        
            let min_vel_x_less = solve_vel_x_less_n(min_x).ceil() as i32;
            let max_vel_x_less = solve_vel_x_less_n(max_x + 1.).ceil() as i32;
            // max_vel_x - min_vel_x
            (min_vel_x..max_vel_x, min_vel_x_less..max_vel_x_less)
        }
        let min_x = 10;
        let max_x = 30;
        
        let one_step = determine_n_x_vels_dbg(1., min_x.into(), max_x.into());
        println!("1 {:?}", one_step);

        let one_step = determine_n_x_vels_dbg(2., min_x.into(), max_x.into());
        println!("2 {:?}", one_step);

        let one_step = determine_n_x_vels_dbg(3., min_x.into(), max_x.into());
        println!("3 {:?}", one_step);

        let four_steps = determine_n_x_vels_dbg(4., min_x.into(), max_x.into());
        println!("4 {:?}", four_steps);

        let five_steps = determine_n_x_vels_dbg(5., min_x.into(), max_x.into());
        println!("5 {:?}", five_steps);

        let five_steps = determine_n_x_vels_dbg(6., min_x.into(), max_x.into());
        println!("6 {:?}", five_steps);

        let five_steps = determine_n_x_vels_dbg(7., min_x.into(), max_x.into());
        println!("7 {:?}", five_steps);

        let five_steps = determine_n_x_vels_dbg(8., min_x.into(), max_x.into());
        println!("8 {:?}", five_steps);

        let five_steps = determine_n_x_vels_dbg(9., min_x.into(), max_x.into());
        println!("9 {:?}", five_steps);

        let five_steps = determine_n_x_vels_dbg(10., min_x.into(), max_x.into());
        println!("10 {:?}", five_steps);

        let five_steps = determine_n_x_vels_dbg(12., min_x.into(), max_x.into());
        println!("12 {:?}", five_steps);

        let five_steps = determine_n_x_vels_dbg(14., min_x.into(), max_x.into());
        println!("14 {:?}", five_steps);

        let five_steps = determine_n_x_vels_dbg(16., min_x.into(), max_x.into());
        println!("16 {:?}", five_steps);

        let five_steps = determine_n_x_vels_dbg(18., min_x.into(), max_x.into());
        println!("18 {:?}", five_steps);

        let twenty_steps = determine_n_x_vels_dbg(20., min_x.into(), max_x.into());
        println!("20 {:?}", twenty_steps);
    }
}


// (20, -10), (21, -10), (22, -10), (23, -10), (24, -10), (25, -10), (26, -10), (27, -10), (28, -10), (29, -10), (30, -10), n = 1
// (20, -9), (21, -9), (22, -9), (23, -9), (24, -9), (25, -9), (26, -9), (27, -9), (28, -9), (29, -9), (30, -9),            n = 1
// (20, -8), (21, -8), (22, -8), (23, -8), (24, -8), (25, -8), (26, -8), (27, -8), (28, -8), (29, -8), (30, -8),            n = 1
// (20, -7), (21, -7), (22, -7), (23, -7), (24, -7), (25, -7), (26, -7), (27, -7), (28, -7), (29, -7), (30, -7),            n = 1
// (20, -6), (21, -6), (22, -6), (23, -6), (24, -6), (25, -6), (26, -6), (27, -6), (28, -6), (29, -6), (30, -6),            n = 1
// (20, -5), (21, -5), (22, -5), (23, -5), (24, -5), (25, -5), (26, -5), (27, -5), (28, -5), (29, -5), (30, -5),            n = 1
// (11, -4), (12, -4), (13, -4), (14, -4), (15, -4),                                                                        n = 2
// (11, -3), (12, -3), (13, -3), (14, -3), (15, -3),                                                                        n = 2
// (8, -2), (9, -2), (10, -2), (11, -2), (12, -2), (13, -2), (14, -2), (15, -2),                                            n = 2, 3
// (7, -1), (8, -1), (9, -1), (10, -1), (11, -1),                                                                           n = 3, 4
// (6, 0), (7, 0), (8, 0), (9, 0),                                                                                          n = 4, 5
// (6, 1), (7, 1), (8, 1),                                                                                                  n = 5, 6
// (6, 2), (7, 2),                                                                                                          n = 7
// (6, 3), (7, 3),                                                                                                          n = 9
// (6, 4), (7, 4),                                                                                                          n = 10
// (6, 5), (7, 5),                                                                                                          n = 12
// (6, 6), (7, 6),                                                                                                          n = 14
// (6, 7), (7, 7),                                                                                                          n = 16
// (6, 8), (7, 8),                                                                                                          n = 18
// (6, 9), (7, 9)                                                                                                           n = 20

// ...............................
// S........................#.....
// ...............................
// ...............................
// ...........................#...
// ...............................
// ....................TTTTTTTTTTT
// ....................TTTTTTTTTTT
// ....................TTTTTTTT#TT
// ....................TTTTTTTTTTT
// ....................TTTTTTTTTTT
// ....................TTTTTTTTTTT