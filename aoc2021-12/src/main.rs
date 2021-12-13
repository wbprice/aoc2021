use std::collections::HashMap;
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
    let paths = walkabout_cave_graph(&cave_graph, false);
    dbg!(paths.len());

    let paths = walkabout_cave_graph(&cave_graph, true);
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
    for edge in parse_edges(input) {
        let left = edge.get(0).unwrap().to_string();
        let right = edge.get(1).unwrap().to_string();

        graph.insert(left, vec![]);
        graph.insert(right, vec![]);
    }

    // Populate each node's outgoing edges
    for edge in parse_edges(input) {
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

fn should_goto_cave(node: &str, breadcrumbs: &[String], scenic_route: bool) -> bool {
    if node == node.to_lowercase() && node != "end" {
        if scenic_route {
            let small_cave_visits: HashMap<String, u8> = breadcrumbs
                .iter()
                .filter(|&p| *p == *p.to_lowercase())
                .fold(HashMap::new(), |mut acc, crumb| {
                    if let Some(visit) = acc.get_mut(crumb) {
                        *visit += 1;
                    } else {
                        acc.insert(crumb.to_string(), 1);
                    }
                    acc
                });

            // Has this cave been visited before?
            if small_cave_visits.get(node).is_some() {
                // This is fine as long as another cave hasn't been visited twice already
                return !small_cave_visits.into_values().any(|visits| visits > 1);
            }
            return true;
        } else {
            return !breadcrumbs.contains(&node.to_string());
        }
    }

    true
}

fn walk_cave_graph(
    node: &str,
    cave_graph: &CaveGraph,
    breadcrumbs: Vec<String>,
    output: &mut Vec<Path>,
    scenic_route: bool,
) {
    // We have successfully reached the end
    if node == "end" {
        output.push(breadcrumbs.clone().join(","));
        return;
    }

    if let Some(edges) = cave_graph.get(node) {
        let valid_edges: Vec<&String> = edges
            .iter()
            .filter(|&edge| edge != "start")
            .filter(|&edge| should_goto_cave(edge.as_str(), &breadcrumbs, scenic_route))
            .collect();

        // If there are no valid edges now, this isn't a valid path
        // and we should quit now
        if valid_edges.is_empty() {
            return;
        }

        for edge in valid_edges {
            let mut next_breadcrumbs = breadcrumbs.clone();
            next_breadcrumbs.push(edge.to_string());
            walk_cave_graph(edge, cave_graph, next_breadcrumbs, output, scenic_route);
        }
    }
}

fn walkabout_cave_graph(cave_graph: &CaveGraph, scenic_route: bool) -> Vec<Path> {
    let mut output: Vec<Path> = vec![];

    walk_cave_graph(
        &"start".to_string(),
        cave_graph,
        vec![],
        &mut output,
        scenic_route,
    );

    output
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
        let mut output: Vec<Path> = vec![];
        walk_cave_graph(
            &"start".to_string(),
            &cave_graph,
            vec![],
            &mut output,
            false,
        );
        assert!(true);
    }

    #[test]
    fn it_finds_valid_paths_in_input_0() {
        let input: Vec<String> = INPUT_0.lines().map(|line| line.to_string()).collect();

        let cave_graph = build_cave_graph(&input);
        let output = walkabout_cave_graph(&cave_graph, false);
        assert_eq!(output.len(), 10);
    }

    #[test]
    fn it_finds_valid_paths_in_input_1() {
        let input: Vec<String> = INPUT_1.lines().map(|line| line.to_string()).collect();
        let cave_graph = build_cave_graph(&input);
        let output = walkabout_cave_graph(&cave_graph, false);
        assert_eq!(output.len(), 19);
    }

    #[test]
    fn it_finds_valid_paths_in_input_2() {
        let input: Vec<String> = INPUT_2.lines().map(|line| line.to_string()).collect();
        let cave_graph = build_cave_graph(&input);
        let output = walkabout_cave_graph(&cave_graph, false);
        assert_eq!(output.len(), 226);
    }

    #[test]
    fn it_takes_the_scenic_route_in_input_0() {
        let input: Vec<String> = INPUT_0.lines().map(|line| line.to_string()).collect();
        let cave_graph = build_cave_graph(&input);
        let output = walkabout_cave_graph(&cave_graph, true);
        assert_eq!(output.len(), 36);
    }

    #[test]
    fn it_takes_the_scenic_route_in_input_1() {
        let input: Vec<String> = INPUT_1.lines().map(|line| line.to_string()).collect();
        let cave_graph = build_cave_graph(&input);
        let output = walkabout_cave_graph(&cave_graph, true);
        assert_eq!(output.len(), 103);
    }

    #[test]
    fn it_takes_the_scenic_route_in_input_2() {
        let input: Vec<String> = INPUT_2.lines().map(|line| line.to_string()).collect();
        let cave_graph = build_cave_graph(&input);
        let output = walkabout_cave_graph(&cave_graph, true);
        assert_eq!(output.len(), 3509);
    }
}
