use std::{ops::Deref, fmt::{Display, Formatter}, collections::{VecDeque, HashMap, BinaryHeap}, cmp::Ordering, hash::Hash};
use Type::*;

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
    Burrow::organize_pods(initial, vec![])
}

pub fn get_solution_2() -> usize {
    unsafe {
        HOME_SIZE = 4;
    }
    let mut initial = create_pods(4);
    initial.append(&mut create_additional_pods());
    Burrow::organize_pods(initial, vec![])
}

type Pods = Vec<Amphipod>;

#[derive(Clone, Eq)]
struct Burrow {
    pods: Vec<Amphipod>,
    home: Vec<Amphipod>,
    cost: usize,
    estimated_cost: usize,
}

impl Ord for Burrow {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.cost + self.estimated_cost).cmp(&(other.cost + other.estimated_cost)).reverse()
    }
}

impl PartialOrd for Burrow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.cost + self.estimated_cost)
            .partial_cmp(&(other.cost + other.estimated_cost))
            .and_then(|ord| Some(ord.reverse()))
    }
}

impl Burrow {

    fn new(pods: Pods, home: Pods, cost: usize) -> Self {
        let heuristic = Self::calculate_heuristic(&pods, &home);
        Burrow { pods, home, cost, estimated_cost: heuristic }
    }

    fn organize_pods(initial: Pods, home: Pods) -> usize {
        let mut queue = BinaryHeap::new();
        let start = Burrow::new(initial, home, 0);
        let mut cost_cache: HashMap<String, usize> = HashMap::new();
        cost_cache.insert(start.to_string(), 0);
        queue.push(start);
        
        while let Some(burrow) = queue.pop() {
            if burrow.pods.is_empty() {
                return burrow.cost;
            }
            for neighbour in burrow.get_neighbours(&mut cost_cache) {
                queue.push(neighbour)
            }
        }
        0
    }

