use std::{ops::Deref, fmt::{Display, Formatter}, collections::{VecDeque, HashMap, BinaryHeap}, cmp::Ordering};
use PodType::*;


const BOARD_WIDTH: usize = 13;
static mut HOME_SIZE: usize = 2; 

macro_rules! pod {
    ($p_type:expr, $x:expr, $y:expr) => {
        Amphipod::new($p_type, ($x, $y))
    };
}


fn create_pods(home_size: usize) -> Pods {
    vec![pod!(Copper, 2, 1), pod!(Amber, 4, 1), pod!(Bronze, 6, 1), pod!(Desert, 8, 1),
         pod!(Copper, 2, home_size), pod!(Amber, 4, home_size), pod!(Desert, 6, home_size), pod!(Bronze, 8, home_size)]
}

fn create_additional_pods() -> Pods {
    vec![pod!(Desert, 2, 2), pod!(Copper, 4, 2), pod!(Bronze, 6, 2), pod!(Amber, 8, 2), 
         pod!(Desert, 2, 3), pod!(Bronze, 4, 3), pod!(Amber, 6, 3), pod!(Copper, 8, 3)]
}

pub fn get_solution_1() -> usize {
    unsafe {
        HOME_SIZE = 2;
    }
    let initial = create_pods(2);
    Burrow::organize_pods_queue(initial, vec![])
}

pub fn get_solution_2() -> usize {
    unsafe {
        HOME_SIZE = 4;
    }
    let mut initial = create_pods(4);
    initial.append(&mut create_additional_pods());
    Burrow::organize_pods_queue(initial, vec![])
}

type Pods = Vec<Amphipod>;

// TODO if pod is home, remove it from set, then adjust homes
#[derive(Clone, Eq)]
struct Burrow {
    pods: Vec<Amphipod>,
    homes: [Vec<(usize, usize)>; 4],
    home: Vec<Amphipod>,
    cost: usize,
    heuristic: usize,
}

impl Ord for Burrow {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.cost + self.heuristic).cmp(&(other.cost + other.heuristic)).reverse()
    }
}

impl PartialOrd for Burrow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.cost + self.heuristic)
            .partial_cmp(&(other.cost + other.heuristic))
            .and_then(|ord| Some(ord.reverse()))
    }
}

impl Burrow {

    fn new(pods: Pods, home: Pods, cost: usize) -> Self {
        let mut burrow = Burrow { pods, home, cost, heuristic: 0 };
        burrow.heuristic = burrow.calculate_heuristic();
        burrow
    }

    fn organize_pods_queue(initial: Pods, home: Pods) -> usize {
        let mut queue = BinaryHeap::new();
        let start = Burrow::new(initial, home, 0);
        // let cache = HashMap::new<Vec<Amphipod();


        queue.push(start);
        
        while let Some(burrow) = queue.pop() {
            if burrow.pods.is_empty() {
                return burrow.cost;
            }
            for neighbour in burrow.get_neighbours() {
                queue.push(neighbour)
            }
        }
        0
    }

    fn calculate_heuristic(&self) -> usize {

        let mut pod_homes = HashMap::from([
            (Amber, Amphipod::calculate_home(Amber)),
            (Bronze, Amphipod::calculate_home(Bronze)),
            (Copper, Amphipod::calculate_home(Copper)),
            (Desert, Amphipod::calculate_home(Desert)),
        ]);
        
        // filter homes:
        for pod in &self.home {
            let home =  pod_homes.get_mut(&pod.p_type).unwrap();
            match home.iter().position(|pos| &pod.pos == pos) {
                Some(idx) => { home.remove(idx); },
                None => {
                    println!("invalid index!");
                }
            }
        }

        let mut cost = 0;
        for pod in &self.pods {
            let home = pod_homes.get_mut(&pod.p_type).unwrap();
            let home_pos = home.pop().unwrap();
            if home_pos.0 == pod.pos.0 {
                cost += (home_pos.1 + pod.pos.1 + 2) * pod.cost();
            } else {
                cost += pod.calculate_move_cost(&home_pos);
            }
        }

        cost
    }

