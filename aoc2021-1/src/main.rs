use std::fs;

fn stage_one(input: &Vec<u16>) -> Vec<u16> {
    return input
        .iter()
        .enumerate()
        .filter_map(|(index, value)| {
            if index > 0 {
                let last_value = &input[index - 1];
                if value > last_value {
                    Some(*value)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
}

fn stage_zero(input: &Vec<u16>) -> Vec<u16> {
    return input
        .iter()
        .enumerate()
        .filter_map(|(index, _value)| {
            if index > 1 {
                // check the last three, starting with the third item
                let slice = &input[index - 2..index + 1];
                let sum: u16 = slice.into_iter().sum::<u16>();
                Some(sum)
            } else {
                None
            }
        })
        .collect();
}

fn main() {
    let _preview_input: Vec<u16> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
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
