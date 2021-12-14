use std::collections::HashMap;

type Pair = Vec<u8>;
type Fold = String;
type Paper = HashMap<(u8, u8), char>;

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

fn build_paper(pairs: &[Vec<u8>]) -> Paper {
    pairs.iter().fold(HashMap::new(), |mut acc, pair| {
        acc.insert((pair[0], pair[1]), '#');
        acc
    })
}

fn print_paper(paper: Paper) {
    let mut print: Vec<String> = vec![];
    let mut x = 0;
    let mut y = 0;
    paper.clone().into_keys().for_each(|pair| {
        if pair.0 > x {
            x = pair.0;
        }
        if pair.1 > y {
            y = pair.1;
        }
    });

    for y in 0..y + 1 {
        let mut row = vec![];
        for x in 0..x + 1 {
            match paper.get(&(x, y)) {
                Some(_) => {
                    row.push('#');
                }
                None => row.push('.'),
            }
        }
        let text: String = row
            .iter()
            .fold("".to_string(), |acc, x| acc + &x.to_string());
        print.push(text);
    }
    dbg!(print);
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
        let output = build_paper(&pairs);
        dbg!(&output);
    }

    #[test]
    fn it_prints_the_paper() {
        let inputs = split_input_by_blankline(INPUT);
        let pairs = get_pairs(&inputs[0]);
        let paper = build_paper(&pairs);
        print_paper(paper);
    }
}
