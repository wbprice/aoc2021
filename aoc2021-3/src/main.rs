use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .expect("couldn't read the file")
        .lines()
        .map(|line| line.parse().expect("couldn't parse line"))
        .collect();

    let gamma_rate = get_gamma_rate(&input, 12);
    let episilon_rate = get_epsilion_rate(gamma_rate.clone());
    let decimal_gamma = binary_to_decimal(gamma_rate);
    let decimal_epsilon = binary_to_decimal(episilon_rate);

    dbg!(decimal_gamma * decimal_epsilon);
}

fn aggregate_readings(input: &[String], size: usize) -> Vec<i64> {
    let mut output = vec![0; size];
    for reading in input {
        for (i, c) in reading.chars().enumerate() {
            if c == '1' {
                output[i] += 1;
            }
        }
    }
    output
}

fn binary_to_decimal(input: String) -> isize {
    let str = input.as_str();
    isize::from_str_radix(str, 2).expect("couldn't parse binary")
}

fn get_gamma_rate(input: &[String], size: usize) -> String {
    let aggregates = aggregate_readings(input, size);
    let threshold = input.len() / 2;
    let mut output = "".to_string();
    for value in aggregates {
        if value > threshold as i64 {
            output.push('1');
        } else {
            output.push('0');
        }
    }
    output
}

fn get_epsilion_rate(gamma_rate: String) -> String {
    let mut output = "".to_string();
    for c in gamma_rate.chars() {
        match c {
            '1' => output.push('0'),
            '0' => output.push('1'),
            _ => {}
        }
    }
    output
}

fn get_oxygen_rate(input: &[String], size: usize) {
    let gamma_rate = get_gamma_rate(&input, size);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_finds_the_gamma_rate() {
        let input = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];
        let gamma_rate = get_gamma_rate(&input, 5);
        assert_eq!(gamma_rate, "10110");

        dbg!(isize::from_str_radix(&gamma_rate, 2).unwrap());
    }

    #[test]
    fn it_finds_the_epsilion_rate() {
        let input = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        let gamma_rate = get_gamma_rate(&input, 5);
        let epsilon_rate = get_epsilion_rate(gamma_rate);
        assert_eq!(epsilon_rate, "01001");
    }

    #[test]
    fn it_converts_binary_to_decimal() {
        let input = "10110".to_string();
        assert_eq!(binary_to_decimal(input), 22);
    }

    #[test]
    fn it_finds_the_oxygen_rating() {
        let input = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        let oxygen_rating = get_oxygen_rate(&input, 5);
    }
}
