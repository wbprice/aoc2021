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

    let cheapest_dest = chart_cheapest_alignment_destination(&initial_state);
    dbg!(cheapest_dest);
}

fn chart_cheapest_alignment_destination(input: &[i32]) -> i32 {
    let mut fuel_costs: HashMap<i32, i32> = HashMap::new();
    // Calcuate the fuel cost for each destination
    for destination in input {
        if let None = fuel_costs.get(destination) {
            let cost = calculate_alignment_fuel_cost(&input, *destination);
            fuel_costs.insert(cost, *destination);
        }
    }

    dbg!(&fuel_costs);

    // Return the position that is cheapest for everyone to travel to
    let lowest_cost = fuel_costs.clone().into_keys().min().unwrap();
    let dest = fuel_costs.get(&lowest_cost).unwrap();
    *dest
}

fn calculate_alignment_fuel_cost(input: &[i32], destination: i32) -> i32 {
    input
        .iter()
        .map(|&value| i32::abs(value - destination))
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

        let output = calculate_alignment_fuel_cost(&input, 2);
        assert_eq!(output, 37);
    }

    #[test]
    fn it_finds_the_cheapest_alignment_destination() {
        let input: Vec<i32> = "16,1,2,0,4,2,7,1,2,14"
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();

        let output = chart_cheapest_alignment_destination(&input);
        assert_eq!(output, 2);
    }
}
