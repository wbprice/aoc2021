use std::fs;
use std::vec;

fn main() {
    let input: String = fs::read_to_string("input").expect("couldn't read the file");

    let part_one = part_one(&input);
    dbg!(part_one);

    let part_two = part_two(&input);
    dbg!(part_two);
}

#[derive(Debug)]
struct Board {
    rows: usize,
    columns: usize,
    raw: Vec<i64>,
}

impl Board {
    fn new(rows: usize, columns: usize, raw: &String) -> Self {
        Board {
            rows,
            columns,
            raw: raw
                .split_whitespace()
                .map(|value| value.parse::<i64>().unwrap())
                .collect(),
        }
    }

    fn as_rows(&self) -> Vec<&[i64]> {
        self.raw.chunks(self.rows).collect()
    }

    fn as_columns(&self) -> Vec<Vec<i64>> {
        let mut output: Vec<Vec<i64>> = vec![];
        // Create n number of column vectors
        for _ in 0..self.columns {
            output.push(vec![])
        }

        // Populate each column vector
        for (index, value) in self.raw.clone().iter().enumerate() {
            let col_index = index % self.columns;
            output[col_index].push(*value);
        }

        output
    }

    fn is_winner(&self, moves: &[i64]) -> bool {
        // Check for horizontal wins
        for row in self.as_rows() {
            let row = row.to_vec();
            let marked_cells: Vec<i64> = row
                .iter()
                .filter_map(|cell| {
                    if moves.contains(cell) {
                        Some(*cell)
                    } else {
                        None
                    }
                })
                .collect();

            if marked_cells.len() == self.columns {
                return true;
            }
        }

        // Check for vertical wins
        for column in self.as_columns() {
            let column = column.to_vec();
            let marked_cells: Vec<i64> = column
                .iter()
                .filter_map(|cell| {
                    if moves.contains(cell) {
                        Some(*cell)
                    } else {
                        None
                    }
                })
                .collect();
            if marked_cells.len() == self.rows {
                return true;
            }
        }

        false
    }

    fn get_score(&self, moves: &[i64]) -> i64 {
        self.raw
            .clone()
            .iter()
            .filter_map(|value| {
                if !moves.contains(value) {
                    Some(*value)
                } else {
                    None
                }
            })
            .sum()
    }
}

fn split_input_by_blankline(input: &String) -> Vec<String> {
    input
        .split("\n\n")
        .map(|string| string.to_string())
        .collect()
}

fn get_moves(input: &String) -> Vec<i64> {
    input
        .split(",")
        .map(|value| value.parse::<i64>().unwrap())
        .collect()
}

fn part_one(input: &String) -> i64 {
    let inputs = split_input_by_blankline(input);
    // Get the moves
    let moves = get_moves(&inputs[0]);

    // Build the boards
    let mut boards: Vec<Board> = vec![];
    for board_string in &inputs[1..] {
        boards.push(Board::new(5, 5, board_string));
    }

    for index in 0..moves.len() {
        for board in &boards {
            if board.is_winner(&moves[0..index]) {
                let score = board.get_score(&moves[0..index]);
                let last_number = &moves[0..index].last().unwrap();
                return score * *last_number;
            }
        }
    }
    -1
}

fn part_two(input: &String) -> i64 {
    let inputs = split_input_by_blankline(input);
    // Get the moves
    let moves = get_moves(&inputs[0]);

    let mut boards: Vec<Board> = vec![];
    for board_string in &inputs[1..] {
        boards.push(Board::new(5, 5, board_string));
    }

    for index in 0..moves.len() {
        let mut to_remove: Vec<usize> = vec![];
        for (board_index, board) in boards.iter().enumerate() {
            // Otherwise, check if Increment winners
            if board.is_winner(&moves[0..index]) {
                if boards.len() == 1 {
                    let score = board.get_score(&moves[0..index]);
                    let last_number = &moves[0..index].last().unwrap();
                    return score * *last_number;
                }

                to_remove.push(board_index);
            }
        }
        for index in to_remove {
            boards.remove(index);
        }
    }
    -1
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_splits_the_input_into_moves_and_board_strings() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7"#
            .to_string();
        let input_strings = split_input_by_blankline(&input);
        assert_eq!(input_strings.len(), 4);
    }

    #[test]
    fn it_finds_moves() {
        let input =
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string();
        let output = get_moves(&input);
        assert_eq!(output.len(), 27);
    }

    #[test]
    fn it_creates_a_board() {
        let input = r#"
22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19
"#
        .to_string();
        let board = Board::new(5, 5, &input);
        assert_eq!(board.rows, 5);
        assert_eq!(board.columns, 5);
        assert_eq!(board.raw.len(), 25);
    }

    #[test]
    fn it_returns_a_board_as_rows() {
        let input = r#"
22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19
"#
        .to_string();
        let board = Board::new(5, 5, &input);
        let rows = board.as_rows();
        assert_eq!(rows[0], [22, 13, 17, 11, 0]);
        assert_eq!(rows[1], [8, 2, 23, 4, 24]);
        assert_eq!(rows[2], [21, 9, 14, 16, 7]);
        assert_eq!(rows[3], [6, 10, 3, 18, 5]);
        assert_eq!(rows[4], [1, 12, 20, 15, 19]);
        assert_eq!(rows.len(), 5);
    }

    #[test]
    fn it_returns_a_board_as_columns() {
        let input = r#"
22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19
"#
        .to_string();
        let board = Board::new(5, 5, &input);
        let columns = board.as_columns();
        assert_eq!(columns[0], [22, 8, 21, 6, 1]);
        assert_eq!(columns[1], [13, 2, 9, 10, 12]);
        assert_eq!(columns[2], [17, 23, 14, 3, 20]);
        assert_eq!(columns[3], [11, 4, 16, 18, 15]);
        assert_eq!(columns[4], [0, 24, 7, 5, 19]);
        assert_eq!(columns.len(), 5);
    }

    #[test]
    fn it_checks_if_the_board_is_a_winner() {
        let moves_string = "7,4,9,5,11,17,23,2,0,14,21,24".to_string();
        let board_string = r#"
14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#
        .to_string();

        let moves = get_moves(&moves_string);
        let board = Board::new(5, 5, &board_string);
        assert_eq!(board.is_winner(&moves), true);
    }

    #[test]
    fn it_gets_a_boards_score() {
        let moves_string = "7,4,9,5,11,17,23,2,0,14,21,24".to_string();
        let board_string = r#"
14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#
        .to_string();

        let moves = get_moves(&moves_string);
        let board = Board::new(5, 5, &board_string);
        assert_eq!(board.is_winner(&moves), true);
        assert_eq!(board.get_score(&moves), 188);
    }

    #[test]
    fn it_finds_the_first_winning_board() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7"#
            .to_string();
        let output = part_one(&input);
        assert_eq!(output, 4512);
    }

    #[test]
    fn it_finds_the_last_winning_board() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7"#
            .to_string();
        let output = part_two(&input);
        assert_eq!(output, 1924);
    }
}
