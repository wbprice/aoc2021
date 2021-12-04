use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .expect("couldn't read the file")
        .lines()
        .map(|line| line.parse().expect("couldn't parse line"))
        .collect();

    let aggregate = aggregate_readings(&input, 12);
    let gamma_rate = get_gamma_rate(&aggregate);
    let episilon_rate = get_epsilon_rate(&aggregate);
    let decimal_gamma = binary_to_decimal(gamma_rate);
    let decimal_epsilon = binary_to_decimal(episilon_rate);

    let o2_generator_rating = get_oxygen_generator_rating(&input, 12);
    let co2_dioxide_scrubber_rating = get_carbon_dioxide_scrubber_rating(&input, 12);
    let decimal_o2 = binary_to_decimal(o2_generator_rating);
    let decimal_co2 = binary_to_decimal(co2_dioxide_scrubber_rating);

    dbg!(decimal_gamma * decimal_epsilon);
    dbg!(decimal_o2 * decimal_co2);
}

fn aggregate_readings(input: &[String], size: usize) -> Vec<(i64, i64)> {
    let mut output = vec![(0, 0); size];
    for reading in input {
        for (i, c) in reading.chars().enumerate() {
            if c == '1' {
                output[i].1 += 1;
            } else {
                output[i].0 += 1;
            }
        }
    }
    output
}

fn get_gamma_rate(aggregate: &[(i64, i64)]) -> String {
    let mut output = "".to_string();
    for (zeroes, ones) in aggregate {
        if zeroes > ones {
            output.push('0');
        } else {
            output.push('1');
        }
    }
    output
}

fn get_epsilon_rate(aggregate: &[(i64, i64)]) -> String {
    let mut output = "".to_string();
    for (zeroes, ones) in aggregate {
        if ones < zeroes {
            output.push('1');
        } else {
            output.push('0');
        }
    }
    output
}

fn binary_to_decimal(input: String) -> isize {
    let str = input.as_str();
    isize::from_str_radix(str, 2).expect("couldn't parse binary")
}

fn filter_reading(input: &[String], mask: char, index: usize) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    for reading in input {
        if let Some(r) = reading.chars().nth(index) {
            if r == mask {
                output.push(reading.clone());
            }
        }
    }
    output
}

fn get_oxygen_generator_rating(input: &[String], size: usize) -> String {
    let mut output = vec!["".to_string(); input.len()];
    output.clone_from_slice(input);

    // Get initial gamma_rate
    let aggregate = aggregate_readings(&output, size);
    let mut gamma_rate = get_gamma_rate(&aggregate);

    for index in 0..gamma_rate.len() {
        // If there's only one output remaining, exit early
        if output.len() == 1 {
            break;
        }

        // Otherwise, filter the remaining inputs by the given mask in gamma rate
        if let Some(mask) = gamma_rate.chars().nth(index) {
            output = filter_reading(&output, mask, index);
            // Recalculate gamma_rate
            let aggregate = aggregate_readings(&output, size);
            gamma_rate = get_gamma_rate(&aggregate);
        }
    }

    output[0].clone()
}

fn get_carbon_dioxide_scrubber_rating(input: &[String], size: usize) -> String {
    let mut output = vec!["".to_string(); input.len()];
    output.clone_from_slice(input);

    // Get initial epsilon_rate
    let aggregate = aggregate_readings(&output, size);
    let mut epsilon_rate = get_epsilon_rate(&aggregate);

    for index in 0..epsilon_rate.len() {
        // If there's only one output remaining, exit early
        if output.len() == 1 {
            break;
        }

        // Otherwise, filter the remaining inputs by the given mask in epsilon rate
        if let Some(mask) = epsilon_rate.chars().nth(index) {
            output = filter_reading(&output, mask, index);
            // Recalculate epsilon rate
            let aggregate = aggregate_readings(&output, size);
            epsilon_rate = get_epsilon_rate(&aggregate);
        }
    }

    output[0].clone()
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

        let aggregate = aggregate_readings(&input, 5);
        let gamma_rate = get_gamma_rate(&aggregate);
        assert_eq!(gamma_rate, "10110");
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

        let aggregate = aggregate_readings(&input, 5);
        let epsilon_rate = get_epsilon_rate(&aggregate);
        assert_eq!(epsilon_rate, "01001");
    }

    #[test]
    fn it_converts_binary_to_decimal() {
        let input = "10110".to_string();
        assert_eq!(binary_to_decimal(input), 22);
    }

    #[test]
    fn it_gets_the_oxygen_generator_rating() {
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

        let output = get_oxygen_generator_rating(&input, 5);
        assert_eq!(output, "10111".to_string());
        assert_eq!(binary_to_decimal(output), 23);
    }

    #[test]
    fn it_gets_the_carbon_dioxide_scrubber_rating() {
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

        let output = get_carbon_dioxide_scrubber_rating(&input, 5);
        assert_eq!(output, "01010".to_string());
        assert_eq!(binary_to_decimal(output), 10);
    }
}
