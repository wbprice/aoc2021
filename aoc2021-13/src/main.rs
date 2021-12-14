use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read the input file");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &String) {
    let inputs = split_input_by_blankline(&input);
    let pairs = get_pairs(&inputs[0]);
    let paper = Paper::new(&pairs);
    // fold along x=655
    let paper = paper.fold_at_x(655);
    dbg!(paper.count_visible_dots());
}

fn part_two(input: &String) {
    let inputs = split_input_by_blankline(&input);
    let pairs = get_pairs(&inputs[0]);
    let instructions = get_folding_directions(&inputs[1]);

    let mut paper = Paper::new(&pairs);
    // Fold the paper according to the directions
    for instruction in instructions  {
        match instruction.0 {
            "x" => {
                paper = paper.fold_at_x(instruction.1);
            },
            "y" => {
                paper = paper.fold_at_y(instruction.1);
            }
            _ => {
                // noop
            }
        }
    }
    paper.print();
}

type Pair = Vec<u32>;
#[derive(Debug)]
struct Paper {
    columns: u32,
    rows: u32,
    content: HashMap<(u32, u32), char>,
}

impl Paper {
    fn new(pairs: &[Vec<u32>]) -> Self {
        let mut columns = 0;
        let mut rows = 0;
        for pair in pairs {
            if pair[0] > columns {
                columns = pair[0];
            }
            if pair[1] > rows {
                rows = pair[1];
            }
        }

        Paper {
            rows,
            columns,
            content: pairs.iter().fold(HashMap::new(), |mut acc, pair| {
                acc.insert((pair[0], pair[1]), '#');
                acc
            }),
        }
    }

    fn print(&self) {
        println!("{} x {} Paper", self.columns + 1, self.rows + 1);
        for y in 0..self.rows + 1 {
            let mut row = vec![];
            for x in 0..self.columns + 1 {
                match self.content.get(&(x, y)) {
                    Some(_) => {
                        row.push('#');
                    }
                    None => row.push('.'),
                }
            }
            let text: String = row
                .iter()
                .fold("".to_string(), |acc, x| acc + &x.to_string());
            println!("{}", text);
        }
        println!("");
    }

    fn count_visible_dots(&self) -> u32 {
        let mut output = 0;
        for y in 0..self.rows + 1 {
            for x in 0..self.columns + 1 {
                if self.content.get(&(x, y)).is_some() {
                    output += 1;
                }
            }
        }
        output
    }

    fn fold_at_y(&self, fold_at: u32) -> Paper {
        let mut content = HashMap::new();
        for y in 0..self.rows + 1 {
            for x in 0..self.columns + 1 {
                if self.content.get(&(x, y)).is_some() {
                    // For rows above the fold
                    // x is the same as before
                    if y < fold_at {
                        content.insert((x, y), '#');
                    } else {
                        // When under the fold, y is mirrored from the fold line
                        content.insert((x, 2 * fold_at - y), '#');
                    }
                }
            }
        }

        Paper {
            columns: self.columns,
            rows: (self.rows - 1) / 2,
            content,
        }
    }

    fn fold_at_x(&self, fold_at: u32) -> Paper {
        let mut content = HashMap::new();
        for y in 0..self.rows + 1 {
            for x in 0..self.columns + 1 {
                // For columns left of the fold
                // y is always the same
                if self.content.get(&(x, y)).is_some() {
                    if x < fold_at {
                        content.insert((x, y), '#');
                    } else {
                        content.insert((2 * fold_at - x, y), '#');
                    }
                }
            }
        }

        Paper {
            columns: (self.columns - 1) / 2,
            rows: self.rows,
            content,
        }
    }
}

fn split_input_by_blankline(input: &String) -> Vec<String> {
    input
        .split("\n\n")
        .map(|string| string.to_string())
        .collect()
}

fn get_pairs(input: &str) -> Vec<Pair> {
    input
        .split_whitespace()
        .map(|value| {
            value
                .split(',')
                .map(|number| number.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn get_folding_directions(input: &str) -> Vec<(&str, u32)> {
    let mut output: Vec<(&str, u32)> = vec![];
    let pattern = Regex::new(r"(x|y)=\d*").unwrap();
    for cap in pattern.captures_iter(input) {
        let capture = cap.get(0).unwrap().as_str();
        let split: Vec<&str> = capture.split('=').collect();
        output.push((split[0], split[1].parse::<u32>().unwrap()))
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

    #[test]
    fn it_reads_coordinate_pairs_from_input() {
        let inputs = split_input_by_blankline(&INPUT.to_string());
        let pairs = get_pairs(&inputs[0]);
        assert_eq!(pairs.len(), 18);
    }

    #[test]
    fn it_reads_folding_directions_from_input() {
        let inputs = split_input_by_blankline(&INPUT.to_string());
        let instructions = get_folding_directions(&inputs[1]);
        assert_eq!(instructions.len(), 2);
        assert_eq!(instructions[0], ("y", 7));
        assert_eq!(instructions[1], ("x", 5));
    }

    #[test]
    fn it_builds_the_gridded_map() {
        let inputs = split_input_by_blankline(&INPUT.to_string());
        let pairs = get_pairs(&inputs[0]);
        Paper::new(&pairs);
        assert!(true);
    }

    #[test]
    fn it_prints_the_paper() {
        let inputs = split_input_by_blankline(&INPUT.to_string());
        let pairs = get_pairs(&inputs[0]);
        let paper = Paper::new(&pairs);
        paper.print();
        assert!(true);
    }

    #[test]
    fn it_folds_the_paper_at_y() {
        let inputs = split_input_by_blankline(&INPUT.to_string());
        let pairs = get_pairs(&inputs[0]);
        let paper = Paper::new(&pairs);
        let paper = paper.fold_at_y(7);
        paper.print()
    }

    #[test]
    fn it_folds_the_paper_at_x() {
        let inputs = split_input_by_blankline(&INPUT.to_string());
        let pairs = get_pairs(&inputs[0]);
        let paper = Paper::new(&pairs);
        let paper = paper.fold_at_y(7);
        let paper = paper.fold_at_x(5);
        paper.print();
    }

    #[test]
    fn it_counts_visible_dots() {
        let inputs = split_input_by_blankline(&INPUT.to_string());
        let pairs = get_pairs(&inputs[0]);
        let paper = Paper::new(&pairs);
        let paper = paper.fold_at_y(7);
        let paper = paper.fold_at_x(5);
        let dots = paper.count_visible_dots();
        assert_eq!(dots, 16);
    }
}
