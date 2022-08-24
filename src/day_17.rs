use std::{collections::{HashSet, HashMap}, ops::Range};

pub fn get_solution_1() -> i32 {
    let y_range = -124..-69;
    let min_y = y_range.start + 1;
    min_y * (min_y + 1) / 2
}

pub fn get_solution_2() -> usize {
    let target_x = 211..232;
    let target_y = -124..-69; 
    let solve_vel_x_less_n = |delim: f64| (-0.5 + f64::sqrt(0.25 + 4. * delim * 0.5));
    let x_range_any = solve_vel_x_less_n(target_x.start.into()).ceil() as i32..solve_vel_x_less_n((target_x.end + 1).into()).ceil() as i32;

    let mut vel_xs_for_steps_map = HashMap::new();
    let mut count = 0;
    for vel_y in get_y_vels(target_y.start, target_y.end) {
        let mut vel_x_set = HashSet::new();
        for n in determine_n_steps_to_target(vel_y.into(), target_y.end.into(), target_y.start.into()) {
            let vel_xs = vel_xs_for_steps_map
                            .entry(n)
                            .or_insert(
                                determine_x_vels_from_n_steps(n.into(), target_x.start.into(), target_x.end.into(), &x_range_any)
                                    .collect::<HashSet<_>>()
                            );
            vel_x_set = vel_x_set.union(&vel_xs).map(|x| *x).collect();
        }
        if vel_x_set.len() > 0 {
            count += vel_x_set.len();
        }
    }

    count + ((target_y.end - target_y.start + 1) * (target_x.end - target_x.start + 1)) as usize
}

/// Returns an upper and lower bound for velocities in y direction which could hit the target area
fn get_y_vels(min_y_range: i32, max_y_range: i32) -> Range<i32> {
    max_y_range + 1..min_y_range.abs()
}

/// Calculates the number of steps to hit the target area, for a given velocity in y direction,
/// If the given velocity doesn't hit the target, steps_min will be equal to steps_max
fn determine_n_steps_to_target(vel_y: f64, max_y: f64, min_y: f64) -> Range<i32> {

    let calc_high_to_target = |vel: f64, max: f64| {
        let peak_y = if vel > 0. { vel * (vel + 1.) / 2.} else { (vel.abs() - 1.) * vel.abs() / 2. };
        let neg_offset = if vel >= 0. { 0. } else { vel.abs() - 1.};
        f64::sqrt(0.25 - 2. * -(peak_y + max.abs())) - 0.5 - neg_offset
    };

    let start_to_high = if vel_y < 0. { 0 } else { vel_y as i32 + 1};
    let steps_min = start_to_high + calc_high_to_target(vel_y, max_y).ceil() as i32;
    let steps_max = start_to_high + calc_high_to_target(vel_y, min_y - 1.).ceil() as i32;
    steps_min..steps_max
}

fn determine_x_vels_from_n_steps(n: f64, min_x: f64, max_x: f64, range_any: &Range<i32>) -> Range<i32> {
    if n >= range_any.end as f64{
        return range_any.start..range_any.end;
    }

    fn solve_vel_x(delim: f64, n: f64) -> f64 { 
        (2. * delim + n.powi(2) - n) / (2. * n) 
    }

    let max_vel_x = solve_vel_x(max_x + 1., n).ceil() as i32;

    if n >= range_any.start as f64 {
        return range_any.start..max_vel_x
    }

    let min_vel_x = solve_vel_x(min_x, n).ceil() as i32;

    min_vel_x..max_vel_x
}

/// Helper function for verifying if calculated velocities actually land in the target area
fn _simulate((mut vel_x, mut vel_y): (i32, i32), range_x: Range<i32>, range_y: Range<i32>) -> usize {
    let mut pos = (0, 0);
    let mut steps = 0;

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

    use super::{determine_n_steps_to_target, determine_x_vels_from_n_steps};

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
        let actual = determine_n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 1..2);

        let y_vel = -5.;
        let actual = determine_n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 1..2);

        let y_vel = -4.;
        let actual = determine_n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 2..3);

        let y_vel = -3.;
        let actual = determine_n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 2..3);

        let y_vel = -2.;
        let actual = determine_n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 2..4);

        let y_vel = -1.;
        let actual = determine_n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 3..5);

        let y_vel = 0.;
        let actual = determine_n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 4..6);

        let y_vel = 1.;
        let actual = determine_n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 5..7);

        let y_vel = 2.;
        let actual = determine_n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 7..8);

        let y_vel = 3.;
        let actual = determine_n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 9..10);

        let y_vel = 4.;
        let actual = determine_n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 10..11);

        let y_vel = 5.;
        let actual = determine_n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 12..13);

        let y_vel = 6.;
        let actual = determine_n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 14..15);

        let y_vel = 7.;
        let actual = determine_n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 16..17);

        let y_vel = 8.;
        let actual = determine_n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 18..19);

        let y_vel = 9.;
        let actual = determine_n_steps_to_target(y_vel, max_y, min_y);
        assert_eq!(actual, 20..21);

    }

    #[test]
    fn test_determine_n_x_vels() {
        let solve_vel_x_less_n = |delim: f64| (-0.5 + f64::sqrt(0.25 + 4. * delim * 0.5));
    
        let min_x = 20.;
        let max_x = 30.;

        let min_vel_x_less = solve_vel_x_less_n(min_x.into()).ceil() as i32;
        let max_vel_x_less = solve_vel_x_less_n(max_x + 1.).ceil() as i32;    
        let range_any = min_vel_x_less..max_vel_x_less;
        
        let one_actual = determine_x_vels_from_n_steps(1., min_x, max_x, &range_any).collect::<HashSet<_>>();
        let two_actual = determine_x_vels_from_n_steps(2., min_x, max_x, &range_any).collect::<HashSet<_>>();
        let three_actual = determine_x_vels_from_n_steps(3., min_x, max_x, &range_any).collect::<HashSet<_>>();
        let four_actual = determine_x_vels_from_n_steps(4., min_x, max_x, &range_any).collect::<HashSet<_>>();
        let five_actual = determine_x_vels_from_n_steps(5., min_x, max_x, &range_any).collect::<HashSet<_>>();
        let six_actual = determine_x_vels_from_n_steps(6., min_x, max_x, &range_any).collect::<HashSet<_>>();
        let seven_actual = determine_x_vels_from_n_steps(7., min_x, max_x, &range_any).collect::<HashSet<_>>();
        let nine_actual = determine_x_vels_from_n_steps(9., min_x, max_x, &range_any).collect::<HashSet<_>>();
        let ten_actual = determine_x_vels_from_n_steps(10., min_x, max_x, &range_any).collect::<HashSet<_>>();
        let twelve_actual = determine_x_vels_from_n_steps(12., min_x, max_x, &range_any).collect::<HashSet<_>>();

        
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