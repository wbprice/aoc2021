use std::collections::HashMap;

type Pair = Vec<u8>;
type Fold = String;

#[derive(Debug)]
struct Paper {
    columns: u8,
    rows: u8,
    content: HashMap<(u8, u8), char>,
}

impl Paper {
    fn new(pairs: &[Vec<u8>]) -> Self {
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
    }

    fn fold_horiontally(&self, fold_at: u8) -> Paper {
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
                        content.insert((x, self.rows - y), '#');
                    }
                }
            }
        }

        Paper {
            columns: self.columns / 2,
            rows: self.rows,
            content,
        }
    }

    fn fold_vertically(&self, fold_at: u8) -> Paper {
        let mut content = HashMap::new();
        for y in 0..self.rows + 1 {
            for x in 0..self.columns + 1 {
                // For columns left of the fold
                // y is always the same
                if self.content.get(&(x, y)).is_some() {
                    if x < fold_at {
                        content.insert((x, y), '#');
                    } else {
                        content.insert((self.columns - x, y), '#');
                    }
                }
            }
        }
        
        Paper {
            columns: self.columns,
            rows: self.rows / 2,
            content,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

fn split_input_by_blankline(input: &str) -> Vec<String> {
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
                .map(|number| number.parse::<u8>().unwrap())
                .collect()
        })
        .collect()
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
        let inputs = split_input_by_blankline(INPUT);
        let pairs = get_pairs(&inputs[0]);
        assert_eq!(pairs.len(), 18);
    }

    #[test]
    fn it_builds_the_gridded_map() {
        let inputs = split_input_by_blankline(INPUT);
        let pairs = get_pairs(&inputs[0]);
        let paper = Paper::new(&pairs);
        dbg!(&paper);
    }

    #[test]
    fn it_prints_the_paper() {
        let inputs = split_input_by_blankline(INPUT);
        let pairs = get_pairs(&inputs[0]);
        let paper = Paper::new(&pairs);
        paper.print();
    }

    #[test]
    fn it_folds_the_paper_vertically() {
        let inputs = split_input_by_blankline(INPUT);
        let pairs = get_pairs(&inputs[0]);
        let paper = Paper::new(&pairs);
        let paper = paper.fold_vertically(7);
        paper.print()
    }

    #[test]
    fn it_folds_the_paper_horizontally() {
        let inputs = split_input_by_blankline(INPUT);
        let pairs = get_pairs(&inputs[0]);
        let paper = Paper::new(&pairs);
        let paper = paper.fold_vertically(7);
        let paper = paper.fold_horiontally(5);
        paper.print();
    }
}
