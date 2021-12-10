use std::ops::Index;


fn main() {
    println!("Hello, world!");
}

fn chunk_parser(input: &String) {
    let tokens: Vec<char> = input.chars().collect();
    for (index, c) in tokens.iter().enumerate() {
        let rest = &tokens[0..];
        match c {
            '(' => {
                // look ahead for a ')'
                let close_paren = rest.iter().position(|&c| c == ')').unwrap();
                let insides = &tokens[index + 1..close_paren];
                dbg!(insides);
            },
            '[' => {
                // look ahead for a ']'

            }
            _ => {
                unimplemented!("The chunk parser doesn't handle this character");
            }
        }
    }
}

fn is_line_valid(input: &String) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_chunk() {
        let chunk = "([])".to_string();
        let output = chunk_parser(&chunk);
        dbg!(output);
    }

    #[test]
    fn it_validates_a_line() {
        let valid_line = "[({(<(())[]>[[{[]{<()<>>".to_string();
        let output = is_line_valid(&valid_line);
        assert!(output);

        let invalid_line = "{([(<{}[<>[]}>{[]{[(<()>".to_string();
        let output = is_line_valid(&valid_line);
        assert!(output);
    }
}
