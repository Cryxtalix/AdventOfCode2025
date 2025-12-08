use libaocparser_rs::*;
use std::{
    time::Instant,
    collections::{
        HashMap,
    },
    rc::Rc,
    cell::RefCell,
};

fn print_manifold(manifold: Vec<Vec<u8>>) {
    let printable: Vec<String> = manifold.iter()
        .map(|s| s.iter().map(|c| *c as char).collect::<Vec<char>>())
        .map(|line| line.into_iter().collect::<String>())
        .collect();
    for line in printable {
        println!("{line}");
    }
}

fn puzzle1(aoc: AocParser) {
    let aoc = aoc.get();
    let mut manifold: Vec<Vec<u8>> = aoc
        .iter()
        .map(|s| s.as_bytes().to_vec())
        .collect();

    let mut splits = 0;
    for row in 0..manifold.len() {
        if row == 0 {
            continue;
        }
        let (top, bottom) = manifold.split_at_mut(row);

        let top_line = &top[top.len() - 1];
        let current_line = bottom.get_mut(0).unwrap();

        for col in 0..current_line.len() {
            let cur_char = current_line.get(col).unwrap();
            let top_char = top_line.get(col).unwrap();
            
            match cur_char {
                b'.' => {
                    match top_char {
                        b'S' | b'|' => {
                            current_line[col] = b'|';
                        },
                        _ => {},
                    }
                },
                b'^' => {
                    if *top_char == b'|' {
                        splits += 1;
                        current_line[col - 1] = b'|';
                        current_line[col + 1] = b'|';
                    }
                },
                _ => {},
            }
        }
    }
    
    print!("Puzzle 1: {}", splits);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Node(usize, usize);

fn get_children(node: &Node, manifold: &Vec<Vec<u8>>) -> (Option<Node>, Option<Node>) {
    let left_idx = node.0 - 1;
    let right_idx = node.0 + 1;
    // Search left
    let mut search_y = node.1;
    let left_child = {
        loop {
            search_y += 1;
            if search_y == manifold.len() - 1 {
                break None;
            }
            if manifold[search_y][left_idx] == b'^' {
                break Some(Node(left_idx, search_y));
            }
        }
    };
    // Search right
    let mut search_y = node.1;
    let right_child = {
        loop {
            search_y += 1;
            if search_y == manifold.len() - 1 {
                break None;
            }
            if manifold[search_y][right_idx] == b'^' {
                break Some(Node(right_idx, search_y));
            }
        }
    };
    (left_child, right_child)
}

fn puzzle2(aoc: AocParser) {
    let aoc = aoc.get();
    let mut manifold: Vec<Vec<u8>> = aoc
        .iter()
        .map(|s| s.as_bytes().to_vec())
        .collect();

    for row in 0..manifold.len() {
        if row == 0 {
            continue;
        }
        let (top, bottom) = manifold.split_at_mut(row);

        let top_line = &top[top.len() - 1];
        let current_line = bottom.get_mut(0).unwrap();

        for col in 0..current_line.len() {
            let cur_char = current_line.get(col).unwrap();
            let top_char = top_line.get(col).unwrap();
            
            match cur_char {
                b'.' => {
                    match top_char {
                        b'S' | b'|' => {
                            current_line[col] = b'|';
                        },
                        _ => {},
                    }
                },
                b'^' => {
                    if *top_char == b'|' {
                        current_line[col - 1] = b'|';
                        current_line[col + 1] = b'|';
                    }
                },
                _ => {},
            }
        }
    }

    // Create graph
    let mut child_map: HashMap<Node, (Option<Node>, Option<Node>)> = HashMap::new();
    let mut first_node: Option<Node> = None;

    for (y, row) in manifold.iter().enumerate() {
        for (x, cur_char) in row.iter().enumerate() {
            if *cur_char == b'^' && manifold[y-1][x] == b'|' {
                // Found active node
                let cur_node = Node(x, y);
                // Save first node
                if first_node.is_none() {
                    first_node = Some(cur_node);
                }
                // Find children
                let children = get_children(&cur_node, &manifold);
                child_map.insert(cur_node, children);
            }
        }
    }

    //Traverse graph
    let child_map = Rc::new(child_map);
    // Stores precomputed paths at each node, so I can reference instead of recalculating
    let paths_map: Rc<RefCell<HashMap<Node, u64>>> = Rc::new(RefCell::new(HashMap::new()));

    // Recursion, but without repeats by using paths_map
    fn node_count(node: &Node, child_map: Rc<HashMap<Node, (Option<Node>, Option<Node>)>>, paths_map: Rc<RefCell<HashMap<Node, u64>>>) -> u64 {
        // Paths map already contains path total for node
        if let Some(paths) = paths_map.borrow().get(node) {
            *paths
        } else {
            let children = child_map.get(node).unwrap();
            let count = match *children {
                (None, None) => 2,
                (Some(left), Some(right)) => {
                    node_count(&left, child_map.clone(), paths_map.clone()) +
                    node_count(&right, child_map.clone(), paths_map.clone())
                },
                (Some(left), None) => {
                    node_count(&left, child_map.clone(), paths_map.clone()) + 1
                },
                (None, Some(right)) => {
                    1 + node_count(&right, child_map.clone(), paths_map.clone())
                },
            };
            paths_map.borrow_mut().insert(*node, count);
            count
        }
    }
    
    let first_node = first_node.unwrap();
    let total = node_count(&first_node, child_map, paths_map);
    print!("Puzzle 2: {}", total);
}

pub fn run() {
    println!("============= Day 7 =============");

    let aoc = AocParser::new("inputs/day7/input.txt", Separator::Newline).unwrap();

    let now = Instant::now();
    puzzle1(aoc);
    let elapsed = now.elapsed();
    print!(" [{:.2?}]", elapsed);
    println!();

    let aoc = AocParser::new("inputs/day7/input.txt", Separator::Newline).unwrap();

    let now = Instant::now();
    puzzle2(aoc);
    let elapsed = now.elapsed();
    print!(" [{:.2?}]", elapsed);
    println!();
}
