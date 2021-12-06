use std::collections::HashMap;
use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .expect("couldn't read the file")
        .lines()
        .map(|line| line.parse().expect("couldn't parse line"))
        .collect();

    let overlapping_cells = count_overlapping_cells(&input).unwrap();
    dbg!(overlapping_cells);
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
    } else if i32::abs(x2 - x1) > 0 && i32::abs(y2 - y1) > 0 {
        // Oh yikes, a diagonal line!
        if x1 < x2 && y1 < y2 {
            // 45 degrees
            for (i, _y) in (y1..y2 + 1).enumerate() {
                output.push(vec![x1 + i as i32, y1 + i as i32]);
            }
        } else if x1 > x2 && y1 < y2 {
            // 135 degrees
            for (i, _x) in (x2..x1 + 1).enumerate() {
                output.push(vec![x1 - i as i32, y1 + i as i32]);
            }
        } else if x1 > x2 && y1 > y2 {
            // 225 degrees
            for (i, _y) in (x2..y1 + 1).enumerate() {
                output.push(vec![x1 - i as i32, y1 - i as i32]);
            }
        } else if x1 < x2 && y1 > y2 {
            // 315 degrees
            for (i, _y) in (y2..y1 + 1).enumerate() {
                output.push(vec![x1 + i as i32, y1 - i as i32]);
            }
        }
    }

    output
}

fn count_overlapping_cells(input: &[String]) -> Option<i32> {
    let mut cell_map: HashMap<Vec<i32>, i32> = HashMap::new();

    // Build out all the lines
    let lines: Vec<Vec<Vec<i32>>> = input
        .iter()
        .map(|value| parse_line_pairs(value))
        .map(fill_in_line)
        .collect();

    // Flatten out the lines into a single collection of x,y coordinate pairs
    let cells = lines.concat();
    // Update overlaps with each x,y coordinate pair, noting how many times
    // a coordinate pair appears in cells
    for cell in cells {
        match cell_map.get(&cell) {
            Some(&value) => {
                cell_map.insert(cell, value + 1);
            }
            None => {
                cell_map.insert(cell, 1);
            }
        }
    }

    // Check how many x,y coordinate pairs appeared in cells more than once.
    let overlaps: Vec<i32> = cell_map.into_values().filter(|&value| value >= 2).collect();
    Some(overlaps.len() as i32)
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
    fn it_fills_in_the_45_degree_line() {
        let input = "1,1 -> 3,3".to_string();
        let pair = parse_line_pairs(&input);
        let output = fill_in_line(pair);
        assert_eq!(output.len(), 3);
        assert_eq!(output[0], vec![1, 1]);
        assert_eq!(output[1], vec![2, 2]);
        assert_eq!(output[2], vec![3, 3]);
    }

    #[test]
    fn it_fills_in_the_135_degree_line() {
        let input = "9,7 -> 7,9".to_string();
        let pair = parse_line_pairs(&input);
        let output = fill_in_line(pair);
        assert_eq!(output.len(), 3);
        assert_eq!(output[0], vec![9, 7]);
        assert_eq!(output[1], vec![8, 8]);
        assert_eq!(output[2], vec![7, 9]);
    }

    #[test]
    fn it_fills_in_the_225_degree_line() {
        let input = "0,0 -> -2,-2".to_string();
        let pair = parse_line_pairs(&input);
        let output = fill_in_line(pair);
        assert_eq!(output.len(), 3);
        assert_eq!(output[0], vec![0, 0]);
        assert_eq!(output[1], vec![-1, -1]);
        assert_eq!(output[2], vec![-2, -2]);
    }

    #[test]
    fn it_fills_in_the_315_degree_line() {
        let input = "0,0 -> 2,-2".to_string();
        let pair = parse_line_pairs(&input);
        let output = fill_in_line(pair);
        assert_eq!(output.len(), 3);
        assert_eq!(output[0], vec![0, 0]);
        assert_eq!(output[1], vec![1, -1]);
        assert_eq!(output[2], vec![2, -2]);
    }

    #[test]
    fn it_counts_overlapping_cells() {
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

        let output = count_overlapping_cells(&input);
        assert_eq!(output, Some(12));
    }
}
