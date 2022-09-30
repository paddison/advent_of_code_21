use std::{ops::{Deref, Index}, fmt::{Display, Formatter, Write}, path, collections::{VecDeque, HashSet, HashMap}, env::home_dir};

const BOARD_WIDTH: usize = 13;
const ALL_POSITIONS: [(usize, usize); 15] = [
    (0, 0), (1, 0), (3, 0), (5, 0), (7, 0), (9, 0), (10, 0),
    (2, 1), (4, 1), (6, 1), (8, 1), 
    (2, 2), (4, 2), (6, 2), (8, 2), 
];

use PodType::*;

fn create_actual_data() -> Pods {
    vec![
        Amphipod::new(Amber, (4, 2)), Amphipod::new(Amber, (4, 1)),Amphipod::new(Desert, (8, 1)), Amphipod::new(Copper, (2, 1)),  Amphipod::new(Bronze, (6, 1)),
        Amphipod::new(Copper, (2, 2)), Amphipod::new(Desert, (6, 2)), Amphipod::new(Bronze, (8, 2)),
    ]
}

pub fn get_solution_1() -> usize {
    let initial = create_actual_data();
    Burrow::organize_pods(initial, vec![])
}

fn get_input() -> &'static str {
    include_str!("../data/day_23.txt")
}

// Board Representation:
// #############
// #...........#
// ###B#C#B#D###
//   #A#D#C#A#
//   #########
// Hallway positions are from 0-10
// Four home types: AmberHome, Bronze ...
// Valid Positions for Pods in the hallway to stay are 0,1,3,5,7,9,10
// A pod is at home if it is in its own home, and below it are only pods of the same type
// Home Representation Could be like { PodType, [Option<Pod>; 2] }
// another way is to store x value of home on podtype enum

// the position of a pod can be stored as a tuple

// Another approach is to store different sets of pods
// A pod after moving, will either be in the hallyway or at home.
// If a pod hasn't moved yet, it will still be in one of the homes
// another way to determine if a pod is home, is to check it's position and see if any pods that haven't moved yet share the same x position 
// a pod that has moved can only move again if it can get home 

// how to determine if pod can move home: first check if home is free, then check if path to home is free
// home is free if there are no pods or only pods of the same type in it

type Pods = Vec<Amphipod>;

#[derive(Clone)]
struct Burrow {
    initial: Vec<Amphipod>,
    moved: Vec<Amphipod>,
    home: Vec<Amphipod>,
    cost: usize,
    heuristic: usize,
    neighbours: Option<Vec<Burrow>>,
}

