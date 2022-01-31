use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

fn main() {
    let mut graph = Graph::new();
    let content = fs::read_to_string("./src/inputs/input_aoc_9.txt").unwrap();
    for i in content.lines() {
        graph.insert(parse(i));
    }
    let routes: Vec<Vec<&String>> = graph
        .vertex
        .iter()
        .permutations(8)
        .collect();
    let routes_len: Vec<i64> = routes
        .iter()
        .map(|x| { graph.len_route(&x)})
        .collect();
    println!("part 1 : {}", routes_len.iter().min().unwrap());
    println!("part 2 : {}", routes_len.iter().max().unwrap());
}

#[derive(Debug, Clone)]
struct Graph {
    vertex: HashSet<String>,
    edges: Vec<(String, String, i64)>
}
impl Graph {
    fn new() -> Graph {
        Graph {
            vertex: HashSet::new(),
            edges: vec![]
        }
    }

    fn insert(&mut self, entry : (String,String, i64)) {
        self.vertex.insert(entry.0.to_string());
        self.vertex.insert(entry.1.to_string());
        self.edges.push(entry.clone())
    }

    fn len_route(&self, route : &Vec<&String>) -> i64 {
        let mut n = 0;
        let mut r = route.clone();
        while r.len() > 1 {
            for (input,output,length) in self.edges.iter() {
                if (input == r[0] && output == r[1]) || (input == r[1] && output == r[0]) {
                    n += length;
                }
            }
            r.remove(0);
        }
        n
    }
}

fn parse(s : &str) -> (String, String, i64) {
    let tokens: Vec<&str> = s.split(" ").collect();
    let num = tokens[4].parse::<i64>().unwrap();
    (tokens[0].clone().to_string(), tokens[2].clone().to_string(), num)
}
