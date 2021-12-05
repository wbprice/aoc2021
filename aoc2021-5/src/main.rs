fn main() {
    println!("Hello, world!");
}

fn parse_line_pairs(input: &str) -> Vec<Vec<i32>> {
    let mut output = vec![];
    let pairs: Vec<&str> = input.split(" -> ").collect();
    for pair in pairs {
        let split: Vec<i32> = pair
            .split(',')
            .map(|value| value.parse().unwrap())
            .collect();
        output.push(split)
    }
    output
}

fn fill_in_line(line: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut output = vec![];
    let x1 = line[0][0];
    let y1 = line[0][1];
    let x2 = line[1][0];
    let y2 = line[1][1];

    // Is this a vertical or a horizontal line?
    if i32::abs(y2 - y1) == 0 {
        // horizontal
        if x1 < x2 {
            for x in x1..x2 + 1 {
                output.push(vec![x, y1]);
            }
        } else {
            for x in x2..x1 + 1 {
                output.push(vec![x, y1]);
            }
        }
    } else if i32::abs(x2 - x1) == 0 {
        // vertical
        if y1 < y2 {
            for y in y1..y2 + 1 {
                output.push(vec![x1, y]);
            }
        } else {
            for y in y2..y1 + 1 {
                output.push(vec![x1, y]);
            }
        }
    }

    output
}

fn part_one(input: &Vec<String>) -> Option<i32> {
    dbg!(input);

    let lines: Vec<Vec<Vec<i32>>> = input
        .iter()
        .map(|value| parse_line_pairs(value))
        .map(|value| fill_in_line(value))
        .collect();

    dbg!(lines);

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_line_pairs() {
        let input: Vec<String> = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#
        .lines()
        .map(|line| line.parse().expect("couldn't parse line"))
        .collect();

        let output = parse_line_pairs(&input[0]);
        assert_eq!(output[0][0], 0);
        assert_eq!(output[0][1], 9);
        assert_eq!(output[1][0], 5);
        assert_eq!(output[1][1], 9);
    }

    #[test]
    fn it_fills_in_the_line() {
        let input = "0,9 -> 5,9".to_string();
        let pair = parse_line_pairs(&input);
        let output = fill_in_line(pair);
        assert_eq!(output.len(), 6);
        assert_eq!(output[0], vec![0, 9]);
        assert_eq!(output[1], vec![1, 9]);
        assert_eq!(output[2], vec![2, 9]);
        assert_eq!(output[3], vec![3, 9]);
        assert_eq!(output[4], vec![4, 9]);
        assert_eq!(output[5], vec![5, 9]);
    }

    #[test]
    fn it_fills_in_the_line_backwards() {
        let input = "9,4 -> 3,4".to_string();
        let pair = parse_line_pairs(&input);
        let output = fill_in_line(pair);
        assert_eq!(output.len(), 7);
        assert_eq!(output[0], vec![3, 4]);
        assert_eq!(output[1], vec![4, 4]);
        assert_eq!(output[2], vec![5, 4]);
        assert_eq!(output[3], vec![6, 4]);
        assert_eq!(output[4], vec![7, 4]);
        assert_eq!(output[5], vec![8, 4]);
        assert_eq!(output[6], vec![9, 4]);
    }

    #[test]
    fn it_counts_overlapping_line_cells() {
        let input: Vec<String> = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#
        .lines()
        .map(|line| line.parse().expect("couldn't parse line"))
        .collect();

        let output = part_one(&input);
        assert_eq!(output, Some(5));
    }
}