fn print_burrow(initial: &Pods, moved: &Pods, home: &Pods) {
    let mut burrow_string = String::from(
"#############
#...........#
###.#.#.#.###
  #.#.#.#.#
  #########");
  for p in initial.iter().chain(moved).chain(home) {
    let index = 1 + p.pos.0 + (p.pos.1 + 1) * BOARD_WIDTH + (p.pos.1 + 1);
    burrow_string.replace_range(index..index + 1, p.p_type.into());
  }
  println!("{}", burrow_string);
}

impl Burrow {

    fn new(initial: Pods, moved: Pods, home: Pods, cost: usize) -> Self {
    
        let mut burrow = Burrow { initial, moved, home, cost, neighbours: None, heuristic: 0 };
        burrow.heuristic = burrow.min_cost_to_goal();
        burrow
    }

    fn is_organized(&self) -> bool {
        self.initial.is_empty() && self.moved.is_empty()
    }

    fn organize_pods(initial: Pods, home: Pods) -> usize {

        let mut start = Burrow::new(initial, vec![], home, 0);
        let mut visited = vec![start.clone()];
        start.add_neighbours();
        let mut burrows = vec![start];
        loop {
            let mut new_burrows = vec![];
            for (i, burrow) in burrows.iter().enumerate() {
                if burrow.is_organized() {
                    return burrow.cost;
                }

                // let mut count = 0;
                if let Some((j, min)) = burrow.neighbours.as_ref().unwrap().iter().enumerate().find(|(_, burrow)| !visited.contains(burrow)) {
                    new_burrows.push((i, j, min));

                }
            }

            let (i, j, _) = new_burrows.into_iter().min_by(|(_, _, a), (_, _, b)| (a.cost + a.heuristic).cmp(&(b.cost + b.heuristic))).unwrap();

            let mut min = burrows[i].neighbours.as_mut().unwrap().remove(j);
            // visited.push(min.clone()); 
            min.add_neighbours();
            // println!("{}", min);
            
            burrows.push(min);
        }
    }

    fn min_cost_to_goal(&self) -> usize {
        let mut pod_homes = HashMap::from([
            (Amber, vec![(2, 1), (2, 2)]),
            (Bronze, vec![(4, 1), (4, 2)]),
            (Copper, vec![(6, 1), (6, 2)]),
            (Desert, vec![(8, 1), (8, 2)]),
        ]);
        // filter homes:
        for pod in &self.home {
            let home =  pod_homes.get_mut(&pod.p_type).unwrap();
            let idx = home.iter().position(|pos| &pod.pos == pos).unwrap();
            home.remove(idx);
        }
        let mut cost = 0;
        for pod in &self.initial {
            let home = pod_homes.get_mut(&pod.p_type).unwrap();
            let home_pos = home.pop().unwrap();
            if home_pos.0 == pod.pos.0 {
                cost += (home_pos.1 + pod.pos.1 + 2) * pod.cost();
            } else {
                cost += pod.caclulate_move_cost(&home_pos);
            }
        }

        for pod in &self.moved {
            let home = pod_homes.get_mut(&pod.p_type).unwrap();
            let pos = home.pop().unwrap();
            cost += pod.caclulate_move_cost(&pos);
        }
        cost
    }

    // moves in burrow are always sorted by cost
    fn add_neighbours(&mut self) {
        if self.neighbours.is_some() {
            return
        }
        // first check if pods that haven't moved so far can get home (since this will lead to a minimal score)
        for (i, pod) in self.initial.iter().enumerate() {
            if let Some((cost, pos)) = pod.can_get_home(&self.initial, &self.moved, &self.home) {
                // remove from initial and add to home
                let mut new_initial = self.initial.clone();
                let mut new_home = self.home.clone();
                let mut home_pod = new_initial.remove(i);
                home_pod.pos = pos;
                new_home.push(home_pod);
                self.neighbours = Some(vec![Burrow::new(new_initial, self.moved.clone(), new_home, self.cost + cost)]);
                return
            }
        }

        // check if any pod that has moved can get home
        for (i, pod) in self.moved.iter().enumerate() {
            if let Some((cost, pos)) = pod.can_get_home(&self.initial, &self.moved, &self.home) {
                // remove from moved and add to home
                let mut new_moved = self.moved.clone();
                let mut new_home = self.home.clone();
                let mut home_pod = new_moved.remove(i);
                home_pod.pos = pos;
                new_home.push(home_pod);
                self.neighbours = Some(vec![Burrow::new(self.initial.clone(), new_moved, new_home, self.cost + cost)]);
                return 
            }
        }

        //another approach would be to get all possible moves and choose the cheapest one
        // let mut possible_moves = vec![];
        // for (i, pod) in initial.iter().enumerate() {
        //     for m in pod.get_possible_moves(&initial, &moved) {
        //         possible_moves.push((i, m));
        //     }
        // }
        // possible_moves.sort_by(|a, b| a.1.0.cmp(&b.1.0));
        // possible_moves.sort_by(|a, b| initial[a.0].cost().cmp(&initial[b.0].cost()));
        // for (i, (_, cost, position)) in possible_moves.into_iter() {
        //     let mut new_initial = initial.clone();
        //     let mut new_moved = moved.clone();

        //     let mut moved_pod = new_initial.remove(i);
        //     moved_pod.pos = position;

        //     new_moved.push(moved_pod);
        //     Self::do_move(new_initial, new_moved, home.clone(), total_cost + cost, results)
        // }  

        // move pod out of initial position
        let mut neighbours = Vec::new();
        for (i, pod) in self.initial.iter().enumerate() {
            // get possible moves needs to return possible moves sorted by cost
            for (cost, position) in pod.get_possible_moves(&self.initial, &self.moved).into_iter() {
                // assign pod new position and move pod from initial to moved
                let mut new_initial = self.initial.clone();
                let mut new_moved = self.moved.clone();

                let mut moved_pod = new_initial.remove(i);
                moved_pod.pos = position;

                new_moved.push(moved_pod);

                neighbours.push(Burrow::new(new_initial, new_moved, self.home.clone(), self.cost + cost));
            }
        }
        neighbours.sort_by(|a, b| (a.cost + a.heuristic).cmp(&(b.cost + b.heuristic)));
        self.neighbours = Some(neighbours);
    }

    fn contains(lhs: &Pods, rhs: &Pods) -> bool {
        if lhs.len() != rhs.len() {
            return false;
        }

        for pod in lhs {
            if !rhs.contains(pod) {
                return false;
            }
        }

        true
    }
}

impl Display for Burrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut burrow_string = String::from(
"#############
#...........#
###.#.#.#.###
  #.#.#.#.#
  #########");
              for p in self.initial.iter().chain(&self.moved).chain(&self.home) {
                let index = 1 + p.pos.0 + (p.pos.1 + 1) * BOARD_WIDTH + (p.pos.1 + 1);
                burrow_string.replace_range(index..index + 1, p.p_type.into());
              }
        write!(f, "{}", burrow_string)
    }
}

