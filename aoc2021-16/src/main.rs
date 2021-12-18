use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read the input");
    let binary = hexadecimal_to_binary(&input);

    let packet = parse_packet(&binary);
    let version_sum = sum_packet_versions(packet);
    dbg!(version_sum);

    let packet = parse_packet(&binary);
    let output = evaluate_packet(packet);
    dbg!(output);
}

#[derive(Debug)]
struct Packet {
    version: usize,
    type_id: usize,
    length_type_id: Option<usize>,
    raw: String,
    bits: usize,
    content: Option<String>,
    subpackets: Option<Vec<Packet>>,
}

fn parse_packet(input: &str) -> Packet {
    let version = usize::from_str_radix(&input[..3], 2).unwrap(); // bits 0-3
    let type_id = usize::from_str_radix(&input[3..6], 2).unwrap(); // bits 3-6

    if type_id == 4 {
        // A literal value
        let mut content = "".to_string();
        // bits_read starts 6 to account for the version and type id headers
        let mut bits_read = 6;
        loop {
            // Get the done bit
            let done = input
                .chars()
                .nth(bits_read)
                .expect("Couldn't get the first bit of a literal chunk.")
                == '0';
            bits_read += 1;
            let value = &input[bits_read..bits_read + 4];
            bits_read += 4;
            content += value;
            if done {
                break;
            }
        }

        Packet {
            version,
            type_id,
            bits: bits_read,
            raw: input[..bits_read].to_string(),
            content: Some(content),
            subpackets: None,
            length_type_id: None,
        }
    } else {
        // This is an operator packet
        let length_type_id = if input.chars().nth(6).unwrap() == '1' {
            1
        } else {
            0
        };
        // bits_read starts at 7 to account for the version, type_id, and length_type_id headers
        let mut bits_read = 7;

        // are we looking for subpackets by bits or by count?
        if length_type_id == 0 {
            // the next 15 bits represent the number of bits in the subpackets
            let subpackets_end =
                usize::from_str_radix(&input[bits_read..bits_read + 15], 2).unwrap();
            bits_read += 15;
            let packet_end = bits_read + subpackets_end;

            // Parse all the subpackets
            let mut subpackets = vec![];
            while bits_read < packet_end {
                let subpacket = parse_packet(&input[bits_read..]);
                bits_read += subpacket.bits;
                subpackets.push(subpacket);
            }

            // Return the packet
            Packet {
                version,
                type_id,
                length_type_id: Some(length_type_id),
                content: None,
                subpackets: Some(subpackets),
                raw: input[..packet_end].to_string(),
                bits: packet_end,
            }
        } else {
            // the next 11 bits encode the number of subpackets
            let subpacket_count =
                usize::from_str_radix(&input[bits_read..bits_read + 11], 2).unwrap();
            bits_read += 11;

            // Parse all the subpackets
            let mut subpackets = vec![];
            while subpackets.len() < subpacket_count {
                let subpacket = parse_packet(&input[bits_read..]);
                bits_read += subpacket.bits;
                subpackets.push(subpacket);
            }

            // Return the packet
            Packet {
                version,
                type_id,
                length_type_id: Some(length_type_id),
                content: None,
                subpackets: Some(subpackets),
                raw: input[..bits_read].to_string(),
                bits: bits_read,
            }
        }
    }
}

fn sum_packet_versions(packet: Packet) -> usize {
    let mut output = 0;
    output += packet.version;

    if let Some(subpackets) = packet.subpackets {
        for subpacket in subpackets {
            output += sum_packet_versions(subpacket);
        }
    }

    output
}