    // moves in burrow are always sorted by cost
    fn get_neighbours(mut self) -> Vec<Self> {
        // first check if pods that haven't moved so far can get home (since this will lead to a minimal score)
        for (i, pod) in self.pods.iter().enumerate() {
            if let Some((cost, pos)) = pod.can_get_home(&self.pods, &self.home) {
                // remove from initial and add to home
                let mut home_pod = self.pods.remove(i);
                home_pod.pos = pos;
                self.home.push(home_pod);
                return vec![Burrow::new(self.pods, self.home, self.cost + cost)];
            }
        }

        // move pods
        let mut neighbours = Vec::new();
        for (i, pod) in self.pods.iter().enumerate() {
            // get possible moves needs to return possible moves sorted by cost
            for (cost, position) in pod.get_possible_moves(&self.pods).into_iter() {
                // assign pod new position
                let mut new_pods = self.pods.clone();
                if let Some(moved_pod) = new_pods.get_mut(i) {
                    moved_pod.pos = position;
                }

                neighbours.push(Burrow::new(new_pods, self.home.clone(), self.cost + cost));
            }
        }
        neighbours
    }

    fn contains(lhs: &Pods, rhs: &Pods) -> bool {
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
  #.#.#.#.#  
  #.#.#.#.#  
  #########");
              for p in self.pods.iter().chain(&self.home) {
                let index = 1 + p.pos.0 + (p.pos.1 + 1) * BOARD_WIDTH + (p.pos.1 + 1);
                burrow_string.replace_range(index..index + 1, p.p_type.into());
              }
        write!(f, "{}", burrow_string)
    }
}



