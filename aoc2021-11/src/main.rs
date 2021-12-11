use std::{collections::HashMap, hash::Hash};

fn main() {
    println!("Hello, world!");
}

fn build_octopus_grid(input: &[String]) -> HashMap<(i8, i8), i8> {
    let mut output: HashMap<(i8, i8), i8> = HashMap::new();

    for (y, row) in input.iter().enumerate() {
        for (x, value) in row.chars().enumerate() {
            let charge = value.to_digit(10).unwrap() as i8;
            output.insert((x as i8, y as i8), charge);
        }
    }

    output
}

fn debug_octogrid(octogrid: &HashMap<(i8, i8), i8>, width: i8, height: i8) -> Vec<Vec<i8>> {
    let mut output: Vec<Vec<i8>> = vec![];

    for y in 0..height {
        let mut row = vec![];
        for x in 0..width {
            let value = octogrid.get(&(x, y)).unwrap();
            row.push(*value);
        }
        output.push(row);
    }

    output
}

fn increment_octopus_energy_level(octogrid: &HashMap<(i8, i8), i8>) -> HashMap<(i8, i8), i8> {
    let mut output: HashMap<(i8, i8), i8> = HashMap::new();

    for position in octogrid.clone().into_keys() {
        let value = octogrid.get(&position).unwrap();
        output.insert(position, value + 1);
    }

    output
}

fn get_octopus_neighbors(position: (i8, i8), octogrid: &HashMap<(i8, i8), i8>) -> Vec<(i8, i8)> {
    [
        (position.0, position.1 - 1),
        (position.0, position.1 + 1),
        (position.0 - 1, position.1),
        (position.0 + 1, position.1),
    ]
    .iter()
    .filter_map(|neighbor| match octogrid.get(neighbor) {
        Some(_) => Some(*neighbor),
        None => None,
    })
    .collect()
}

#[derive(Eq, PartialEq, Hash)]
enum Action {
    Increment,
    Flash,
}

fn handle_charged_octopods(octogrid: &HashMap<(i8, i8), i8>) -> (HashMap<(i8, i8), i8>, u32) {
    let mut patch: HashMap<(i8, i8), Action> = HashMap::new();

    let mut flashes = 0;
    for position in octogrid.clone().into_keys() {
        let value = octogrid.get(&position).unwrap();
        if value > &9 {
            // This octopus will flash, incrementing the energy
            // level of it's neighbors by one and setting it's value to zero
            patch.insert(position, Action::Flash);
            let neighbors = get_octopus_neighbors(position, octogrid);
            for neighbor in neighbors {
                if patch.get(&neighbor).is_none() {
                    if value < &9 {
                        patch.insert(position, Action::Increment);
                    }
                }
            }
        }
    }

    // Apply the patch to the output
    let mut output: HashMap<(i8, i8), i8> = HashMap::new();
    for position in octogrid.clone().into_keys() {
        let value = octogrid.get(&position).unwrap();
        match patch.get(&position) {
            Some(Action::Increment) => {
                output.insert(position, value + 1);
            }
            Some(Action::Flash) => {
                flashes += 1;
                output.insert(position, 0);
            }
            _ => {
                output.insert(position, *value);
            }
        }
    }

    (output, flashes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_an_octopus_map() {
        let input: Vec<String> = r#"11111
19991
19191
19991
11111"#
            .to_string()
            .lines()
            .map(|value| value.to_string())
            .collect();

        let octogrid = build_octopus_grid(&input);
        assert_eq!(octogrid.get(&(0, 0)), Some(&1));
        assert_eq!(octogrid.get(&(3, 1)), Some(&9));
        assert_eq!(octogrid.get(&(1, 3)), Some(&9));
    }

    #[test]
    fn it_increments_octopus_energy() {
        let input: Vec<String> = r#"11111
19991
19191
19991
11111"#
            .to_string()
            .lines()
            .map(|value| value.to_string())
            .collect();

        let octogrid = build_octopus_grid(&input);
        let octogrid = increment_octopus_energy_level(&octogrid);
        assert_eq!(octogrid.get(&(0, 0)), Some(&2));
        assert_eq!(octogrid.get(&(3, 1)), Some(&10));
        assert_eq!(octogrid.get(&(1, 3)), Some(&10));
    }

    #[test]
    fn it_handles_charged_octopods() {
        let input: Vec<String> = r#"11111
19991
19191
19991
11111"#
            .to_string()
            .lines()
            .map(|value| value.to_string())
            .collect();

        let octogrid = build_octopus_grid(&input);
        let octogrid = increment_octopus_energy_level(&octogrid);
        let output = handle_charged_octopods(&octogrid);

        dbg!(debug_octogrid(&output.0, 5, 5));
    }
}