impl PartialEq for Burrow {
    fn eq(&self, other: &Self) -> bool {

        Burrow::contains(&self.initial, &other.initial) &&
        Burrow::contains(&self.moved, &other.moved) &&
        Burrow::contains(&self.home, &other.home) &&
        self.cost == other.cost
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Amphipod {
    p_type: PodType,
    pos: (usize, usize),
}

impl Amphipod {
    fn new(p_type: PodType, position: (usize, usize)) -> Self {
        Self {p_type, pos: position }
    }

    fn cost(&self) -> usize {
        **self as usize
    }

    // only gets called on pod that is in intial
    fn caclulate_move_cost(&self, position: &(usize, usize)) -> usize {
        let x_diff = self.pos.0.abs_diff(position.0);
        let y_diff = self.pos.1 + position.1;

        (x_diff + y_diff) * self.cost()
    }

    // only gets called on pod that is in intial
    fn get_possible_moves(&self, initial: &Pods, moved: &Pods) -> Vec<(usize, (usize, usize))> {
        // pod will always move into hallway
        // if pod is at y == 2, check if there is a pod above him, if so, return empty vec (pod cannot move)
        if self.pos.1 == 2 {
            if initial.iter().any(|pod| pod.pos == (self.pos.0, 1)) {
                return vec![]
            }
        }
        let mut right_positions = VecDeque::from([(0, 0), (1, 0), (3, 0), (5, 0), (7, 0), (9, 0), (10, 0)]);
        
        // get adjacent positions and verify if they are free
        // only pods in the moved set can be in the hallway
        let idx = right_positions.iter().position(|pos| pos.0 > self.pos.0).unwrap(); // unwrap is safe, since rooms are between 2 and 8 on x
        let mut left_positions = right_positions.drain(..idx).collect::<Vec<(usize, usize)>>();
        let mut possible_positions = vec![];

        while !right_positions.is_empty() || !left_positions.is_empty() {
            if let Some(left_pos) = left_positions.pop() {
                if moved.iter().any(|pod| pod.pos == left_pos) {
                    left_positions = vec![];
                } else {
                    possible_positions.push(left_pos);
                }
            }
            if let Some(right_pos) = right_positions.pop_front() {
                if moved.iter().any(|pod| pod.pos == right_pos) {
                    right_positions = VecDeque::new();
                } else {
                    possible_positions.push(right_pos);
                }
            }
        }
        let pod_home = self.get_pod_home();
        // let distance_from_home = |(x, y): (usize, usize)| (y + x.abs_diff(pod_home[0].0) + 1) * self.cost();

        let mut with_costs = possible_positions
                                .into_iter()
                                .map(|pos| (self.caclulate_move_cost(&pos), pos))
                                .collect::<Vec<(usize, (usize, usize))>>();

        // sorting is not strictly necessary (depends on approach used)
        with_costs.sort_by(|a, b| (a.0).cmp(&b.0));
        with_costs
    }

    fn get_pod_home(&self) -> [(usize, usize); 2] {
        match self.p_type {
            PodType::Amber => [(2, 1), (2, 2)],
            PodType::Bronze => [(4, 1), (4, 2)],
            PodType::Copper => [(6, 1), (6, 2)],
            PodType::Desert => [(8, 1), (8, 2)],
        }
    }

    // only gets called on pods that are in moved or initial
    fn can_get_home(&self, initial: &Pods, moved: &Pods, home: &Pods) -> Option<(usize, (usize, usize))> {
        // three steps:
        // 1. check if pod can move (if it is in initial)
        // 2. check if home is free
        // 3. check if hallway is free

        // check if pod can move:
        if self.pos.0 > 1 && initial.iter().any(|pod| pod.pos.0 == self.pos.0 && pod.pos.1 < self.pos.1) {
            return None;
        }

        // check if pod_home is free
        let pod_home = self.get_pod_home();

        // only pods in initial or home may be in pod_home
        let other_pods_in_home = initial.into_iter()
                                        .chain(home)
                                        .filter(|pod| pod.pos == pod_home[0] || pod.pos == pod_home[1])
                                        .collect::<Vec<&Amphipod>>(); 

        // check if home is free
        let mut home_pos = pod_home[1];

        for pod in &other_pods_in_home {
            if self.p_type != pod.p_type {
                return None
            } else {
                home_pos = (home_pos.0, home_pos.1 - 1)
            }
        }  

        // check if there are any pods on path to pod_home in hallway
        let lower_bound = self.pos.0.min(home_pos.0);
        let upper_bound = self.pos.0.max(home_pos.0);

        if moved.iter().any(|pod| pod.pos.0 < upper_bound && pod.pos.0 > lower_bound) {
            return None
        }

        Some((self.caclulate_move_cost(&home_pos), home_pos))
    }           
}

impl Deref for Amphipod {
    type Target = PodType;

    fn deref(&self) -> &Self::Target {
        &self.p_type
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       let pod = match self.p_type {
        PodType::Amber => "A",
        PodType::Bronze => "B",
        PodType::Copper => "C",
        PodType::Desert => "D",
    };

        write!(f, "{}", pod)
    }
}

impl TryFrom<(usize, char)> for Amphipod {
    type Error = &'static str;

    fn try_from(value: (usize, char)) -> Result<Self, Self::Error> {
        todo!()
    }
} 

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
enum PodType {
    Amber = 1,
    Bronze = 10,
    Copper = 100,
    Desert = 1000,
}

impl From<char> for PodType {
    fn from(c: char) -> Self {
        match c {
            'A' => PodType::Amber,
            'B' => PodType::Bronze,
            'C' => PodType::Copper,
            'D' => PodType::Desert,
            _ => panic!("Invalid Char for Podtype")
        }
    }
}

impl From<PodType> for &str {
    fn from(pod: PodType) -> Self {
        match pod {
            PodType::Amber => "A",
            PodType::Bronze => "B",
            PodType::Copper => "C",
            PodType::Desert => "D",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::ALL_POSITIONS;
    use super::Amphipod;
    use super::Burrow;
    use super::PodType::*;
    use super::Pods;
    use super::print_burrow;

    fn get_test_input() -> &'static str{
        include_str!("../data/day_23_test.txt")
    }

    // do this manually
    fn create_test_data() -> (Pods, Pods) {
        (
            vec![
                Amphipod::new(Bronze, (6, 1)), Amphipod::new(Bronze, (2, 1)), Amphipod::new(Copper, (4, 1)), Amphipod::new(Desert, (8, 1)),
                Amphipod::new(Desert, (4, 2)), Amphipod::new(Amber, (8, 2))
            ],
            vec![Amphipod::new(Amber, (2, 2)), Amphipod::new(Copper, (6, 2))],
        )
    }

    fn create_actual_data() -> Pods {
        vec![
            Amphipod::new(Copper, (2, 1)), Amphipod::new(Amber, (4, 1)), Amphipod::new(Bronze, (6, 1)), Amphipod::new(Desert, (8, 1)),
            Amphipod::new(Copper, (2, 2)), Amphipod::new(Amber, (4, 2)), Amphipod::new(Desert, (6, 2)), Amphipod::new(Bronze, (8, 2)),
        ]
    }

    // helper function to prepare results for comparison
    fn map_to_position(moves: Vec<(usize, (usize, usize))>) -> Vec<(usize, usize)> {
        let mut moves = moves.into_iter().map(|(_, pos)| pos).collect::<Vec<(usize, usize)>>();
        moves.sort_by(|(ax, _), (bx, _)| ax.cmp(&bx));
        moves
    }

    #[test]
    fn test_amphi_cost() {
        let (amber, bronze, copper, desert) = (Amphipod::new(Amber, (0, 0)), Amphipod::new(Bronze, (0, 0)), Amphipod::new(Copper, (0, 0)), Amphipod::new(Desert, (0, 0)));
        assert_eq!(amber.cost(), 1);
        assert_eq!(bronze.cost(), 10);
        assert_eq!(copper.cost(), 100);
        assert_eq!(desert.cost(), 1000);
    }

    #[test]
    fn test_print_burrow() {
        let initial = vec![
            Amphipod::new(Amber, (0, 0)),
            Amphipod::new(Amber, (2, 2)),
            Amphipod::new(Bronze, (2, 1)),
            Amphipod::new(Bronze, (4, 2)),
            Amphipod::new(Copper, (3, 0)),
            Amphipod::new(Copper, (5, 0)),
            Amphipod::new(Desert, (8, 1)),
            Amphipod::new(Desert, (10, 0)),
            ];
        
        print_burrow(&initial, &vec![], &vec![]);
    }

    #[test]
    fn test_calculate_move_costs() {
        let bronze = Amphipod::new(Bronze, (0, 0)); // right
        let copper = Amphipod::new(Copper, (2, 2)); // up right
        let desert = Amphipod::new(Desert, (5, 0)); // right down
        let amber = Amphipod::new(Amber, (8, 2));   // up left down

        let right = (9, 0);
        let up_right = (3, 0);
        let right_down = (8, 1);
        let up_left_down = (2, 2);

        assert_eq!(bronze.caclulate_move_cost(&right), 90);
        assert_eq!(copper.caclulate_move_cost(&up_right), 300);
        assert_eq!(desert.caclulate_move_cost(&right_down), 4000);
        assert_eq!(amber.caclulate_move_cost(&up_left_down), 10);
    }

    #[test]
    fn test_get_possible_moves() {
        let test_pod = Amphipod::new(Amber, (6, 2));
        
        // no other pods, should return all possible moves
        assert_eq!(
            map_to_position(test_pod.get_possible_moves(&vec![], &vec![])), 
            vec![(0, 0), (1, 0), (3, 0), (5, 0), (7, 0), (9, 0), (10, 0)]
        );

        // above is no space, should not return any moves
        let initial = vec![Amphipod::new(Desert, (6, 1))];
        assert_eq!(test_pod.get_possible_moves(&initial, &vec![]), vec![]);

        // only left positions are free
        let moved = vec![Amphipod::new(Desert, (7, 0))];
        assert_eq!(
            map_to_position(test_pod.get_possible_moves(&vec![], &moved)),
            vec![(0, 0), (1, 0), (3, 0), (5, 0)]
        );

        // only right positions are free
        let moved = vec![Amphipod::new(Desert, (5, 0))];
        assert_eq!(
            map_to_position(test_pod.get_possible_moves(&vec![], &moved)),
            vec![(7, 0), (9, 0), (10, 0)]
        );

        // some left and some right are free
        let moved = vec![
            Amphipod::new(Desert, (0, 0)),
            Amphipod::new(Desert, (3, 0)),
            Amphipod::new(Desert, (10, 0)),
        ];
        assert_eq!(
            map_to_position(test_pod.get_possible_moves(&vec![], &moved)),
            vec![(5, 0), (7, 0), (9, 0)]
        );
    }

    #[test]
    fn test_can_get_home() {
        fn verify_result(actual: Option<(usize, (usize, usize))>, expected: (usize, usize), is_some: bool) {
            if is_some {
                assert!(actual.is_some());
                assert_eq!(actual.unwrap().1, expected);
            } else {
                assert!(actual.is_none());
            }
        }
        let test_pod_initial = Amphipod::new(Amber, (8, 2));
        
        // no other pods, can get home
        verify_result(test_pod_initial.can_get_home(&vec![], &vec![], &vec![]), (2, 2), true);

        // one pod already in home
        verify_result(test_pod_initial.can_get_home(&vec![], &vec![], &vec![Amphipod::new(Amber, (2, 2))]), (2, 1), true);
    
        // pod with different type in home
        verify_result(test_pod_initial.can_get_home(&vec![], &vec![], &vec![Amphipod::new(Bronze, (2, 2))]), (2, 1), false);

        // pod is in initial and can't move outside
        verify_result(test_pod_initial.can_get_home(&vec![Amphipod::new(Bronze, (8, 1))], &vec![], &vec![]), (2, 1), false);

        // some pods in hallway and initial but not in the way
        let test_pod_initial = Amphipod::new(Bronze, (8, 2));
        let moved = vec![
            Amphipod::new(Bronze, (9, 0)), 
            Amphipod::new(Amber, (0, 1)), 
        ];
        let initial = vec![Amphipod::new(Bronze, (6, 2)), Amphipod::new(Desert, (6, 1))];
        verify_result(test_pod_initial.can_get_home(&initial, &moved, &vec![]), (4, 2), true);

        // pods in the way
        let moved = vec![
            Amphipod::new(Bronze, (7, 0))
        ];
        verify_result(test_pod_initial.can_get_home(&vec![], &moved, &vec![]), (4, 2), false);

    }

    #[test]
    fn test_do_move() {
        let (initial, home) = create_test_data();
        // let initial = create_actual_data();
        // let home = vec![];
        println!("{}", Burrow::organize_pods(initial, home));
    }
}
