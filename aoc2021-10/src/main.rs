use std::fs;

fn main() {
    let subsystem: Vec<String> = fs::read_to_string("input")
        .expect("Couldn't read the input")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let error_score = score_subsystem(&subsystem);
    dbg!(error_score);

    let mut completion_scores = score_completions(&subsystem);
    completion_scores.sort_unstable();
    let middle = (completion_scores.len() as f32 / 2.0).floor() as usize;
    dbg!(completion_scores[middle]);
}

fn score_subsystem(input: &[String]) -> u64 {
    input.iter().map(|line| score_line(line)).sum::<u64>()
}

fn score_completions(input: &[String]) -> Vec<u64> {
    let mut output: Vec<u64> = vec![];

    for line in input {
        // Drop invalid lines
        if score_line(line) != 0 {
            continue;
        }

        if let Some(suggestions) = suggest_completions(line) {
            let suggestion_score = score_completion_suggestions(&suggestions);
            output.push(suggestion_score);
        }
    }

    output
}

fn score_line(line: &str) -> u64 {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            ')' => {
                if let Some('(') = stack.last() {
                    stack.pop();
                } else {
                    return 3;
                }
            }
            ']' => {
                if let Some('[') = stack.last() {
                    stack.pop();
                } else {
                    return 57;
                }
            }
            '}' => {
                if let Some('{') = stack.last() {
                    stack.pop();
                } else {
                    return 1197;
                }
            }
            '>' => {
                if let Some('<') = stack.last() {
                    stack.pop();
                } else {
                    return 25137;
                }
            }
            _ => stack.push(c),
        }
    }
    0
}

fn suggest_completions(line: &str) -> Option<String> {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            ')' => {
                if let Some('(') = stack.last() {
                    stack.pop();
                }
            }
            ']' => {
                if let Some('[') = stack.last() {
                    stack.pop();
                }
            }
            '}' => {
                if let Some('{') = stack.last() {
                    stack.pop();
                }
            }
            '>' => {
                if let Some('<') = stack.last() {
                    stack.pop();
                }
            }
            _ => stack.push(c),
        }
    }

    // Reverse the stack order, find matching symbols to recommend completions
    stack.reverse();
    let output: String = stack
        .iter()
        .map(|c| match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => unimplemented!("Symbol not handled"),
        })
        .fold("".to_string(), |acc, x| acc + &x.to_string());

    Some(output)
}

fn score_completion_suggestions(input: &str) -> u64 {
    input
        .chars()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unimplemented!("Symbol not handled"),
        })
        .fold(0, |acc, x| acc * 5 + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUBSYSTEM: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    #[test]
    fn it_scores_a_valid_chunk_zero() {
        let chunk = "([])".to_string();
        let output = score_line(&chunk);
        assert_eq!(output, 0);

        let chunk = "{()()()}".to_string();
        let output = score_line(&chunk);
        assert_eq!(output, 0);

        let chunk = "<([{}])>".to_string();
        let output = score_line(&chunk);
        assert_eq!(output, 0);

        let chunk = "[<>({}){}[([])<>]]".to_string();
        let output = score_line(&chunk);
        assert_eq!(output, 0);

        let chunk = "(((((((((())))))))))".to_string();
        let output = score_line(&chunk);
        assert_eq!(output, 0);
    }

    #[test]
    fn it_scores_invalid_lines() {
        let chunk = "{([(<{}[<>[]}>{[]{[(<()>".to_string();
        let output = score_line(&chunk);
        assert_eq!(output, 1197);

        let chunk = "[[<[([]))<([[{}[[()]]]".to_string();
        let output = score_line(&chunk);
        assert_eq!(output, 3);

        let chunk = "[{[{({}]{}}([{[{{{}}([]".to_string();
        let output = score_line(&chunk);
        assert_eq!(output, 57);

        let chunk = "[<(<(<(<{}))><([]([]()".to_string();
        let output = score_line(&chunk);
        assert_eq!(output, 3);

        let chunk = "<{([([[(<>()){}]>(<<{{".to_string();
        let output = score_line(&chunk);
        assert_eq!(output, 25137);
    }

    #[test]
    fn it_scores_subsystems() {
        let input: Vec<String> = SUBSYSTEM.lines().map(|line| line.to_string()).collect();
        let output = score_subsystem(&input);
        assert_eq!(output, 26397);
    }

    #[test]
    fn it_suggests_completions() {
        let input = "[({(<(())[]>[[{[]{<()<>>".to_string();
        let output = suggest_completions(&input);
        assert_eq!(output, Some("}}]])})]".to_string()));

        let input = "[(()[<>])]({[<{<<[]>>(".to_string();
        let output = suggest_completions(&input);
        assert_eq!(output, Some(")}>]})".to_string()));

        let input = "(((({<>}<{<{<>}{[]{[]{}".to_string();
        let output = suggest_completions(&input);
        assert_eq!(output, Some("}}>}>))))".to_string()));

        let input = "{<[[]]>}<{[{[{[]{()[[[]".to_string();
        let output = suggest_completions(&input);
        assert_eq!(output, Some("]]}}]}]}>".to_string()));

        let input = "<{([{{}}[<[[[<>{}]]]>[]]".to_string();
        let output = suggest_completions(&input);
        assert_eq!(output, Some("])}>".to_string()));
    }

    #[test]
    fn it_scores_completions() {
        let input = "}}]])})]".to_string();
        let output = score_completion_suggestions(&input);
        assert_eq!(output, 288957);

        let input = ")}>]})".to_string();
        let output = score_completion_suggestions(&input);
        assert_eq!(output, 5566);

        let input = "}}>}>))))".to_string();
        let output = score_completion_suggestions(&input);
        assert_eq!(output, 1480781);

        let input = "]]}}]}]}>".to_string();
        let output = score_completion_suggestions(&input);
        assert_eq!(output, 995444);

        let input = "])}>".to_string();
        let output = score_completion_suggestions(&input);
        assert_eq!(output, 294);
    }

    #[test]
    fn it_scores_all_the_completions() {
        let input: Vec<String> = SUBSYSTEM
            .to_string()
            .lines()
            .map(|value| value.to_string())
            .collect();

        let mut scored_suggestions = score_completions(&input);
        scored_suggestions.sort_unstable();
        let middle = (scored_suggestions.len() as f32 / 2.0).floor() as usize;
        assert_eq!(scored_suggestions[middle], 288957);
    }
}