fn evaluate_packet(packet: Packet) -> usize {
    let mut output = 0;
    match packet.type_id {
        // addition
        // sum the packet contents
        0 => {
            if let Some(subpackets) = packet.subpackets {
                for subpacket in subpackets {
                    if let Some(content) = subpacket.content {
                        output += binary_to_decimal(&content);
                    } else {
                        output += evaluate_packet(subpacket);
                    }
                }
            }
        }
        // multiplication
        // multiply the packet contents
        1 => {
            // can't multiply anything by 0
            output = 1;
            if let Some(subpackets) = packet.subpackets {
                for subpacket in subpackets {
                    if let Some(content) = subpacket.content {
                        output *= binary_to_decimal(&content);
                    } else {
                        output *= evaluate_packet(subpacket);
                    }
                }
            }
        }
        // minimum
        // get the minimum value of the packet contents
        2 => {
            let mut values = vec![];
            if let Some(subpackets) = packet.subpackets {
                for subpacket in subpackets {
                    if let Some(content) = subpacket.content {
                        values.push(binary_to_decimal(&content));
                    } else {
                        values.push(evaluate_packet(subpacket));
                    }
                }
            }
            output = *values.iter().min().unwrap();
        }
        // maximum
        // get the maximum value of the packet contents
        3 => {
            let mut values = vec![];
            if let Some(subpackets) = packet.subpackets {
                for subpacket in subpackets {
                    if let Some(content) = subpacket.content {
                        values.push(binary_to_decimal(&content));
                    } else {
                        values.push(evaluate_packet(subpacket));
                    }
                }
            }
            output = *values.iter().max().unwrap();
        }
        // literal
        // literal value
        4 => {
            unimplemented!("Evaluating literal values is unimplemented");
        }
        // greater than
        // return 1 if the first packet value is greater than the second
        5 => {
            let mut values = vec![];
            if let Some(subpackets) = packet.subpackets {
                for subpacket in subpackets {
                    if let Some(content) = subpacket.content {
                        values.push(binary_to_decimal(&content));
                    } else {
                        values.push(evaluate_packet(subpacket));
                    }
                }
            }
            let first = values.get(0).expect("The first value couldn't be found");
            let second = values.get(1).expect("The first value couldn't be found");
            output = if first > second { 1 } else { 0 }
        }
        // less than
        // return 1 if the first packet value is less than the second
        6 => {
            let mut values = vec![];
            if let Some(subpackets) = packet.subpackets {
                for subpacket in subpackets {
                    if let Some(content) = subpacket.content {
                        values.push(binary_to_decimal(&content));
                    } else {
                        values.push(evaluate_packet(subpacket));
                    }
                }
            }
            let first = values.get(0).expect("The first value couldn't be found");
            let second = values.get(1).expect("The first value couldn't be found");
            output = if first < second { 1 } else { 0 }
        }
        // equality
        // return 1 if the value of the first subpacket is equal to that of the second
        7 => {
            let mut values = vec![];
            if let Some(subpackets) = packet.subpackets {
                for subpacket in subpackets {
                    if let Some(content) = subpacket.content {
                        values.push(binary_to_decimal(&content));
                    } else {
                        values.push(evaluate_packet(subpacket));
                    }
                }
            }
            let first = values.get(0).expect("The first value couldn't be found");
            let second = values.get(1).expect("The first value couldn't be found");
            output = if first == second { 1 } else { 0 }
        }
        _ => unreachable!(),
    }
    output
}

fn hexadecimal_to_binary(hex: &str) -> String {
    hex.chars().fold("".to_string(), |acc, c| {
        let value = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => unreachable!(),
        };
        acc + value
    })
}

