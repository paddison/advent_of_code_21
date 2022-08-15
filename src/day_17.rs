use std::collections::HashSet;

type TargetArea<'a> = (&'a HashSet<isize>, &'a HashSet<isize>);

struct Probe {
    pos: (isize, isize),
    x_vel: isize,
    y_vel: isize,
}

impl Probe {
    fn new(x_vel: isize, y_vel: isize) -> Self {
        Probe { pos: (0, 0), x_vel, y_vel }
    }
    
    fn advance(&mut self) {
        self.pos = (self.pos.0 + self.x_vel, self.pos.1 + self.y_vel);
        
        if self.x_vel < 0 {
            self.x_vel += 1;
        } else if self.x_vel > 0 {
            self.x_vel -= 1;
        } 
        
        self.y_vel -= 1;
    } 

    fn get_max_y(&mut self) -> isize {
        assert!(self.pos.0 == 0 && self.pos.1 == 0);
        if self.y_vel <= 0 {
            self.pos.1
        } else {
            self.y_vel * (self.y_vel + 1) / 2
        }
    }

    fn wont_hit_target(&self, (x_range, y_range): TargetArea) -> bool {
        let max_x = x_range.iter().max().unwrap();
        let max_y = y_range.iter().max().unwrap();
        let min_x = x_range.iter().min().unwrap();
        self.pos.0 > *max_x || self.pos.1 > *max_y || (self.x_vel == 0 && self.pos.0 < *min_x)
    }

    fn hits_target(&self, (x_range, y_range): TargetArea) -> bool {
        x_range.contains(&self.pos.0) && y_range.contains(&self.pos.1)
    }

    fn advance_n_steps(&mut self, n: isize) {
        let sum = |n| n * (n + 1) / 2;
        let old_x_vel = self.x_vel;

        let new_x = if old_x_vel == 0 {
            self.pos.0
        } else if n >= old_x_vel {
            self.x_vel = 0;
            self.pos.0 + sum(old_x_vel)
        } else {
            self.x_vel -= n;
            self.pos.0 + sum(old_x_vel) - sum(old_x_vel - n)
        };

        let old_y_vel = self.y_vel;

        let new_y = if old_y_vel <= 0 {
            // just subtract
            self.pos.1 - sum(-(old_y_vel - n)) + sum(-old_y_vel) 
        } else if n > old_y_vel { // here velocity is always positive
            self.pos.1 + sum(old_y_vel) - sum(n - old_y_vel - 1)
        } else {
            // add like with x_vel
            self.pos.1 + sum(old_y_vel) - sum(old_y_vel - n)
        };
        self.y_vel -= n;

        self.pos = (new_x, new_y);
    }
}

pub fn get_solution_1() -> i32 {
    
    let x_range = (211..232).collect::<HashSet<_>>();
    let y_range = (-124..-69).collect::<HashSet<_>>();
    let min_y = y_range.iter().min().unwrap();
    let max_y_vel = (min_y + 1) * -1;
    min_y * (min_y + 1) / 2
}

pub fn get_solution_2() {
    let min_x_range = 211;
    let max_x_range = 232;
    let min_y_range = -124;
    let max_y_range = -69;
    let max_y_vel = (min_y_range + 1) * -1;
    // since 124 - 69 = 55 < 69, there is a range of y_vel values between 123 and -123 where we overshoot after the last time before the target area.
    let range_y_min = (min_y_range * -1) - 2; // end up one before target area
    let range_y_max = (max_y_range * -1 - 1) / 2; // end up before target are so that |y_vel| + 2 + |y_vel| + 1 = |min_y_range| + 1

    // calculate all y_vels where it is possible to hit the target area:
    let pos_y_vels = get_x_vels(max_y_vel, range_y_max, range_y_min);
}

fn get_x_vels(max_y_vel: i32, range_y_max: i32, range_y_min: i32) -> Vec<i32> {
    (max_y_vel * -1..max_y_vel)
        .collect::<HashSet<i32>>()
        .difference(&((range_y_min..range_y_max).collect()))
        .map(|x| *x)
        .collect::<HashSet<i32>>()
        .difference(&(range_y_max * -1..range_y_min * - 1).collect())
        .map(|x| *x)
        .collect()
}


#[cfg(test)]
pub mod tests {
    use std::collections::HashSet;

    use super::Probe;

    fn create_test_range() -> (HashSet<isize>, HashSet<isize>) {
        ((20..30).collect::<HashSet<_>>(), (-10..-5).collect::<HashSet<_>>())
    }

    #[test]
    fn test_probe_advance() {
        let mut probe = Probe::new(7, 2);

        probe.advance();
        assert_eq!(probe.pos, (7, 2));

        probe.advance();
        assert_eq!(probe.pos, (13, 3));

        probe.advance();
        assert_eq!(probe.pos, (18, 3));

        probe.advance();
        assert_eq!(probe.pos, (22, 2));

        probe.advance();
        assert_eq!(probe.pos, (25, 0));

        probe.advance();
        assert_eq!(probe.pos, (27, -3));
        
        probe.advance();
        assert_eq!(probe.pos, (28, -7));


    }

    #[test]
    fn test_probe_hits_target() {
        let mut probe = Probe::new(7, 2);
        let (x_range, y_range) = create_test_range();

        probe.advance();
        assert!(!probe.hits_target((&x_range, &y_range)));

        probe.advance();
        assert!(!probe.hits_target((&x_range, &y_range)));

        probe.advance();
        assert!(!probe.hits_target((&x_range, &y_range)));

        probe.advance();
        assert!(!probe.hits_target((&x_range, &y_range)));

        probe.advance();
        assert!(!probe.hits_target((&x_range, &y_range)));

        probe.advance();
        assert!(!probe.hits_target((&x_range, &y_range)));
        
        probe.advance();
        assert!(probe.hits_target((&x_range, &y_range)));

    }

    #[test]
    fn test_advance_n_steps() {
        let mut probe = Probe::new(7, 2);

        probe.advance_n_steps(4);

        assert_eq!(probe.pos, (22, 2));
    }
}

