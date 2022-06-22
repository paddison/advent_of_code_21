// build a graph from input data
// traversing:
// begin with start, get all neighbors, start building paths, similar to breadth first search
// path needs to be aware of which nodes have been visited
// nodes can be stored as reference on path
// store each path in array

// algo:
// name: build_paths(n, graph, vec<paths>) -> paths
// 1.   if n == end
// 2.       return paths
// 3.   if graph.neighbors.lenght == 0
// 4.       return None
// 5.   for n in neighbors
//          if n is not capital:
//              set n to 0 in graph (indicates visited)
//          let found_paths = build_path(n, graph) // needs to pass copy of graph unfortunately
//          for p in found_paths
//              paths.append(p)

// graph is represented adjacency matrix containing bools
// bool indicates edge, 
// use hashmap to create lookup table for indices in adjacency matrix
// create graph struct which contains adjacency matrix and lookup table
// two types of nodes: capital letters and small letters.
// no two capital letters should be connected to each other

// HashMap entries are organized the following way:
// entry 0 = start,
// entry 1 = end,
// all other will be added sequentially by inserting at n > 1
// use counter to add nodes

pub fn get_solution_1() -> usize {
    let lines = parse_lines("data/day_12.txt");
    let nodes = parse_input(lines);
    let g = Graph::new(nodes, 1);
    let mut paths = vec![];
    Graph::build_paths(0, g.adj_matrix, &g.l_table, &mut paths, vec!["start"]);
        // println!("{:?}", paths);
    paths.len()
}

pub fn get_solution_2() -> usize {
    let lines = parse_lines("data/day_12.txt");
    let nodes = parse_input(lines);
    let g = Graph::new(nodes, 2);
    let mut paths = vec![];
    Graph::build_paths_2(0, g.adj_matrix, &g.l_table, &mut paths, vec!["start"], HashSet::new(), 0);
    paths.len()
}

use std::collections::{HashMap, HashSet};

use crate::parse_lines;

struct Graph {
    adj_matrix: Vec<Vec<i8>>,
    l_table: HashMap<usize, String>,
}

impl Graph {
    // first find all nodes
    // look at each touple and add them to temp HashSet
    // create l_table from HashSet
    // create adjacency matrix by looping through edges and look up nodes
    // problem: l_table stores nodes as values
    // create second table in constructor to lookup nodes?
    pub fn new(node_edges: Vec<(String, String)>, num_visits: i8) -> Self {
        let mut nodes = HashSet::new();
        let mut l_table = HashMap::new();
        let mut tmp_table = HashMap::new();
        for (left, right) in &node_edges {
            if !nodes.contains(left) {
                nodes.insert(left.to_string());
            }
            if !nodes.contains(right) {
                nodes.insert(right.to_string());
            }
        }

        let mut adj_matrix = vec![vec![0; nodes.len()]; nodes.len()];

        // create lookup table
        let mut c = 2;
        for node in nodes.drain() {
            if node != "start".to_string() && node != "end".to_string() {
                tmp_table.insert(node.clone(), c);
                l_table.insert(c, node);
                c += 1;
            } else if node == "start".to_string() {
                tmp_table.insert(node.clone(), 0);
                l_table.insert(0, node);
            } else {
                tmp_table.insert(node.clone(), 1);
                l_table.insert(1, node);
            }
        }

        // create adjacency matrix
        for (left, right) in node_edges {
            let l_index = *tmp_table.get(&left).unwrap();
            let r_index = *tmp_table.get(&right).unwrap();
            adj_matrix[l_index][r_index] = num_visits;
            adj_matrix[r_index][l_index] = num_visits; // graph should be bidirectional
        }

        Graph { adj_matrix, l_table }
    }

    fn build_paths<'a>(n: usize, adj_matrix: Vec<Vec<i8>>, l_table: &'a HashMap<usize, String>, paths: &mut Vec<Vec<&'a str>>, path: Vec<&'a str>) {
        if n == 1 {
            return paths.push(path);
        }
        
        let is_upper = is_upper_string(l_table.get(&n).unwrap());

        for (i, neighbor) in adj_matrix[n].iter().enumerate() {
            if i == 0 {
                continue;
            }
            if neighbor > &0 {
                let mut new_adj_matrix = adj_matrix.clone();
                let mut new_path = path.clone();
                if let Some(node) = l_table.get(&i) {
                    if !is_upper {
                        for j in 0..new_adj_matrix.len() {
                            new_adj_matrix[n][j] -= 1;
                            new_adj_matrix[j][n] -= 1;
                        }
                    }
                    new_path.push(node);
                    Graph::build_paths(i, new_adj_matrix, &l_table, paths, new_path);
                }
            }
        }
    }

    fn build_paths_2<'a>(
            n: usize, adj_matrix: Vec<Vec<i8>>, 
            l_table: &'a HashMap<usize, String>, 
            paths: &mut Vec<Vec<&'a str>>, 
            path: Vec<&'a str>,
            mut visited: HashSet<&'a str>,
            threshold: i8,
        ) {
        if n == 1 {
            return paths.push(path);
        }

        // so messy...
        let node = l_table.get(&n).unwrap();
        let is_upper = is_upper_string(node);
        let mut new_threshold = threshold;
        if !is_upper && threshold == 0 && visited.contains(node.as_str()){
            new_threshold += 1;
        } else {
            visited.insert(node);
        }

        for (i, neighbor) in adj_matrix[n].iter().enumerate() {
            if i == 0 {
                continue;
            }
            if neighbor > &threshold {
                let mut new_adj_matrix = adj_matrix.clone();
                let mut new_path = path.clone();
                if let Some(node) = l_table.get(&i) {
                    if !is_upper {
                        for j in 0..new_adj_matrix.len() {
                            new_adj_matrix[n][j] -= 1;
                            new_adj_matrix[j][n] -= 1;
                        }
                    }
                    new_path.push(node);
                    Graph::build_paths_2(i, new_adj_matrix, &l_table, paths, new_path, visited.clone(), new_threshold);
                }
            }
        }
    }

}

