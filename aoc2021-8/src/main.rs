use std::collections::HashMap;
use std::fmt::Display;
use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .expect("Couldn't read the input")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let output = part_oner(&input);
    dbg!(output);

    let output = part_twoer(&input);
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

fn part_twoer(input: &[String]) -> Option<u64> {
    let mut output = 0;

    for entry in input {
        let split = split_entry(&entry);
        let signals = split.get(0).expect("Couldn't get scrambled signals");
        let display = split.get(1).expect("Couldn't get scrambled display");
        let descrambler = get_descrambler(&signals);
        let value = display_descrambler(&display, &descrambler);
        output += value;
    }

    Some(output)
}

fn fits_a_four(input: &String, map: &HashMap<u8, String>) -> bool {
    let four = map.get(&4).unwrap();
    let mut i = 0;
    for c in four.chars() {
        for v in input.chars() {
            if c == v {
                i += 1;
            }
        }
    }

    i == 4
}

fn fits_a_seven(input: &String, map: &HashMap<u8, String>) -> bool {
    let seven = map.get(&7).unwrap();

    let mut i = 0;
    for c in seven.chars() {
        for v in input.chars() {
            if c == v {
                i += 1;
            }
        }
    }

    i == seven.len()
}

fn fits_a_one(input: &String, map: &HashMap<u8, String>) -> bool {
    let one = map.get(&1).unwrap();

    let mut i = 0;
    for c in one.chars() {
        for v in input.chars() {
            if c == v {
                i += 1;
            }
        }
    }

    i == one.len()
}

fn has_lower_right_segment(input: &String, map: &HashMap<Segment, String>) -> bool {
    let lower_right = map.get(&Segment::LowerRight).unwrap();

    for v in input.chars() {
        if v.to_string() == *lower_right {
            return true;
        }
    }

    false
}

fn get_common_segments(inputA: &String, inputB: &String) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for a in inputA.chars() {
        for b in inputB.chars() {
            if a == b {
                output.push(a.to_string());
            }
        }
    }

    output
}

#[derive(Eq, PartialEq, Hash)]
enum Segment {
    Top,
    Middle,
    UpperRight,
    LowerRight,
}

fn get_descrambler_stage0(input: &[String]) -> HashMap<u8, String> {
    let mut output: HashMap<u8, String> = HashMap::new();
    for value in input {
        match value.len() {
            2 => {
                output.insert(1, value.to_string());
            }
            4 => {
                output.insert(4, value.to_string());
            }
            3 => {
                output.insert(7, value.to_string());
            }
            7 => {
                output.insert(8, value.to_string());
            }
            _ => {
                // noop
            }
        }
    }
    output
}

fn get_descrambler(input: &[String]) -> HashMap<String, String> {
    let mut segments: HashMap<Segment, String> = HashMap::new();

    // Freebies
    // Figure out where 1, 4, 7, and 8 are based on token length
    let mut scratch = get_descrambler_stage0(&input);

    // Derived from freebies
    // First, figure out the 6 segment numbers
    let mut six_segments_found = 0;
    while six_segments_found < 3 {
        for value in input {
            match value.len() {
                6 => {
                    if fits_a_four(value, &scratch) {
                        scratch.insert(9, value.to_string());
                        six_segments_found += 1;
                    } else if !fits_a_four(value, &scratch) && fits_a_one(value, &scratch) {
                        scratch.insert(0, value.to_string());
                        six_segments_found += 1;
                    } else {
                        scratch.insert(6, value.to_string());
                        let one = scratch.get(&1).unwrap();
                        let common = get_common_segments(value, one);
                        segments.insert(Segment::LowerRight, common.get(0).unwrap().to_string());
                        six_segments_found += 1;
                    }
                }
                _ => {
                    // noop
                }
            }
        }
    }

    // Next, figure out the five segment numbers
    let mut five_segments_found = 0;
    while five_segments_found < 3 {
        for value in input {
            match value.len() {
                5 => {
                    if fits_a_seven(value, &scratch) {
                        scratch.insert(3, value.to_string());
                        five_segments_found += 1;
                    } else if has_lower_right_segment(value, &segments) {
                        scratch.insert(5, value.to_string());
                        five_segments_found += 1;
                    } else {
                        scratch.insert(2, value.to_string());
                        five_segments_found += 1;
                    }
                }
                _ => {
                    // noop
                }
            }
        }
    }

    // Invert scratch map for use with the output descrambler
    let mut output: HashMap<String, String> = HashMap::new();
    for key in scratch.clone().into_keys() {
        let mut split: Vec<String> = scratch
            .get(&key)
            .expect("Couldn't get key from scratch")
            .chars()
            .map(|value| value.to_string())
            .collect();
        split.sort_unstable();
        let k = split.iter().fold("".to_string(), |acc, value| acc + value);
        let v = format!("{}", key);
        output.insert(k.to_string(), v);
    }

    output
}

