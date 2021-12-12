use std::collections::HashMap;

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
        (position.0, position.1 - 1),     // top
        (position.0 + 1, position.1 - 1), // top right
        (position.0 + 1, position.1),     // right
        (position.0 + 1, position.1 + 1), // bottom right
        (position.0, position.1 + 1),     // bottom
        (position.0 - 1, position.1 + 1), // bottom left
        (position.0 - 1, position.1),     // left
        (position.0 - 1, position.1 - 1), // top left
    ]
    .iter()
    .filter_map(|neighbor| match octogrid.get(neighbor) {
        Some(_) => Some(*neighbor),
        None => None,
    })
    .collect()
}

fn handle_charged_octopods(octogrid: &HashMap<(i8, i8), i8>) -> (HashMap<(i8, i8), i8>, u32) {
    let mut flashes = 0;
    let mut output: HashMap<(i8, i8), i8> = HashMap::new();

    // Whatever, copy the initial state over first
    for position in octogrid.clone().into_keys() {
        if let Some(charge) = octogrid.get(&position) {
            output.insert(position, *charge);
        }
    }

    // Build the initial set of octopods that will flash
    let mut flashed: Vec<(i8, i8)> = vec![];
    let mut should_flash: Vec<(i8, i8)> = output
        .clone()
        .into_keys()
        .filter(|position| match output.get(&position) {
            Some(charge) => charge > &9,
            None => false,
        })
        .collect();

    while !should_flash.is_empty() {
        if let Some(position) = should_flash.pop() {
            // Increment the flashes count and set the new value of this octopus to zero
            flashes += 1;
            flashed.push(position);

            // Increment the energy count of any neighbors by one, adding them to the list of
            // octopuses that will flash if their charge goes over 9
            let neighbors: Vec<(i8, i8)> = get_octopus_neighbors(position, &output)
                .into_iter()
                .filter(|neighbor| !flashed.contains(neighbor))
                .collect();
            for neighbor in neighbors {
                if let Some(charge) = output.get(&neighbor) {
                    // Otherwise, increment it's energy level by one.
                    let new_charge = charge + 1;
                    output.insert(neighbor, new_charge);
                    // Push this position to the list of octopods that should flash
                    if new_charge > 9 {
                        should_flash.push(position);
                    }
                }
            }
        }
    }

    (output, flashes)
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
        let output = handle_charged_octopods(&octogrid);

        dbg!(debug_octogrid(&output.0, 5, 5));
    }
}