fn is_upper_string(s: &str) -> bool {
    for c in s.chars() {
        if c.is_lowercase() {
            return false;
        }
    }

    true
}

fn parse_input(lines: Vec<String>) -> Vec<(String, String)> {
    let mut nodes = vec![];
    for line in lines {
        let splits = line.split("-").collect::<Vec<&str>>();
        assert_eq!(2, splits.len());
        nodes.push((splits[0].to_string(), splits[1].to_string()));
    }

    nodes
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::Graph;
    use super::is_upper_string;
    use super::parse_input;
    use crate::parse_lines;

    fn init_graph(num_visits: i8) -> Graph {
        let edges = vec![
            ("start".to_string(), "A".to_string()),
            ("start".to_string(), "b".to_string()),
            ("A".to_string(),"c".to_string()),
            ("b".to_string(),"d".to_string()),
            ("A".to_string(),"b".to_string()),
            ("b".to_string(),"end".to_string()),
            ("A".to_string(),"end".to_string()),
        ];
        Graph::new(edges, num_visits)
    }

    #[test]
    fn test_new_graph() {
        let g = init_graph(1);
        println!("{:?}", g.l_table);
        println!("{:?}", g.adj_matrix);
    }

    #[test]
    fn test_is_upper_string() {
        assert!(is_upper_string("HELLO"));
        assert!(!is_upper_string("hello"));
        assert!(!is_upper_string("heLLo"));
        assert!(!is_upper_string("aaaA"));
        assert!(is_upper_string("AAAA"));
        assert!(!is_upper_string("AAAa"));
    }

    #[test]
    fn test_paths() {
        let g = init_graph(1);
        let mut paths = vec![];
        Graph::build_paths(0, g.adj_matrix, &g.l_table, &mut paths, vec!["start"]);
        assert_eq!(10, paths.len());
    }

    #[test]
    fn test_paths_2() {
        let lines = parse_lines("data/day_12_test_2.txt");
        let nodes = parse_input(lines);
        let g = Graph::new(nodes, 1);
        let mut paths = vec![];
        Graph::build_paths(0, g.adj_matrix, &g.l_table, &mut paths, vec!["start"]);
        println!("{:?}", paths);
        assert_eq!(19, paths.len());
    }

    #[test]
    fn test_paths_3() {
        let lines = parse_lines("data/day_12_test_3.txt");
        let nodes = parse_input(lines);
        let g = Graph::new(nodes, 1);
        let mut paths = vec![];
        Graph::build_paths(0, g.adj_matrix, &g.l_table, &mut paths, vec!["start"]);
        println!("{:?}", paths);
        assert_eq!(226, paths.len());
    }

    #[test]
    fn test_paths_p_1() {
        let g = init_graph(2);
        let mut paths = vec![];
        Graph::build_paths_2(0, g.adj_matrix, &g.l_table, &mut paths, vec!["start"], HashSet::new(), 0);
        // let p2 = Graph::filter_paths(paths);

        assert_eq!(36, paths.len());
    }

    #[test]
    fn test_paths_p_2() {
        let lines = parse_lines("data/day_12_test_2.txt");
        let nodes = parse_input(lines);
        let g = Graph::new(nodes, 2);
        let mut paths = vec![];
        Graph::build_paths_2(0, g.adj_matrix, &g.l_table, &mut paths, vec!["start"], HashSet::new(), 0);
        // println!("{:?}", paths);
        // let p2 = Graph::filter_paths(paths);

        assert_eq!(103, paths.len());
    }

    #[test]
    fn test_paths_p_3() {
        let lines = parse_lines("data/day_12_test_3.txt");
        let nodes = parse_input(lines);
        let g = Graph::new(nodes, 2);
        let mut paths = vec![];
        Graph::build_paths_2(0, g.adj_matrix, &g.l_table, &mut paths, vec!["start"], HashSet::new(), 0);

        assert_eq!(3509, paths.len());
    }
}
