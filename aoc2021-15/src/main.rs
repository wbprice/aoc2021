use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;

type Position = (i32, i32);
type Cost = i32;
type RiskMap = HashMap<Position, Cost>;
type CostMap = HashMap<Position, Cost>;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: Position,
}

// Impelment Ord so the queue becomes a min-heap queue
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    cost: Cost,
    position: Position,
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .expect("Couldn't read the input")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let risk_map = build_risk_map(&input, &(0, 0));
    let output = shortest_path(&(0, 0), &(99, 99), &risk_map);
    dbg!(output);

    let a_bigger_risk_map = build_bigger_risk_map(&input, 5, 5);
    let output = shortest_path(&(0, 0), &(499, 499), &a_bigger_risk_map);
    dbg!(output);
}

fn build_risk_map(input: &[String], tile: &Position) -> RiskMap {
    let rows = input.len() as i32;
    let columns = input[0].len() as i32;

    let mut output: HashMap<(i32, i32), i32> = HashMap::new();
    for (y, row) in input.iter().enumerate() {
        for (x, column) in row.chars().into_iter().enumerate() {
            // The danger generally increases the farther away from the origin the sub is
            let danger = column.to_digit(10).unwrap() as i32;
            let danger_bonus = tile.0.abs() + tile.1.abs();
            // Big risk maps get tiled
            let x_bonus = tile.0.abs() * columns;
            let y_bonus = tile.1.abs() * rows;
            let x = x as i32 + x_bonus;
            let y = y as i32 + y_bonus;

            if danger + danger_bonus > 9 {
                output.insert((x, y), (danger + danger_bonus) % 9);
            } else {
                output.insert((x, y), danger + danger_bonus);
            }
        }
    }
    output
}

fn build_bigger_risk_map(input: &[String], width: i32, height: i32) -> RiskMap {
    let mut output = RiskMap::new();
    for y in 0..height {
        for x in 0..width {
            output.extend(build_risk_map(input, &(x, y)));
        }
    }
    output
}

fn get_edges(here: Position, risk_map: &RiskMap) -> Vec<Edge> {
    [
        (here.0, here.1 - 1),
        (here.0, here.1 + 1),
        (here.0 - 1, here.1),
        (here.0 + 1, here.1),
    ]
    .into_iter()
    .filter_map(|position: Position| {
        risk_map.get(&position).map(|cost| Edge {
            position,
            cost: *cost,
        })
    })
    .collect()
}

// Adapted from https://doc.rust-lang.org/std/collections/binary_heap/index.html
fn shortest_path(start: &Position, finish: &Position, risk_map: &RiskMap) -> Option<i32> {
    // dist hashamp tracks the shortest distance from 'start' to a given 'node'
    let mut dist = risk_map
        .clone()
        .into_keys()
        .map(|position| (position, i32::MAX))
        .collect::<CostMap>();

    let mut heap = BinaryHeap::new();

    // Start at the start. Starting cost is zero
    if let Some(value) = dist.get_mut(start) {
        *value = 0;
    }
    heap.push(State {
        cost: 0,
        position: *start,
    });

    // Look through the rest of the grid, starting with lower cost nodes
    while let Some(State { cost, position }) = heap.pop() {
        // Is this the goal? Exit early
        if position == *finish {
            return Some(cost);
        }

        // If this neighbor is more expensive than other routes we already know about
        // skip it
        if cost > dist[&position] {
            continue;
        }

        // For each neighbor of the current position,
        // see if we can find a route with a lower cost
        for edge in get_edges(position, risk_map) {
            let next = State {
                cost: cost + edge.cost,
                position: edge.position,
            };

            // if this is cheaper, add it to the heap
            if next.cost < dist[&next.position] {
                heap.push(next);
                // We found a better way
                if let Some(value) = dist.get_mut(&next.position) {
                    *value = next.cost;
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn it_calculates_a_low_risk_path_to_the_finish() {
        let input: Vec<String> = INPUT
            .to_string()
            .lines()
            .map(|line| line.to_string())
            .collect();

        let risk_map = build_risk_map(&input, &(0, 0));
        let output = shortest_path(&(0, 0), &(9, 9), &risk_map);
        assert_eq!(output, Some(40));
    }

    #[test]
    fn it_builds_a_bigger_risk_map() {
        let input: Vec<String> = vec!["8".to_string()];
        let risk_map = build_bigger_risk_map(&input, 5, 5);
        assert_eq!(risk_map[&(0, 0)], 8);
        assert_eq!(risk_map[&(1, 0)], 9);
        assert_eq!(risk_map[&(0, 1)], 9);
        assert_eq!(risk_map[&(1, 1)], 1);
        assert_eq!(risk_map[&(4, 4)], 7);
    }

    #[test]
    fn it_calculates_a_low_risk_path_to_the_finish_on_a_big_map() {
        let input: Vec<String> = INPUT
            .to_string()
            .lines()
            .map(|line| line.to_string())
            .collect();

        let risk_map = build_bigger_risk_map(&input, 5, 5);
        let output = shortest_path(&(0, 0), &(49, 49), &risk_map);
        assert_eq!(output, Some(315));
    }
}
