use rand::{thread_rng, Rng};
use std::collections::{HashMap, HashSet};
use std::fs;

type CaveGraph = HashMap<String, Vec<String>>;
type Edge = Vec<String>;
type Path = String;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .expect("Couldn't read the input file")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let cave_graph = build_cave_graph(&input);
    let paths = walkabout_cave_graph(&cave_graph, 10_000_000);
    dbg!(paths.len());
}

fn parse_edges(input: &[String]) -> Vec<Edge> {
    let mut output: Vec<Edge> = vec![];
    for line in input {
        let pair: Vec<String> = line.split('-').map(|value| value.to_string()).collect();
        output.push(pair);
    }
    output
}

fn build_cave_graph(input: &[String]) -> CaveGraph {
    let mut graph: CaveGraph = HashMap::new();

    // Define each node in the graph
    for edge in parse_edges(&input) {
        let left = edge.get(0).unwrap().to_string();
        let right = edge.get(1).unwrap().to_string();

        graph.insert(left, vec![]);
        graph.insert(right, vec![]);
    }

    // Populate each node's outgoing edges
    for edge in parse_edges(&input) {
        let left = edge.get(0).unwrap().to_string();
        let right = edge.get(1).unwrap().to_string();

        // Assume edges are bidirectional
        if let Some(left_edges) = graph.get_mut(&left) {
            left_edges.push(right.to_owned());
        }
        if let Some(right_edges) = graph.get_mut(&right) {
            right_edges.push(left);
        }
    }

    graph
}

fn walk_cave_graph(cave_graph: &CaveGraph) -> Path {
    // Leave a trail of breadcrumbs
    let mut output: Vec<String> = vec!["start".to_string()];
    let mut rng = thread_rng();

    loop {
        // get the next possible steps
        if let Some(here) = output.last() {
            // "end" can't be left
            if here == "end" {
                break;
            }

            if let Some(destinations) = cave_graph.get(here) {
                // Where to next?
                let valid_destinations: Vec<&String> = destinations
                    .iter()
                    .filter(|&destination| {
                        if *destination == *destination.to_lowercase() && destination != "end" {
                            !output.contains(destination)
                        } else if destination == "start" {
                            // Can't go back to start
                            false
                        } else {
                            true
                        }
                    })
                    .collect();

                // If there are no valid destinations
                if valid_destinations.is_empty() {
                    break;
                }

                // Pick the next destination randomly what's available
                let pick = rng.gen_range(0..valid_destinations.len());
                let next = valid_destinations[pick];
                output.push(next.to_string());
            }
        }
    }

    // Join the breadcrumbs into a string
    output.join(",")
}

fn walkabout_cave_graph(cave_graph: &CaveGraph, attempts: u32) -> Vec<Path> {
    let mut output: HashSet<Path> = HashSet::new();

    for attempt in 0..attempts {
        output.insert(walk_cave_graph(cave_graph));
    }

    // Remove paths that don't end at "end"
    output
        .into_iter()
        .filter(|path| path.ends_with("end"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_0: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

    const INPUT_1: &str = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;

    const INPUT_2: &str = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;

    #[test]
    fn it_creates_a_graph_with_nodes_and_edges() {
        let input: Vec<String> = INPUT_0.lines().map(|line| line.to_string()).collect();
        let _output = build_cave_graph(&input);
        assert!(true);

        let input: Vec<String> = INPUT_1.lines().map(|line| line.to_string()).collect();
        let _output = build_cave_graph(&input);
        dbg!(_output);
        assert!(true);
    }

    #[test]
    fn it_creates_a_path_to_walk_the_graph() {
        let input: Vec<String> = INPUT_0.lines().map(|line| line.to_string()).collect();
        let cave_graph = build_cave_graph(&input);
        walk_cave_graph(&cave_graph);
        assert!(true);
    }

    #[test]
    fn it_finds_valid_paths_in_input_0() {
        let input: Vec<String> = INPUT_0.lines().map(|line| line.to_string()).collect();

        let cave_graph = build_cave_graph(&input);
        let output = walkabout_cave_graph(&cave_graph, 1000);
        assert_eq!(output.len(), 10);
    }

    #[test]
    fn it_finds_valid_paths_in_input_1() {
        let input: Vec<String> = INPUT_1.lines().map(|line| line.to_string()).collect();
        let cave_graph = build_cave_graph(&input);
        let output = walkabout_cave_graph(&cave_graph, 1000);
        assert_eq!(output.len(), 19);
    }

    #[test]
    fn it_finds_valid_paths_in_input_2() {
        let input: Vec<String> = INPUT_2.lines().map(|line| line.to_string()).collect();
        let cave_graph = build_cave_graph(&input);
        let output = walkabout_cave_graph(&cave_graph, 100_000);
        assert_eq!(output.len(), 226);
    }
}