fn binary_to_decimal(binary: &str) -> usize {
    usize::from_str_radix(binary, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_type_id_4_packet() {
        let hex = "D2FE28";
        let binary = hexadecimal_to_binary(hex);
        let packet = parse_packet(&binary);
        assert_eq!(packet.content, Some("011111100101".to_string()));
        assert_eq!(packet.bits, 21);
        assert_eq!(packet.raw, "110100101111111000101".to_string());
    }

    #[test]
    fn it_parses_a_type_id_6_packet_with_length_type_id_0() {
        let hex = "38006F45291200";
        let binary = hexadecimal_to_binary(hex);
        let packet = parse_packet(&binary);
        let subpackets = packet.subpackets.unwrap();
        assert_eq!(packet.version, 1);
        assert_eq!(packet.type_id, 6);
        assert_eq!(packet.length_type_id, Some(0));
        assert_eq!(subpackets[0].content, Some("1010".to_string()));
        assert_eq!(subpackets[1].content, Some("00010100".to_string()));
    }

    #[test]
    fn it_parses_a_type_id_3_packet_with_length_type_id_1() {
        let hex = "EE00D40C823060";
        let binary = hexadecimal_to_binary(hex);
        let packet = parse_packet(&binary);
        let subpackets = packet.subpackets.unwrap();
        assert_eq!(packet.version, 7);
        assert_eq!(packet.type_id, 3);
        assert_eq!(packet.length_type_id, Some(1));
        assert_eq!(subpackets.len(), 3);
        assert_eq!(subpackets[0].content, Some("0001".to_string()));
        assert_eq!(subpackets[1].content, Some("0010".to_string()));
        assert_eq!(subpackets[2].content, Some("0011".to_string()));
    }

    #[test]
    fn it_parses_nested_operator_packets() {
        let hex = "8A004A801A8002F478";
        let binary = hexadecimal_to_binary(hex);
        let packet = parse_packet(&binary);
        let version_sum = sum_packet_versions(packet);
        assert_eq!(version_sum, 16);
    }

    #[test]
    fn it_parses_treed_operator_packets_pt1() {
        let hex = "620080001611562C8802118E34";
        let binary = hexadecimal_to_binary(hex);
        let packet = parse_packet(&binary);
        let version_sum = sum_packet_versions(packet);
        assert_eq!(version_sum, 12);
    }

    #[test]
    fn it_parses_treed_operator_packets_pt2() {
        let hex = "C0015000016115A2E0802F182340";
        let binary = hexadecimal_to_binary(hex);
        let packet = parse_packet(&binary);
        let version_sum = sum_packet_versions(packet);
        assert_eq!(version_sum, 23);
    }

    #[test]
    fn it_parses_nested_operator_packets_with_several_literals() {
        let hex = "A0016C880162017C3686B18A3D4780";
        let binary = hexadecimal_to_binary(hex);
        let packet = parse_packet(&binary);
        let version_sum = sum_packet_versions(packet);
        assert_eq!(version_sum, 31);
    }

    #[test]
    fn it_sums_subpackets() {
        let hex = "C200B40A82";
        let binary = hexadecimal_to_binary(hex);
        let packet = parse_packet(&binary);
        let sum = evaluate_packet(packet);
        assert_eq!(sum, 3);
    }

    #[test]
    fn it_multiplies_subpackets() {
        let hex = "04005AC33890";
        let binary = hexadecimal_to_binary(hex);
        let packet = parse_packet(&binary);
        let product = evaluate_packet(packet);
        assert_eq!(product, 54);
    }

    #[test]
    fn it_finds_the_minimum_value_of_subpackets() {
        let hex = "880086C3E88112";
        let binary = hexadecimal_to_binary(hex);
        let packet = parse_packet(&binary);
        let product = evaluate_packet(packet);
        assert_eq!(product, 7);
    }

    #[test]
    fn it_finds_the_maximum_value_of_subpackets() {
        let hex = "CE00C43D881120";
        let binary = hexadecimal_to_binary(hex);
        let packet = parse_packet(&binary);
        let product = evaluate_packet(packet);
        assert_eq!(product, 9);
    }

    #[test]
    fn it_returns_1_if_the_first_packet_is_less_than_the_second() {
        let hex = "D8005AC2A8F0";
        let binary = hexadecimal_to_binary(hex);
        let packet = parse_packet(&binary);
        let lt = evaluate_packet(packet);
        assert_eq!(lt, 1);
    }

    #[test]
    fn it_returns_0_if_the_first_packet_is_not_greater_than_the_second() {
        let hex = "F600BC2D8F";
        let binary = hexadecimal_to_binary(hex);
        let packet = parse_packet(&binary);
        let gt = evaluate_packet(packet);
        assert_eq!(gt, 0);
    }

    #[test]
    fn it_returns_0_when_the_first_packet_is_not_equal_to_the_second() {
        let hex = "9C005AC2F8F0";
        let binary = hexadecimal_to_binary(hex);
        let packet = parse_packet(&binary);
        let et = evaluate_packet(packet);
        assert_eq!(et, 0);
    }

    #[test]
    fn it_handles_nested_operators() {
        let hex = "9C0141080250320F1802104A08";
        let binary = hexadecimal_to_binary(hex);
        let packet = parse_packet(&binary);
        let et = evaluate_packet(packet);
        assert_eq!(et, 1);
    }
}
