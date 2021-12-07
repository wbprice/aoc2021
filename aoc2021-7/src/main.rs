use std::collections::HashMap;
use std::fs;

fn main() {
    let initial_state: Vec<i32> = fs::read_to_string("input")
        .expect("couldn't read the input file")
        .split(',')
        .map(|value| {
            value
                .parse()
                .expect("couldn't convert a string into a number")
        })
        .collect();

    let cost = get_cheapest_destination_cost(&initial_state);
    dbg!(cost);

    let cost = get_cheapest_destination_cost_v2(&initial_state);
    dbg!(cost);
}

fn get_cheapest_destination_cost(input: &[i32]) -> Option<i32> {
    // Calcuate the fuel cost for each destination
    let mut fuel_costs: HashMap<i32, i32> = HashMap::new();
    for destination in input {
        if fuel_costs.get(destination).is_none() {
            let cost = calculate_destination_fuel_cost(input, *destination);
            fuel_costs.insert(cost, *destination);
        }
    }

    // Return the cost of the cheapest destination
    fuel_costs.into_keys().min()
}

fn calculate_destination_fuel_cost(input: &[i32], destination: i32) -> i32 {
    input
        .iter()
        .map(|&value| i32::abs(value - destination))
        .sum::<i32>()
}

fn get_cheapest_destination_cost_v2(input: &[i32]) -> Option<i32> {
    let min = input.iter().min().expect("Couldn't find the minimum");
    let max = input.iter().max().expect("Couldn't find the maximum");

    // Calcuate the fuel cost for each destination
    let mut fuel_costs: HashMap<i32, i32> = HashMap::new();
    // For each possible destination
    for destination in *min..*max {
        // Calculate how expensive it would be to move everyone there
        if fuel_costs.get(&destination).is_none() {
            fuel_costs.insert(
                calculate_destination_fuel_cost_v2(input, destination),
                destination,
            );
        }
    }

    // Return the cost of the cheapest destination
    fuel_costs.clone().into_keys().min()
}

fn calculate_destination_fuel_cost_v2(input: &[i32], destination: i32) -> i32 {
    input
        .iter()
        .map(|&value| {
            let steps = i32::abs(value - destination);
            (0..steps + 1).sum::<i32>()
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_calculates_the_fuel_cost_to_align_crabs() {
        let input: Vec<i32> = "16,1,2,0,4,2,7,1,2,14"
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();

        let output = calculate_destination_fuel_cost(&input, 2);
        assert_eq!(output, 37);
    }

    #[test]
    fn it_finds_the_cheapest_destination_cost() {
        let input: Vec<i32> = "16,1,2,0,4,2,7,1,2,14"
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();

        let output = get_cheapest_destination_cost(&input);
        assert_eq!(output, Some(37));
    }

    #[test]
    fn it_calculates_the_fuel_cost_to_align_crabs_v2() {
        let input: Vec<i32> = "16,1,2,0,4,2,7,1,2,14"
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();

        let output = calculate_destination_fuel_cost_v2(&input, 5);
        assert_eq!(output, 168);
        let output = calculate_destination_fuel_cost_v2(&input, 2);
        assert_eq!(output, 206);
    }

    #[test]
    fn it_finds_the_cheapest_destination_cost_v2() {
        let input: Vec<i32> = "16,1,2,0,4,2,7,1,2,14"
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();

        let output = get_cheapest_destination_cost_v2(&input);
        assert_eq!(output, Some(168));
    }
}
