use std::fs;

fn stage_one(input: &[u16]) -> Vec<u16> {
    input
        .windows(2)
        .into_iter()
        .filter_map(|slice| {
            if let [first, second] = slice {
                if second > first {
                    Some(*second)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn stage_zero(input: &[u16]) -> Vec<u16> {
    input
        .windows(3)
        .into_iter()
        .map(|slice| slice.iter().sum::<u16>())
        .collect()
}

fn main() {
    let input: Vec<u16> = fs::read_to_string("input")
        .expect("couldn't read the file")
        .lines()
        .map(|line| line.parse().expect("couldn't parse number"))
        .collect();

    let first = stage_one(&input);
    println!("part 1: there were {} increases", first.len());

    let second = stage_one(&stage_zero(&input));
    println!("part 2: there were {} increases", second.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stage_one_works() {
        let input: Vec<u16> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let output = stage_one(&input);
        assert_eq!(output.len(), 7);
    }

    #[test]
    fn stage_zero_works() {
        let input: Vec<u16> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let output = stage_one(&stage_zero(&input));
        assert_eq!(output.len(), 5);
    }
}
