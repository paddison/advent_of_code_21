use std::{ops::Deref, collections::{HashMap, BinaryHeap}};

pub fn get_solution_1() -> usize {
    let mut c = parse(include_str!("../data/day_15.txt"));
    c.find_cheapest_path()
}

pub fn get_solution_2() -> usize {
    let c = parse(include_str!("../data/day_15.txt"));
    let mut larger = enlarge_cave(c, 5);
    larger.find_cheapest_path()
}

fn parse(input: &str) -> Cave {
    let cols = input.find('\n').unwrap();
    let vals = input
                .split('\n')
                .flat_map(|l| l.chars())
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>();
    
    let rows = vals.len() / cols;
    Cave { vals, dim: (rows, cols), visited: HashMap::new() }
}

struct Cave {
    vals: Vec<usize>,
    dim: (usize, usize), // row, col
    visited: HashMap<(usize, usize), usize>
}

impl Deref for Cave {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.vals
    }
}

impl Cave {
    fn get(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.dim.0 || col >= self.dim.1 {
            return None;
        } 
        let index = self.dim.1 * row + col;
        self.vals.get(index).cloned()     
    }

    fn get_neighbours(&self, n: &Node) -> Vec<Node> {
        let mut neighbours = Vec::new();
        let (row, col) = n.pos;
        // right
        if let Some(val) = self.get(row, col + 1) {
            neighbours.push(Node::new(val, (row, col + 1), n.cost + n.val, self.distance(row, col + 1)));
        }
        //down
        if let Some(val) = self.get(row + 1, col) {
            neighbours.push(Node::new(val, (row + 1, col), n.cost + n.val, self.distance(row + 1, col)));
        }
        // left
        if col > 0 {
            if let Some(val) = self.get(row, col - 1) {
                neighbours.push(Node::new(val, (row, col - 1), n.cost + n.val, self.distance(row, col - 1)));
            }
        }
        // up
        if row > 0 {
            if let Some(val) = self.get(row - 1, col) {
                neighbours.push(Node::new(val, (row - 1, col), n.cost + n.val, self.distance(row - 1, col)));
            }
        }

        neighbours
    }

    // calculate manhattan distance
    fn distance(&self, row: usize, col: usize) -> usize {
        row.abs_diff(self.dim.0) + col.abs_diff(self.dim.1)
    }

    fn is_goal(&self, n: &Node) -> bool {
        n.pos == (self.dim.0 - 1, self.dim.1 - 1)
    }

    fn find_cheapest_path(&mut self) -> usize {
        let initial = Node::new(0, (0, 0), 0, self.distance(0, 0));
        let mut queue = BinaryHeap::new();
        queue.push(initial);

        while let Some(n) = queue.pop() {
            for neighbour in self.get_neighbours(&n) {
                if self.is_goal(&neighbour) {
                    return neighbour.cost + neighbour.val
                }
                if let Some(cost) = self.visited.get_mut(&neighbour.pos) {
                    if &neighbour.cost < cost {
                        *cost = neighbour.cost;
                        queue.push(neighbour)
                    }
                } else {
                    self.visited.insert(neighbour.pos, neighbour.cost);
                    queue.push(neighbour)
                }
            }
        }

        unreachable!()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Node {
    val: usize, // value of node
    pos: (usize, usize),
    cost: usize, // cost of getting to node
    dist: usize // manhattan distance to goal
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.val + self.cost + self.dist).cmp(&(other.val + other.cost + other.dist)).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.val + self.cost + self.dist)
            .partial_cmp(&(other.val + other.cost + other.dist))
            .and_then(|ord| Some(ord.reverse()))
    }
}

impl Node {
    fn new(val: usize, pos: (usize, usize), cost: usize, dist: usize) -> Self {
        Node { val, pos, cost, dist }
    }
}

#[test]
fn test_parse() {
    let c = parse(include_str!("../data/day_15_test.txt"));
    assert_eq!(c.dim.0, 10);
    assert_eq!(c.dim.1, 10);
    assert_eq!(c.vals.len(), 100);
}

#[test]
fn test_get() {
    let c = parse(include_str!("../data/day_15_test.txt"));
    let v99 = c.get(9, 9);
    assert_eq!(Some(1), v99);
    let v_010 = c.get(0, 10);
    assert_eq!(None, v_010);
}

#[test]
fn with_test_data() {
    let mut c = parse(include_str!("../data/day_15_test.txt"));

    let result = c.find_cheapest_path();
    assert_eq!(result, 40);
}

#[test]
fn test_ord_node() {
    use std::cmp::Ordering;
    let c = parse(include_str!("../data/day_15_test.txt"));

    let expensive = Node::new(3, (1, 1), 1, c.distance(1, 1));
    let cheap = Node::new(2, (2, 0), 1, c.distance(2, 0));

    assert_eq!(cheap.cmp(&expensive), Ordering::Greater);
    assert_eq!(expensive.cmp(&cheap), Ordering::Less);

    let mut heap = BinaryHeap::new();
    heap.push(expensive);
    heap.push(cheap);
    assert_eq!(heap.pop(), Some(cheap));

    let mut heap = BinaryHeap::new();
    heap.push(cheap);
    heap.push(expensive);
    assert_eq!(heap.pop(), Some(cheap));
}


fn enlarge_cave(c: Cave, scale_factor: usize) -> Cave {
    // transfrom map.vals to normal numbers:
    let base_block: Vec<usize> = c.vals.iter().map(|val| *val).collect();

    let mut new_vals = vec![];

    for i in 0..scale_factor {
        for y in 0..c.dim.1 {
            for j in 0..scale_factor {
                for x in 0..c.dim.0 {
                    let new_val = (base_block[y * c.dim.0 + x] + i as usize + j as usize - 1) % 9 + 1;
                    new_vals.push(new_val);
                }
            }
        }
    }

    // let vals = new_vals.into_iter().map(Some).collect();

    Cave { vals: new_vals, dim: (c.dim.0 * scale_factor, c.dim.1 * scale_factor), visited: HashMap::new() }
}

