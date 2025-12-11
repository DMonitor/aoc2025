use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("../res/input");

#[derive(Debug, Clone)]
struct Node {
    children: Vec<usize>,
    times_traveled: usize
}

impl Node {
    fn new() -> Self
    {
        Node { children: Vec::new(), times_traveled: 0 }
    }
    fn add_child(&mut self, child: usize) {
        self.children.push(child);
    }

    fn traverse_next(&mut self) -> Option<usize> {
        self.children.get(self.times_traveled).cloned()
    }
}


fn parse_input_1(input: &str) -> (Vec<Node>, usize) {

    let mut point_bucket:Vec<Node> = vec![Node::new(); input.lines().count()];
    let mut map:HashMap<&str,usize> = HashMap::new();

    // first pass to build the mapping
    input.lines().enumerate().for_each(|(idx, line)| {
        map.insert(line.split_at(3).0, idx);
    });

    for line in input.lines() {
        let (name, connections) = line.split_at(3);
        let id = map.get(name).unwrap();

        for child in connections[2..].split_whitespace() {
            if child == "out" {continue;}
            point_bucket[*id].add_child(*map.get(child).unwrap());
        }
    }
    (point_bucket,*map.get("you").unwrap())
}

fn solve_2(input: &str) -> u32 {
    0
}

fn explore_node(bucket:&mut Vec<Node>, node: usize) -> u64
{
    let mut ends = 0;
    let node = bucket.get_mut(node);
    if node.is_none() {return 0;}
    let node = node.unwrap();

    let mut queue: VecDeque<usize> = node.children.iter().copied().collect();

    while let Some(next_node_idx) = queue.pop_front()
    {
        let next_node = bucket.get_mut(next_node_idx).unwrap();

        if next_node.children.is_empty() {
            ends += 1;
            continue;
        }
        queue.extend(next_node.children.iter().copied());
    }

    ends

}

fn solve_1(input: &str) -> u64 {
    /*
    The plan:
        - build a node map
        - starting at [you] traverse until [out] is reached
        - track how many times a node has been traversed
        - if times traversed == number of child nodes, remove self from network
        - repeat until [you] has been removed
     */
    let (mut bucket, st) = parse_input_1(input);


    explore_node(bucket.as_mut(), st)

}

fn main()
{


    println!("{}", solve_1(INPUT));
    println!("{}", solve_2(INPUT));
}