fn display_descrambler(input: &[String], map: &HashMap<String, String>) -> u64 {
    let mut display = "".to_string();
    for number in input {
        let mut characters: Vec<String> = number.chars().map(|value| value.to_string()).collect();
        characters.sort_unstable();
        let sorted = characters.iter().fold("".to_string(), |acc, x| acc + x);
        let number = map.get(&sorted).expect("Couldn't find descrambled value");
        display += &number.to_string();
    }

    display.parse().unwrap()
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
        let input: Vec<String> = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
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
    fn it_returns_true_if_a_four_would_fit() {
        let input: Vec<String> = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb"
            .to_string()
            .split_whitespace()
            .map(|value| value.to_string())
            .collect();

        let map = get_descrambler_stage0(&input);
        let output = fits_a_four(&"cbdgef".to_string(), &map);
        assert_eq!(output, true);

        let output = fits_a_four(&"agebfd".to_string(), &map);
        assert_eq!(output, false)
    }

    #[test]
    fn it_returns_true_if_a_seven_would_fit() {
        let input: Vec<String> = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb"
            .to_string()
            .split_whitespace()
            .map(|value| value.to_string())
            .collect();

        let map = get_descrambler_stage0(&input);
        let output = fits_a_seven(&"fecdb".to_string(), &map);
        assert_eq!(output, true);

        let output = fits_a_seven(&"fdcge".to_string(), &map);
        assert_eq!(output, false)
    }

    #[test]
    fn it_returns_true_if_a_one_would_fit() {
        let input: Vec<String> = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb"
            .to_string()
            .split_whitespace()
            .map(|value| value.to_string())
            .collect();

        let map = get_descrambler_stage0(&input);
        let output = fits_a_one(&"agebfd".to_string(), &map);
        assert_eq!(output, true);

        let output = fits_a_one(&"fgaecd".to_string(), &map);
        assert_eq!(output, false)
    }

    #[test]
    fn it_gets_a_hash_for_descrambling_output() {
        let input: Vec<String> = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb"
            .to_string()
            .split_whitespace()
            .map(|value| value.to_string())
            .collect();

        let output = get_descrambler(&input);
        assert_eq!(output.get(&"be".to_string()), Some(&"1".to_string()));
        assert_eq!(output.get(&"bde".to_string()), Some(&"7".to_string()));
        assert_eq!(output.get(&"bceg".to_string()), Some(&"4".to_string()));
        assert_eq!(output.get(&"bcdef".to_string()), Some(&"3".to_string()));
        assert_eq!(output.get(&"acdefg".to_string()), Some(&"6".to_string()));
        assert_eq!(output.get(&"bcdefg".to_string()), Some(&"9".to_string()));
        assert_eq!(output.get(&"abdefg".to_string()), Some(&"0".to_string()));
        assert_eq!(output.get(&"abcdefg".to_string()), Some(&"8".to_string()));
        assert_eq!(output.get(&"abcdf".to_string()), Some(&"2".to_string()));
        assert_eq!(output.get(&"cdefg".to_string()), Some(&"5".to_string()));
    }

    #[test]
    fn it_descrambles_output() {
        let input: Vec<String> = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb"
            .to_string()
            .split_whitespace()
            .map(|value| value.to_string())
            .collect();
        let display: Vec<String> = "fdgacbe cefdb cefbgd gcbe"
            .to_string()
            .split_whitespace()
            .map(|value| value.to_string())
            .collect();

        let descrambler = get_descrambler(&input);
        let output = display_descrambler(&display, &descrambler);
        assert_eq!(output, 8394);
    }

    #[test]
    fn it_sums_scrambled_display_values() {
        let input: Vec<String> = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
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

        let output = part_twoer(&input);
        assert_eq!(output, Some(61229));
    }
}
