use std::collections::HashMap;
use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .expect("Couldn't read the input")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let (_octogrid, flashes) = model_octopod_flashes(&input, 100);
    dbg!(flashes);

    let (_octogrid, step) = model_octopod_sync_flash(&input, 500).unwrap();
    dbg!(step);
}

fn model_octopod_flashes(input: &[String], steps: u32) -> (HashMap<(i8, i8), i8>, u32) {
    let mut flashes = 0;
    let mut octogrid = build_octopus_grid(input);

    for _step in 0..steps {
        octogrid = increment_octopus_energy_level(&octogrid);
        let (new_octogrid, new_flashes) = handle_charged_octopods(&octogrid);
        octogrid = new_octogrid;
        flashes += new_flashes;
    }

    (octogrid, flashes)
}

fn model_octopod_sync_flash(input: &[String], steps: u32) -> Option<(HashMap<(i8, i8), i8>, u32)> {
    let mut octogrid = build_octopus_grid(input);
    for step in 0..steps {
        octogrid = increment_octopus_energy_level(&octogrid);
        let (new_octogrid, _flashes) = handle_charged_octopods(&octogrid);
        octogrid = new_octogrid;
        if check_octopod_flash_synchronization(&octogrid) {
            return Some((octogrid, step));
        }
    }

    None
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
    .filter_map(|neighbor| octogrid.get(neighbor).map(|_| *neighbor))
    .collect()
}

fn handle_charged_octopods(octogrid: &HashMap<(i8, i8), i8>) -> (HashMap<(i8, i8), i8>, u32) {
    let mut output: HashMap<(i8, i8), i8> = HashMap::new();

    // Whatever, copy the initial state over first
    for position in octogrid.clone().into_keys() {
        if let Some(charge) = octogrid.get(&position) {
            output.insert(position, *charge);
        }
    }

    // Build the initial set of octopods that will flash
    let mut flashed: Vec<(i8, i8)> = vec![];
    let mut handle_flash: Vec<(i8, i8)> = output
        .clone()
        .into_keys()
        .filter(|position| match output.get(position) {
            Some(charge) => charge > &9,
            None => false,
        })
        .collect();

    while !handle_flash.is_empty() {
        if let Some(position) = handle_flash.pop() {
            // If this octopus has already flashed, don't handle the flash again
            if flashed.contains(&position) {
                continue;
            } else {
                flashed.push(position);
            }

            // Look up any neighbors, excluding those who have already flashed
            let neighbors: Vec<(i8, i8)> = get_octopus_neighbors(position, &output)
                .into_iter()
                .collect();

            for neighbor in neighbors {
                if let Some(charge) = output.get(&neighbor) {
                    // Otherwise, increment it's energy level by one.
                    let new_charge = charge + 1;
                    output.insert(neighbor, new_charge);
                    // If this octopodd's charge level is above 9 add it to the list to check
                    if new_charge > 9 {
                        handle_flash.push(neighbor);
                    }
                }
            }
        }
    }

    // Set the charge of any flashed octopods to zero
    let mut flashes = 0;
    for position in flashed {
        output.insert(position, 0);
        flashes += 1;
    }

    (output, flashes)
}

fn check_octopod_flash_synchronization(octogrid: &HashMap<(i8, i8), i8>) -> bool {
    for &charge in octogrid.clone().values() {
        if charge > 1 {
            return false;
        }
    }

    true
}

fn debug_octogrid(octogrid: &HashMap<(i8, i8), i8>, width: i8, height: i8) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for y in 0..height {
        let mut row = vec![];
        for x in 0..width {
            let value = octogrid.get(&(x, y)).unwrap();
            row.push(*value);
        }
        let text: String = row
            .iter()
            .fold("".to_string(), |acc, x| acc + &x.to_string());

        output.push(text)
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;

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
        assert_eq!(output.1, 9);
    }

    #[test]
    fn it_simulates_octopod_flash_intervals() {
        let input: Vec<String> = INPUT.lines().map(|line| line.to_string()).collect();

        let (_octogrid, flashes) = model_octopod_flashes(&input, 100);
        assert_eq!(flashes, 1656);
    }

    #[test]
    fn it_predicts_when_all_the_octopods_will_flash_together() {
        let input: Vec<String> = INPUT.lines().map(|line| line.to_string()).collect();

        let (_octogrid, step) = model_octopod_sync_flash(&input, 200).unwrap();
        assert_eq!(step, 194);
    }
}
