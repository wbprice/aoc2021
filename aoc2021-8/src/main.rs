use std::collections::HashMap;
use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .expect("Couldn't read the input")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let output = part_oner(&input);
    dbg!(output);
}

fn split_entry(input: &String) -> Vec<Vec<String>> {
    input
        .split(" | ")
        .map(|value| {
            value
                .split_whitespace()
                .map(|value| value.to_string())
                .collect()
        })
        .collect()
}

fn get_display_segment_count(input: &[String]) -> Vec<u8> {
    input.iter().map(|val| val.len() as u8).collect()
}

fn part_oner(input: &[String]) -> Option<u64> {
    let mut counts: HashMap<u8, u64> = HashMap::new();
    for entry in input {
        let split = split_entry(&entry);
        if let Some(display) = split.get(1) {
            let display_count = get_display_segment_count(&display);
            for count in display_count {
                if let Some(value) = counts.get_mut(&count) {
                    *value += 1;
                } else {
                    counts.insert(count, 1);
                }
            }
        }
    }

    let mut letters: HashMap<u8, u64> = HashMap::new();
    letters.insert(1, *counts.get(&2).unwrap());
    letters.insert(4, *counts.get(&4).unwrap());
    letters.insert(7, *counts.get(&3).unwrap());
    letters.insert(8, *counts.get(&7).unwrap());

    Some(letters.into_values().sum::<u64>())
}

fn get_pattern_matcher_map() -> HashMap<String, char> {
    HashMap::from([
        ("abcdeg".to_string(), '0'),
        ("ab".to_string(), '1'),
        ("acdfg".to_string(), '2'),
        ("abcdf".to_string(), '3'),
        ("abef".to_string(), '4'),
        ("bcdef".to_string(), '5'),
        ("bcdefg".to_string(), '6'),
        ("abd".to_string(), '7'),
        ("abcdefg".to_string(), '8'),
        ("abcdef".to_string(), '9'),
    ])
}

fn signal_pattern_matcher(input: String, map: HashMap<String, char>) -> u64 {
    // let mut characters: Vec<String> = input.split_whitespace().map(|value| value.to_string()).collect();
    let signals: Vec<String> = input
        .split_whitespace()
        .map(|value| value.to_string())
        .collect();

    let string: String = signals
        .iter()
        .map(|signal| {
            let mut characters: Vec<String> =
                signal.chars().map(|value| value.to_string()).collect();
            characters.sort_unstable();

            let sorted = characters
                .iter()
                .fold("".to_string(), |acc, value| acc + value);

            dbg!(&sorted);
            map.get(&sorted)
                .expect("This value didn't appear in the map")
        })
        .collect();

    dbg!(&string);

    string.parse().unwrap()
}

fn input_descrambler(input: &[String]) -> HashMap<String, char> {
    let mut output: HashMap<String, char> = HashMap::new();
    // Figure out where 1, 4, 7, and 8 are:
    for value in input {
        match value.len() {
            2 => {
                output.insert(value.to_string(), '1');
            }
            4 => {
                output.insert(value.to_string(), '4');
            }
            3 => {
                output.insert(value.to_string(), '7');
            }
            7 => {
                output.insert(value.to_string(), '8');
            }
            _ => {
                // noop
            }
        }
    }

    output
}

fn output_descrambler(input: String, map: HashMap<String, char>) -> u16 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_splits_an_entry() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
            .to_string();

        let split = split_entry(&input);
        let inputs = split.get(0).unwrap();
        let display = split.get(1).unwrap();
        assert_eq!(split.len(), 2);
        assert_eq!(inputs.len(), 10);
        assert_eq!(display.len(), 4);
    }

    #[test]
    fn it_counts_the_length_of_each_item_in_an_entry_output() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
            .to_string();
        let split = split_entry(&input);
        let display = split.get(1).unwrap();
        let output = get_display_segment_count(&display);
        assert_eq!(output, &[7, 5, 6, 4]);
    }

    #[test]
    fn it_counts_ones_fours_sevens_and_eights() {
        let input: Vec<String> = r#"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#
            .lines()
            .map(|line| line.to_string())
            .collect();

        let output = part_oner(&input);
        assert_eq!(output, Some(26));
    }

    #[test]
    fn it_reads_outputs_into_numbers() {
        let input = "fdgacbe cefdb cefbgd gcbe".to_string();
        let dictionary = get_pattern_matcher_map();
        let output = signal_pattern_matcher(input, dictionary);
    }

    #[test]
    fn it_descrambles_inputs() {
        let input: Vec<String> = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb"
            .to_string()
            .split_whitespace()
            .map(|value| value.to_string())
            .collect();

        let output = input_descrambler(&input);
        dbg!(output);
    }
}
