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
    // let paths = walkabout_cave_graph(&cave_graph, 10_000_000, false);
    // dbg!(paths.len());

    let paths = walkabout_cave_graph(&cave_graph, 100_000_000, true);
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

fn should_goto_cave(cave: String, path: &[String], scenic_route: bool) -> bool {
    if *cave == *cave.to_lowercase() && &cave != "end" {
        if scenic_route {
            // LOL
            let mut small_cave_visits: HashMap<String, u8> = HashMap::new();
            for p in path
                .iter()
                .filter(|&p| *p == *p.to_lowercase())
                .collect::<Vec<&String>>()
            {
                if let Some(visit) = small_cave_visits.get_mut(p) {
                    *visit += 1;
                } else {
                    small_cave_visits.insert(p.to_string(), 1);
                }
            }

            // Has this cave been visited before?
            if let Some(this_cave_visits) = small_cave_visits.get(&cave) {
                // This is fine as long as another cave hasn't been visited twice already
                let caves_visited_twice: Vec<u8> = small_cave_visits
                    .clone()
                    .into_values()
                    .filter(|&visits| visits > 1)
                    .collect();

                return caves_visited_twice.len() < 1;
            }
            return true;
        } else {
            return !path.contains(&cave);
        }
    } else if cave != "start" {
        return true;
    }

    false
}

fn walk_cave_graph(cave_graph: &CaveGraph, scenic_route: bool) -> Path {
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
                    .filter(|&destination| destination != "start")
                    .filter(|&destination| {
                        should_goto_cave(destination.to_string(), &output, scenic_route)
                    })
                    .collect();

                // If there are no valid destinations
                if valid_destinations.is_empty() {
                    break;
                }

                // Pick the next destination randomly from what's available
                let pick = rng.gen_range(0..valid_destinations.len());

                let next = valid_destinations.get(pick).unwrap();
                output.push(next.to_string());
            }
        }
    }

    // Join the breadcrumbs into a string
    output.join(",")
}

fn walkabout_cave_graph(cave_graph: &CaveGraph, attempts: u32, scenic_route: bool) -> Vec<Path> {
    let mut output: HashSet<Path> = HashSet::new();

    for attempt in 0..attempts {
        dbg!(attempt);
        output.insert(walk_cave_graph(cave_graph, scenic_route));
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
        walk_cave_graph(&cave_graph, false);
        assert!(true);
    }

    #[test]
    fn it_finds_valid_paths_in_input_0() {
        let input: Vec<String> = INPUT_0.lines().map(|line| line.to_string()).collect();

        let cave_graph = build_cave_graph(&input);
        let output = walkabout_cave_graph(&cave_graph, 1000, false);
        assert_eq!(output.len(), 10);
    }

    #[test]
    fn it_finds_valid_paths_in_input_1() {
        let input: Vec<String> = INPUT_1.lines().map(|line| line.to_string()).collect();
        let cave_graph = build_cave_graph(&input);
        let output = walkabout_cave_graph(&cave_graph, 1000, false);
        assert_eq!(output.len(), 19);
    }

    #[test]
    fn it_finds_valid_paths_in_input_2() {
        let input: Vec<String> = INPUT_2.lines().map(|line| line.to_string()).collect();
        let cave_graph = build_cave_graph(&input);
        let output = walkabout_cave_graph(&cave_graph, 100_000, false);
        assert_eq!(output.len(), 226);
    }

    #[test]
    fn it_takes_the_scenic_route_in_input_0() {
        let input: Vec<String> = INPUT_0.lines().map(|line| line.to_string()).collect();
        let cave_graph = build_cave_graph(&input);
        let output = walkabout_cave_graph(&cave_graph, 10_000, true);
        assert_eq!(output.len(), 36);
    }

    #[test]
    fn it_takes_the_scenic_route_in_input_1() {
        let input: Vec<String> = INPUT_1.lines().map(|line| line.to_string()).collect();
        let cave_graph = build_cave_graph(&input);
        let output = walkabout_cave_graph(&cave_graph, 10_000, true);
        assert_eq!(output.len(), 103);
    }

    #[test]
    fn it_takes_the_scenic_route_in_input_2() {
        let input: Vec<String> = INPUT_2.lines().map(|line| line.to_string()).collect();
        let cave_graph = build_cave_graph(&input);
        let output = walkabout_cave_graph(&cave_graph, 500_000, true);
        assert_eq!(output.len(), 3509);
    }
}
