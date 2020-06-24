use std::collections;
use std::env;
use std::fs;

fn build_adjacency(
    edges: &Vec<Vec<String>>,
) -> collections::HashMap<String, Vec<String>> {
    let mut adjacency: collections::HashMap<String, Vec<String>> =
        collections::HashMap::new();

    for pair in edges {
        let first_result = adjacency.get_mut(&pair[0]);
        match first_result {
            Some(results) => {
                results.push(pair[1].clone());
            }
            None => {
                adjacency.insert(pair[0].clone(), vec![pair[1].clone()]);
            }
        }

        let second_result = adjacency.get_mut(&pair[1]);
        match second_result {
            Some(results) => {
                results.push(pair[0].clone());
            }
            None => {
                adjacency.insert(pair[1].clone(), vec![pair[0].clone()]);
            }
        }
    }

    return adjacency;
}

fn part_one(adjacency: &collections::HashMap<String, Vec<String>>) -> usize {
    let mut stack = Vec::<(String, usize)>::new();
    let mut visited: collections::HashSet<String> = collections::HashSet::new();
    stack.push((String::from("COM"), 0));
    let mut orbit_count: usize = 0;

    while stack.len() > 0 {
        let (label, count) = stack.pop().unwrap();

        if visited.contains(&label) {
            continue;
        }

        visited.insert(label.clone());

        orbit_count += count;

        let label_adjacency = adjacency.get(&label);

        match label_adjacency {
            Some(next_labels) => {
                for next_label in next_labels {
                    if visited.contains(next_label) {
                        continue;
                    }

                    stack.push((next_label.clone(), count + 1));
                }
            }
            None => {
                continue;
            }
        }
    }

    return orbit_count;
}

fn part_two(adjacency: &collections::HashMap<String, Vec<String>>) -> usize {
    let mut queue = collections::VecDeque::<(String, usize)>::new();
    let mut visited: collections::HashSet<String> = collections::HashSet::new();
    queue.push_back((String::from("YOU"), 0));

    while queue.len() > 0 {
        let (label, count) = queue.pop_front().unwrap();

        if visited.contains(&label) {
            continue;
        }

        visited.insert(label.clone());

        if label == "SAN" {
            return count;
        }

        let label_adjacency = adjacency.get(&label);

        match label_adjacency {
            Some(next_labels) => {
                for next_label in next_labels {
                    if visited.contains(next_label) {
                        continue;
                    }
                    queue.push_back((next_label.clone(), count + 1));
                }
            }
            None => {
                continue;
            }
        }
    }

    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <input file>", args[0]);
    }

    let contents = fs::read_to_string(&args[1]);
    let unwrapped_contents = match contents {
        Ok(s) => s,
        Err(e) => panic!("Unable to read file: {:?}", e),
    };

    let edges: Vec<Vec<String>> = unwrapped_contents
        .lines()
        .map(|l| l.split(")").map(|s| s.to_string()).collect::<Vec<String>>())
        .collect();

    let adjacency = build_adjacency(&edges);
    let orbit_count = part_one(&adjacency);
    println!("Total orbits: {}", orbit_count);

    let distance = part_two(&adjacency);
    println!("Distance: {}", distance - 2);
}
