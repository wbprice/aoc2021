use std::collections::HashMap;
use std::fs;

type Polymer = String;
type PolymerRules = HashMap<String, String>;
type PolymerInventory = HashMap<String, u64>;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read the string");

    let inputs = split_input_by_blankline(&input);
    let polymer = &inputs[0].to_string();
    let polymer_rules: Vec<String> = inputs[1].split("\n").map(|line| line.to_string()).collect();
    let polymer_map = get_polymer_rules(&polymer_rules);

    let output = model_polymerization(polymer, 10, &polymer_map);
    let mut counts: Vec<u64> = output.into_values().collect();
    counts.sort_unstable();
    let highest = counts.last().unwrap();
    let lowest = counts.first().unwrap();
    let difference = highest - lowest;
    dbg!(difference);

    let output = model_polymerization(polymer, 20, &polymer_map);
    let mut counts: Vec<u64> = output.into_values().collect();
    counts.sort_unstable();
    let highest = counts.last().unwrap();
    let lowest = counts.first().unwrap();
    let difference = highest - lowest;
    dbg!(difference);
}

fn split_input_by_blankline(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|string| string.to_string())
        .collect()
}

fn get_polymer_rules(input: &[String]) -> PolymerRules {
    let mut output: PolymerRules = HashMap::new();
    for line in input {
        let split: Vec<&str> = line.split(" -> ").collect();
        output.insert(split[0].to_string(), split[1].to_string());
    }
    output
}

fn count_polymers(polymer: &str) -> PolymerInventory {
    let mut output = HashMap::new();

    let chars: Vec<char> = polymer.chars().collect();
    for c in chars {
        if let Some(count) = output.get_mut(&c.to_string()) {
            *count += 1;
        } else {
            output.insert(c.to_string(), 1);
        }
    }

    output
}

fn polymerize(
    polymer: Polymer,
    steps: u64,
    rules: &PolymerRules,
    inventory: &mut PolymerInventory,
) {
    // Base case
    if steps == 0 {
        return;
    }

    // Recursive case
    let chars: Vec<char> = polymer.chars().collect();
    chars.clone().windows(2).for_each(|window| {
        let key = window
            .iter()
            .fold("".to_string(), |acc, x| acc + &x.to_string());
        if let Some(new_polymer) = rules.get(&key) {
            // Add the new polymer to the inventory
            if let Some(value) = inventory.get_mut(new_polymer) {
                *value += 1;
            } else {
                inventory.insert(new_polymer.to_string(), 1);
            }

            // Recurse with new polymers
            polymerize(
                format!("{}{}", window[0], new_polymer),
                steps - 1,
                rules,
                inventory,
            );
            polymerize(
                format!("{}{}", new_polymer, window[1]),
                steps - 1,
                rules,
                inventory,
            );
        }
    })
}

fn polymerize_v2(
    polymer: Polymer,
    depth: u64,
    rules: &PolymerRules,
    inventory: PolymerInventory,
) -> PolymerInventory {
    // Base case
    // Return the inventory when the maximum depth is reached
    if depth == 0 {
        return inventory;
    }

    // Recursive case
    // For each window in polymer, polymerize one layer lower
    polymer
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .fold(inventory, |inventory, window| {
            let polymer_pair = format!("{}{}", window[0], window[1]);
            if let Some(new_polymer) = rules.clone().get(&polymer_pair) {
                // Update inventory
                let mut next_inventory = inventory.clone();
                if let Some(value) = inventory.get(new_polymer) {
                    next_inventory.insert(new_polymer.to_string(), value + 1);
                } else {
                    next_inventory.insert(new_polymer.to_string(), 1);
                }

                // Call polymerize on both new polymers
                vec![
                    format!("{}{}", window[0], new_polymer),
                    format!("{}{}", new_polymer, window[1]),
                ]
                .iter()
                .fold(next_inventory, |inventory, polymer| {
                    polymerize_v2(polymer.to_string(), depth - 1, &rules, inventory)
                })
            } else {
                inventory
            }
        })
}

fn polymerize_v3(polymer: &Polymer, steps: u64, rules: &PolymerRules) -> PolymerInventory {
    let mut inventory = count_polymers(polymer);
    for _ in 0..steps {
        let mut output = PolymerInventory::new();
        for polymer in inventory.clone().keys() {
            if let Some(count) = inventory.get(polymer) {
                if let Some(new_element) = rules.get(polymer) {
                    let mut elements = polymer.chars();
                    let left_polymer = format!("{}{}", elements.nth(0).unwrap(), new_element);
                    let right_polymer = format!("{}{}", new_element, elements.nth(1).unwrap());

                    if let Some(left_polymer) = inventory.get_mut(&left_polymer) {
                        *left_polymer += count;
                    } else {
                        inventory.insert(left_polymer, *count);
                    }

                    if let Some(right_polymer) = inventory.get_mut(&right_polymer) {
                        *right_polymer += count;
                    } else {
                        inventory.insert(right_polymer, *count);
                    }

                }
            }
        }
    }
    output
}

fn model_polymerization(polymer: &Polymer, steps: u64, rules: &PolymerRules) -> PolymerInventory {
    let polymer_inventory = count_polymers(&polymer);
    polymerize_v2(polymer.to_string(), steps, &rules, polymer_inventory)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

    #[test]
    fn it_creates_a_polymer_map() {
        let inputs = split_input_by_blankline(INPUT);
        let polymer_rules: Vec<String> =
            inputs[1].split("\n").map(|line| line.to_string()).collect();
        let output = get_polymer_rules(&polymer_rules);
        assert_eq!(output.into_keys().len(), 16);
    }

    #[test]
    fn it_polymerizes() {
        let inputs = split_input_by_blankline(INPUT);
        let polymer = &inputs[0].to_string();
        let polymer_rules: Vec<String> =
            inputs[1].split("\n").map(|line| line.to_string()).collect();
        
        let polymer_rules = get_polymer_rules(&polymer_rules);
        let polymer_inventory = count_polymers(polymer);
        let output =
            polymerize_v2(polymer.to_string(), 10, &polymer_rules, polymer_inventory);

        let mut counts: Vec<u64> = output.into_values().collect();
        counts.sort_unstable();
        let highest = counts.last().unwrap();
        let lowest = counts.first().unwrap();
        let difference = highest - lowest;
        assert_eq!(difference, 1588);
    }

    fn it_polymerizes_faster() {
        let inputs = split_input_by_blankline(INPUT);
        let polymer = &inputs[0].to_string();
        let polymer_rules: Vec<String> =
            inputs[1].split("\n").map(|line| line.to_string()).collect();
            
        let rules = get_polymer_rules(&polymer_rules);
        let inventory = count_polymers(polymer); 

        let output = polymerize_v3(inventory, rules);
    }
}