impl PartialEq for Burrow {
    fn eq(&self, other: &Self) -> bool {
        Burrow::contains(&self.pods, &other.pods) &&
        Burrow::contains(&self.home, &other.home) &&
        self.cost == other.cost
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
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
    fn calculate_move_cost(&self, position: &(usize, usize)) -> usize {
        let x_diff = self.pos.0.abs_diff(position.0);
        let y_diff = self.pos.1 + position.1;

        (x_diff + y_diff) * self.cost()
    }

    // only gets called on pod that is in intial
    fn get_possible_moves(&self, pods: &Pods) -> Vec<(usize, (usize, usize))> {
        // pod will always move into hallway
        // if pod is at y == 2, check if there is a pod above him, if so, return empty vec (pod cannot move)
        if self.pos.1 == 0 {
            return vec![];
        }

        if self.pos.1 >= 2 {
            if pods.iter().any(|pod| pod.pos.0 == self.pos.0 && pod.pos.1 < self.pos.1) {
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
                if pods.iter().any(|pod| pod.pos == left_pos) {
                    left_positions = vec![];
                } else {
                    possible_positions.push(left_pos);
                }
            }
            if let Some(right_pos) = right_positions.pop_front() {
                if pods.iter().any(|pod| pod.pos == right_pos) {
                    right_positions = VecDeque::new();
                } else {
                    possible_positions.push(right_pos);
                }
            }
        }

        possible_positions
            .into_iter()
            .map(|pos| (self.calculate_move_cost(&pos), pos))
            .collect::<Vec<(usize, (usize, usize))>>()
    }

    fn calculate_home(p_type: PodType) -> Vec<(usize, usize)> {
        let x = match p_type {
            Amber => 2,
            Bronze => 4,
            Copper => 6,
            Desert => 8,
        };
        unsafe {
            (1..HOME_SIZE + 1).map(|y| (x, y)).collect::<Vec<(usize, usize)>>()
        }
    }

    fn get_pod_home(&self) -> Vec<(usize, usize)> {
        Self::calculate_home(self.p_type)
    }

    // only gets called on pods that are in moved or initial
    fn can_get_home(&self, pods: &Pods, home: &Pods) -> Option<(usize, (usize, usize))> {
        // three steps:
        // 1. check if pod can move (if it is in initial)
        // 2. check if home is free
        // 3. check if hallway is free

        // check if pod can move:
        if self.pos.0 > 1 && pods.iter().any(|other| other.pos.0 == self.pos.0 && other.pos.1 < self.pos.1) {
            return None;
        }

        // check if pod_home is free
        let pod_home = self.get_pod_home();

        // only pods in initial or home may be in pod_home
        if pods.iter().any(|other| pod_home.contains(&other.pos)) {
            return None
        }

        let y_pos = unsafe {
            HOME_SIZE - home.into_iter()
                            .filter(|other| self.p_type == other.p_type)
                            .count()
        };

        let home_pos = match self.p_type {
            Amber => (2, y_pos),
            Bronze => (4, y_pos),
            Copper => (6, y_pos),
            Desert => (8, y_pos),
        };

        // other_pods_in_home.sort_by(|a, b| a.pos.1.cmp(&b.pos.1).reverse());
        // check if home is free

        // for other in &other_pods_in_home {
        //     if self.p_type != other.p_type {
        //         return None
        //     } else {
        //         home_pos = (home_pos.0, home_pos.1 - 1)
                
        //     }
        // }  

        // check if there are any pods on path to pod_home in hallway
        let lower_bound = self.pos.0.min(home_pos.0);
        let upper_bound = self.pos.0.max(home_pos.0);

        if pods.iter().any(|pod| pod.pos.1 == 0 && pod.pos.0 < upper_bound && pod.pos.0 > lower_bound) {
            return None
        }

        Some((self.calculate_move_cost(&home_pos), home_pos))
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

#[derive(Copy, Clone, PartialEq, Hash, Eq, PartialOrd)]
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
    use std::collections::BinaryHeap;

    use crate::day_23::HOME_SIZE;

    use super::Amphipod;
    use super::Burrow;
    use super::PodType::*;
    use super::Pods;

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

    fn create_test_data_2() -> (Pods, Pods) {
        // #B#C#B#D#
        // #D#C#B#A#
        // #D#B#A#C#
        // #A#D#C#A#
        (
            vec![
                Amphipod::new(Bronze, (2, 1)), Amphipod::new(Copper, (4, 1)), Amphipod::new(Bronze, (6, 1)), Amphipod::new(Desert, (8, 1)),
                Amphipod::new(Desert, (2, 2)), Amphipod::new(Copper, (4, 2)), Amphipod::new(Bronze, (6, 2)), Amphipod::new(Amber, (8, 2)),
                Amphipod::new(Desert, (2, 3)), Amphipod::new(Bronze, (4, 3)), Amphipod::new(Amber, (6, 3)), Amphipod::new(Copper, (8, 3)),
                Amphipod::new(Desert, (4, 4)), Amphipod::new(Amber, (8, 4))
            ],
            vec![Amphipod::new(Amber, (2, 4)), Amphipod::new(Copper, (6, 4))]
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
        let pods = vec![
            Amphipod::new(Amber, (0, 0)),
            Amphipod::new(Amber, (2, 2)),
            Amphipod::new(Bronze, (2, 1)),
            Amphipod::new(Bronze, (4, 2)),
            Amphipod::new(Copper, (3, 0)),
            Amphipod::new(Copper, (5, 0)),
            Amphipod::new(Desert, (8, 1)),
            Amphipod::new(Desert, (10, 0)),
            ];
        
        Burrow::new(pods, vec![], 0);
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

        assert_eq!(bronze.calculate_move_cost(&right), 90);
        assert_eq!(copper.calculate_move_cost(&up_right), 300);
        assert_eq!(desert.calculate_move_cost(&right_down), 4000);
        assert_eq!(amber.calculate_move_cost(&up_left_down), 10);
    }

    #[test]
    fn test_get_possible_moves() {
        let test_pod = Amphipod::new(Amber, (6, 2));
        
        // no other pods, should return all possible moves
        assert_eq!(
            map_to_position(test_pod.get_possible_moves(&vec![])), 
            vec![(0, 0), (1, 0), (3, 0), (5, 0), (7, 0), (9, 0), (10, 0)]
        );

        // above is no space, should not return any moves
        let initial = vec![Amphipod::new(Desert, (6, 1))];
        assert_eq!(test_pod.get_possible_moves(&initial), vec![]);

        // only left positions are free
        let moved = vec![Amphipod::new(Desert, (7, 0))];
        assert_eq!(
            map_to_position(test_pod.get_possible_moves(&moved)),
            vec![(0, 0), (1, 0), (3, 0), (5, 0)]
        );

        // only right positions are free
        let moved = vec![Amphipod::new(Desert, (5, 0))];
        assert_eq!(
            map_to_position(test_pod.get_possible_moves(&moved)),
            vec![(7, 0), (9, 0), (10, 0)]
        );

        // some left and some right are free
        let moved = vec![
            Amphipod::new(Desert, (0, 0)),
            Amphipod::new(Desert, (3, 0)),
            Amphipod::new(Desert, (10, 0)),
        ];
        assert_eq!(
            map_to_position(test_pod.get_possible_moves(&moved)),
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
        let test_pod_initial = pod!(Amber, 8, 2);
        
        // no other pods, can get home
        verify_result(test_pod_initial.can_get_home(&vec![], &vec![]), (2, 2), true);

        // one pod already in home
        verify_result(test_pod_initial.can_get_home(&vec![], &vec![pod!(Amber, 2, 2)]), (2, 1), true);
    
        // pod with different type in home
        verify_result(test_pod_initial.can_get_home(&vec![], &vec![pod!(Bronze, 2, 2)]), (2, 1), false);

        // pod is in initial and can't move outside
        verify_result(test_pod_initial.can_get_home(&vec![pod!(Bronze, 8, 1)], &vec![]), (2, 1), false);

        // some pods in hallway and initial but not in the way
        let test_pod_initial = pod!(Bronze, 8, 2);
        let pods = vec![pod!(Bronze, 9, 0), pod!(Amber, 0, 1), pod!(Bronze, 6, 2), pod!(Desert, 6, 1)];
        verify_result(test_pod_initial.can_get_home(&pods, &vec![]), (4, 2), true);

        // pods in the way
        let moved = vec![
            pod!(Bronze, 7, 0)
        ];
        verify_result(test_pod_initial.can_get_home(&moved, &vec![]), (4, 2), false);

    }

    #[test]
    fn test_organize_pods() {
        unsafe {
            HOME_SIZE = 4;
        }
        let (initial, home) = create_test_data_2();
        // let initial = create_actual_data();
        // let home = vec![];
        // println!("{}", Burrow::new(initial, vec![], home, 0));
        println!("{}", Burrow::organize_pods_queue(initial, home));
        // unsafe {
        //     HOME_SIZE = 2;
        // }
    }

    #[test]
    fn test_burrow_in_bin_heap() {
        let initial1 = create_actual_data();
        let (initial2, home2) = create_test_data();
        let burrow1 = Burrow::new(initial1.clone(), vec![], 200);
        let burrow2 = Burrow::new(initial2, home2, 0);
        let burrow3 = Burrow::new(initial1.clone(), vec![], 100);

        let mut queue = BinaryHeap::new();
        queue.push(burrow1);
        queue.push(burrow2);
        queue.push(burrow3);

        while let Some(burrow) = queue.pop() {
            println!("{}\ncost: {}\nheuristic: {}", burrow, burrow.cost, burrow.heuristic);
        }
    }
}
