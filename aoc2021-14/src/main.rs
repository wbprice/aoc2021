use std::collections::HashMap;
use std::fs;

type Polymer = String;
type PolymerMap = HashMap<String, String>;
type PolymerInventory = HashMap<String, u32>;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read the string");

    let inputs = split_input_by_blankline(&input);
    let polymer = &inputs[0].to_string();
    let polymer_rules: Vec<String> = inputs[1].split("\n").map(|line| line.to_string()).collect();
    let polymer_map = get_polymer_map(&polymer_rules);

    let output = model_polymerization(polymer, &polymer_map, 10);
    let mut counts: Vec<u32> = output.into_values().collect();
    counts.sort_unstable();
    let highest = counts.last().unwrap();
    let lowest = counts.first().unwrap();
    let difference = highest - lowest;
    dbg!(difference);

    let output = model_polymerization(polymer, &polymer_map, 40);
    let mut counts: Vec<u32> = output.into_values().collect();
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

fn get_polymer_map(input: &[String]) -> PolymerMap {
    let mut output: PolymerMap = HashMap::new();
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

fn polymerize(polymer: Polymer, polymer_map: &PolymerMap) -> Polymer {
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

fn model_polymerization(
    polymer: &Polymer,
    polymer_map: &PolymerMap,
    steps: u32,
) -> PolymerInventory {
    let mut polymer = polymer.to_string();
    for _i in 0..steps {
        polymer = polymerize(polymer, &polymer_map);
    }

    count_polymers(&polymer)
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

        let mut counts: Vec<u32> = output.into_values().collect();
        counts.sort_unstable();
        let highest = counts.last().unwrap();
        let lowest = counts.first().unwrap();
        let difference = highest - lowest;
        assert_eq!(difference, 1588);
    }
}