    fn calculate_heuristic(pods: &Pods, home: &Pods) -> usize {

        let mut pod_homes = HashMap::from([
            (Amber, Amphipod::calculate_home(Amber)),
            (Bronze, Amphipod::calculate_home(Bronze)),
            (Copper, Amphipod::calculate_home(Copper)),
            (Desert, Amphipod::calculate_home(Desert)),
        ]);
        
        // filter homes:
        for pod in home.iter() {
            let home =  pod_homes.get_mut(&pod.typ).unwrap();
            if let Some(index) = home.iter().position(|pos| &pod.pos == pos) {
                home.remove(index);
            }
        }

        let mut cost = 0;
        for pod in pods {
            let home = pod_homes.get_mut(&pod.typ).unwrap();
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
    fn get_neighbours(mut self, cost_cache: &mut HashMap<String, usize>) -> Vec<Self> {
        // first check if pods that haven't moved so far can get home (since this will lead to a minimal score)
        for (i, pod) in self.pods.iter().enumerate() {
            if let Some(position) = pod.can_get_home(&self.pods, &self.home) {
                // remove from initial and add to home
                let updated_cost = self.cost + pod.calculate_move_cost(&position);
                let mut home_pod = self.pods.remove(i);
                home_pod.pos = position;
                self.home.push(home_pod);

                return vec![Burrow::new(self.pods, self.home, updated_cost)];
            }
        }

        // move pods
        let mut neighbours = Vec::new();
        for (i, pod) in self.pods.iter().enumerate() {
            // get possible moves needs to return possible moves sorted by cost
            for position in pod.get_possible_moves(&self.pods).into_iter() {
                // assign pod new position
                let mut new_pods = self.pods.clone();
                let updated_cost = self.cost + pod.calculate_move_cost(&position);
                if let Some(moved_pod) = new_pods.get_mut(i) {
                    moved_pod.pos = position;
                }
                let burrow = Burrow::new(new_pods, self.home.clone(), updated_cost);
                if let Some(cached_cost) = cost_cache.get_mut(&burrow.to_string()) {
                    if *cached_cost > updated_cost {
                        *cached_cost = updated_cost;
                        neighbours.push(burrow);
                    }
                } else {
                    cost_cache.insert(burrow.to_string(), updated_cost);
                    neighbours.push(burrow);
                }
            }
        }
        neighbours
    }

    fn to_string(&self) -> String {
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
        burrow_string.replace_range(index..index + 1, p.typ.into());
        }
        burrow_string
    }
}

impl Display for Burrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Burrow {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string() &&
        self.cost == other.cost
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
struct Amphipod {
    typ: Type,
    pos: (usize, usize),
}

impl Amphipod {
    fn new(p_type: Type, position: (usize, usize)) -> Self {
        Self {typ: p_type, pos: position }
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
    fn get_possible_moves(&self, pods: &Pods) -> Vec<(usize, usize)> {
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
    }

    fn calculate_home(typ: Type) -> Vec<(usize, usize)> {
        let x = match typ {
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
        Self::calculate_home(self.typ)
    }

    fn can_get_home(&self, pods: &Pods, home: &Pods) -> Option<(usize, usize)> {
        // 1. check if pod can move:
        if self.pos.0 > 1 && pods.iter().any(|other| other.pos.0 == self.pos.0 && other.pos.1 < self.pos.1) {
            return None;
        }

        // 2. check if pod_home is free
        let pod_home = self.get_pod_home();

        // check if there are other than own podtypes in home
        if pods.iter().any(|other| pod_home.contains(&other.pos)) {
            return None
        }

        let y_pos = unsafe {
            HOME_SIZE - home.into_iter()
                            .filter(|other| self.typ == other.typ)
                            .count()
        };

        let home_pos = match self.typ {
            Amber => (2, y_pos),
            Bronze => (4, y_pos),
            Copper => (6, y_pos),
            Desert => (8, y_pos),
        };

        // check if there are any pods on path to pod_home in hallway
        let lower_bound = self.pos.0.min(home_pos.0);
        let upper_bound = self.pos.0.max(home_pos.0);

        if pods.iter().any(|pod| pod.pos.1 == 0 && pod.pos.0 < upper_bound && pod.pos.0 > lower_bound) {
            return None
        }

        Some(home_pos)
    }           
}

impl Deref for Amphipod {
    type Target = Type;

    fn deref(&self) -> &Self::Target {
        &self.typ
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       let pod = match self.typ {
        Type::Amber => "A",
        Type::Bronze => "B",
        Type::Copper => "C",
        Type::Desert => "D",
    };

        write!(f, "{}", pod)
    }
}

#[derive(Copy, Clone, PartialEq, Hash, Eq, PartialOrd)]
enum Type {
    Amber = 1,
    Bronze = 10,
    Copper = 100,
    Desert = 1000,
}

impl From<char> for Type {
    fn from(c: char) -> Self {
        match c {
            'A' => Type::Amber,
            'B' => Type::Bronze,
            'C' => Type::Copper,
            'D' => Type::Desert,
            _ => panic!("Invalid Char for Podtype")
        }
    }
}

impl From<Type> for &str {
    fn from(pod: Type) -> Self {
        match pod {
            Type::Amber => "A",
            Type::Bronze => "B",
            Type::Copper => "C",
            Type::Desert => "D",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use crate::day_23::HOME_SIZE;

    use super::Amphipod;
    use super::Burrow;
    use super::Pods;
    use super::Type::*;

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
            test_pod.get_possible_moves(&vec![]), 
            vec![(0, 0), (1, 0), (3, 0), (5, 0), (7, 0), (9, 0), (10, 0)]
        );

        // above is no space, should not return any moves
        let initial = vec![Amphipod::new(Desert, (6, 1))];
        assert_eq!(test_pod.get_possible_moves(&initial), vec![]);

        // only left positions are free
        let moved = vec![Amphipod::new(Desert, (7, 0))];
        assert_eq!(
            test_pod.get_possible_moves(&moved),
            vec![(0, 0), (1, 0), (3, 0), (5, 0)]
        );

        // only right positions are free
        let moved = vec![Amphipod::new(Desert, (5, 0))];
        assert_eq!(
            test_pod.get_possible_moves(&moved),
            vec![(7, 0), (9, 0), (10, 0)]
        );

        // some left and some right are free
        let moved = vec![
            Amphipod::new(Desert, (0, 0)),
            Amphipod::new(Desert, (3, 0)),
            Amphipod::new(Desert, (10, 0)),
        ];
        assert_eq!(
            test_pod.get_possible_moves(&moved),
            vec![(5, 0), (7, 0), (9, 0)]
        );
    }

    #[test]
    fn test_can_get_home() {
        fn verify_result(actual: Option<(usize, usize)>, expected: (usize, usize), is_some: bool) {
            if is_some {
                assert!(actual.is_some());
                assert_eq!(actual.unwrap(), expected);
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
        println!("{}", Burrow::organize_pods(initial, home));
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
            println!("{}\ncost: {}\nheuristic: {}", burrow, burrow.cost, burrow.estimated_cost);
        }
    }
}