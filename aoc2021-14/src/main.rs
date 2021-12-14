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
    let polymer_map = get_polymer_map(&polymer_rules);

    let output = model_polymerization(polymer, &polymer_map, 10);
    let mut counts: Vec<u64> = output.into_values().collect();
    counts.sort_unstable();
    let highest = counts.last().unwrap();
    let lowest = counts.first().unwrap();
    let difference = highest - lowest;
    dbg!(difference);

    let output = model_polymerization(polymer, &polymer_map, 40);
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

fn get_polymer_map(input: &[String]) -> PolymerRules {
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

fn polymerize(polymer: Polymer, polymer_map: &PolymerRules) -> Polymer {
    let mut output = "".to_string();

    let chars: Vec<char> = polymer.chars().collect();
    chars.clone().windows(2).for_each(|window| {
        let key = window
            .iter()
            .fold("".to_string(), |acc, x| acc + &x.to_string());
        if let Some(value) = polymer_map.get(&key) {
            output += &format!("{}{}", window[0], value,);
        }
    });

    output += chars.last().unwrap().to_string().as_str();

    output
}

fn agg_polymerize(
    agg_polymer: &PolymerInventory,
    polymer_rules: &PolymerRules,
) -> PolymerInventory {
    let mut output = PolymerInventory::new();

    // Each polymer pair in `agg_polymer` creates two new polymer pairs in the output
    // according to polymer rules
    for polymer_pair in agg_polymer.clone().into_keys() {
        if let Some(new_polymer) = polymer_rules.get(&polymer_pair) {
            let polymer_pair_split: Vec<char> = polymer_pair.chars().collect();
            let polymer_a = format!("{}{}", polymer_pair_split[0], new_polymer);
            let polymer_b = format!("{}{}", new_polymer, polymer_pair_split[1]);

            if let Some(count) = output.get_mut(&polymer_a) {
                *count += 1;
            } else {
                output.insert(polymer_a, 1);
            }

            if let Some(count) = output.get_mut(&polymer_b) {
                *count += 1;
            } else {
                output.insert(polymer_b, 1);
            }
        }
    }

    output
}

fn model_polymerization(
    polymer: &Polymer,
    polymer_map: &PolymerRules,
    steps: u64,
) -> PolymerInventory {
    let mut polymer = polymer.to_string();
    for _i in 0..steps {
        polymer = polymerize(polymer, &polymer_map);
    }

    count_polymers(&polymer)
}

fn agg_model_polymerization(
    inventory: PolymerInventory,
    polymer_rules: &PolymerRules,
    steps: u64,
) -> PolymerInventory {
    let mut inventory = inventory.clone();
    for _i in 0..steps {
        inventory = agg_polymerize(&inventory, polymer_rules);
    }

    inventory
}

fn agg_count_polymers(inventory: PolymerInventory) -> PolymerInventory {
    let mut left_map = PolymerInventory::new();
    let mut right_map = PolymerInventory::new();
    let mut output = PolymerInventory::new();

    for key in inventory.clone().into_keys() {
        let chars: Vec<char> = key.clone().chars().collect();
        let left_key = chars.get(0).unwrap().to_string();
        let right_key = chars.get(1).unwrap().to_string();

        if let Some(count) = inventory.get(&key) {
            if let Some(left_count) = left_map.get_mut(&left_key) {
                *left_count += count;
            } else {
                left_map.insert(left_key, *count);
            }
        }

        if let Some(count) = inventory.get(&key) {
            if let Some(right_count) = right_map.get_mut(&right_key) {
                *right_count += count;
            } else {
                right_map.insert(right_key, *count);
            }
        }
    }

    for left_key in left_map.clone().into_keys() {
        if let Some(&left_value) = left_map.get(&left_key) {
            if let Some(&right_value) = right_map.get(&left_key) {
                output.insert(left_key, left_value.max(right_value));
            }
        }
    }

    output
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
        let output = get_polymer_map(&polymer_rules);
        assert_eq!(output.into_keys().len(), 16);
    }

    #[test]
    fn it_polymerizes() {
        let inputs = split_input_by_blankline(INPUT);
        let polymer = &inputs[0].to_string();
        let polymer_rules: Vec<String> =
            inputs[1].split("\n").map(|line| line.to_string()).collect();
        let polymer_map = get_polymer_map(&polymer_rules);

        let polymer = polymer.to_string();
        let polymer = polymerize(polymer, &polymer_map);
        assert_eq!(polymer, "NCNBCHB");
        let polymer = polymerize(polymer, &polymer_map);
        assert_eq!(polymer, "NBCCNBBBCBHCB");
        let polymer = polymerize(polymer, &polymer_map);
        assert_eq!(polymer, "NBBBCNCCNBBNBNBBCHBHHBCHB");
        let polymer = polymerize(polymer, &polymer_map);
        assert_eq!(polymer, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");
    }

    #[test]
    fn it_counts_polymers() {
        let inputs = split_input_by_blankline(INPUT);
        let polymer = &inputs[0];
        let polymer_rules: Vec<String> =
            inputs[1].split("\n").map(|line| line.to_string()).collect();
        let polymer_map = get_polymer_map(&polymer_rules);

        let output = model_polymerization(polymer, &polymer_map, 10);

        let mut counts: Vec<u64> = output.into_values().collect();
        counts.sort_unstable();
        let highest = counts.last().unwrap();
        let lowest = counts.first().unwrap();
        let difference = highest - lowest;
        assert_eq!(difference, 1588);
    }

    #[test]
    fn it_agg_polymerizes() {
        let inputs = split_input_by_blankline(INPUT);
        let polymer_rules: Vec<String> =
            inputs[1].split("\n").map(|line| line.to_string()).collect();
        let polymer_map = get_polymer_map(&polymer_rules);

        // starting polymer is 'NNCB'
        let agg_polymer: HashMap<String, u64> = HashMap::from([
            ("NN".to_string(), 1),
            ("NC".to_string(), 1),
            ("CB".to_string(), 1),
        ]);

        //
        // 0. NNCB
        // - NCN NBC CHB
        // 1. NC CN NB BC CH HB
        // - NBC CCN BBB BBC CBH HCB
        // 2. NB BC CC CN BB BB BB BC CB BH HC CB

        let agg_polymer = agg_model_polymerization(agg_polymer, &polymer_map, 10);
        // assert_eq!(agg_polymer.clone().into_keys().len(), 6);
        // assert_eq!(agg_polymer["CH"], 1);
        // assert_eq!(agg_polymer["NC"], 1);
        // assert_eq!(agg_polymer["BC"], 1);
        // assert_eq!(agg_polymer["HB"], 1);
        // assert_eq!(agg_polymer["CN"], 1);
        // assert_eq!(agg_polymer["NB"], 1);

        let output = agg_count_polymers(agg_polymer);
        dbg!(output);
    }
}
