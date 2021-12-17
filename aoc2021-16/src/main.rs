fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Packet {
    version: usize,
    type_id: usize,
    contents: Option<String>,
    length: usize,
    subpackets: Option<Vec<Packet>>,
}

fn parse_packet_version(packet: &str) -> usize {
    usize::from_str_radix(&packet[0..3], 2).unwrap()
}

// Type 3 - Operator
// Type 4 - Literal
// Type 6 - Operator
fn parse_packet_type_id(packet: &str) -> usize {
    usize::from_str_radix(&packet[3..6], 2).unwrap()
}

fn parse_length_type_id(packet: &str) -> usize {
    match &packet.chars().nth(6) {
        Some('1') => 1,
        Some('0') => 0,
        _ => {
            unimplemented!("Can't parse a length type of this type");
        }
    }
}

fn parse_type_id_4_packet(packet: &str) -> Packet {
    let data = &packet[6..];
    let mut content = "".to_string();
    let mut index = 0;
    loop {
        content += &data[index + 1..index + 5];
        index += 5;
        if data.chars().nth(index - 5) == Some('0') {
            break;
        }
    }
    // Return the parsed value and the length of the packet
    // (output, index + 6)
    Packet {
        version: parse_packet_version(packet),
        type_id: parse_packet_type_id(packet),
        contents: Some(content),
        length: index + 6,
        subpackets: None,
    }
}

fn parse_operator_packet(packet: &str) -> Packet {
    let mut subpackets: Vec<Packet> = vec![];
    let length_type = parse_length_type_id(&packet);

    match length_type {
        0 => {
            let subpackets_bytes = usize::from_str_radix(&packet[7..22], 2).unwrap();
            let subpackets_slice = &packet[22..subpackets_bytes + 22];
            let mut bytes_read = 0;

            // for packet in subpackets
            while bytes_read < subpackets_bytes {
                // what kind of packet is this?
                let _packet_version = parse_packet_version(&subpackets_slice[bytes_read..]);
                let packet_type_id = parse_packet_type_id(&subpackets_slice[bytes_read..]);

                match packet_type_id {
                    4 => {
                        let packet = parse_type_id_4_packet(&subpackets_slice[bytes_read..]);
                        bytes_read += packet.length;
                        subpackets.push(packet);
                    }
                    _ => {
                        // TODO: is every type other than 4 an operator?
                        let packet = parse_operator_packet(&subpackets_slice[bytes_read..]);
                        bytes_read += packet.length;
                        subpackets.push(packet);
                    }
                }

                // Break when all the packets have been read
                if bytes_read >= subpackets_bytes {
                    break;
                }
            }
        }
        1 => {
            let subpacket_count = usize::from_str_radix(&packet[7..18], 2).unwrap();
            let subpackets_slice = &packet[18..];
            let mut packets_read = 0;
            let mut bytes_read = 0;

            while packets_read < subpacket_count {
                // what kind of packet is this?
                let packet_version = parse_packet_version(&subpackets_slice[bytes_read..]);
                let packet_type_id = parse_packet_type_id(&subpackets_slice[bytes_read..]);

                match packet_type_id {
                    4 => {
                        let packet = parse_type_id_4_packet(&subpackets_slice[bytes_read..]);
                        bytes_read += packet.length;
                        packets_read += 1;
                        subpackets.push(packet);
                    }
                    _ => {
                        // TODO: is every type other than 4 an operator?
                        let packet = parse_operator_packet(&subpackets_slice[bytes_read..]);
                        bytes_read += packet.length;
                        packets_read += 1;
                        subpackets.push(packet);
                    }
                }

                // Exit when all the packets have been read
                if packets_read >= subpacket_count {
                    break;
                }
            }
        }
        _ => unimplemented!("Length type IDs other than 0 and 1 not implemented"),
    }

    Packet {
        length: packet.len(),
        version: parse_packet_version(&packet),
        type_id: parse_packet_type_id(&packet),
        contents: None,
        subpackets: Some(subpackets),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_hexadecimal_operator_packet() {
        let hex = "620080001611562C8802118E34";
        let packet = hexadecimal_to_binary(hex);
        let output = parse_operator_packet(&packet);
        dbg!(output);
    }

    #[test]
    fn it_parses_packet_versions() {
        let packet = "110100101111111000101000".to_string();
        let version = parse_packet_version(&packet);
        assert_eq!(version, 6);
    }

    #[test]
    fn it_parses_packet_type_ids() {
        let packet = "110100101111111000101000".to_string();
        let type_id = parse_packet_type_id(&packet);
        assert_eq!(type_id, 4);
    }

    #[test]
    fn it_parses_packet_type_id_4() {
        let packet = hexadecimal_to_binary("D2FE28");
        let packet = parse_type_id_4_packet(&packet);
        assert_eq!(packet.contents, Some("011111100101".to_string()));
    }

    #[test]
    fn it_parses_length_type_id_of_an_operator_packet() {
        let packet = hexadecimal_to_binary("38006F45291200");
        let length_type_id = parse_length_type_id(&packet);
        assert_eq!(length_type_id, 0);
    }

    #[test]
    fn it_parses_packet_type_id_6_with_length_type_id_0() {
        let packet = hexadecimal_to_binary("38006F45291200");
        let packet = parse_operator_packet(&packet);
        let subpackets = packet.subpackets.unwrap();
        dbg!(&subpackets);
        assert_eq!(subpackets.len(), 2);
        assert_eq!(subpackets[0].contents, Some("1010".to_string()));
        assert_eq!(subpackets[1].contents, Some("00010100".to_string()));
    }

    #[test]
    fn it_parses_packet_type_id_3_with_length_type_id_1() {
        let packet = hexadecimal_to_binary("EE00D40C823060");
        let packet = parse_operator_packet(&packet);
        dbg!(&packet);
    }

    #[test]
    fn it_converts_hexadecimal_to_binary() {
        let packet = "D2FE28";
        let output = hexadecimal_to_binary(packet);
        assert_eq!(output, "110100101111111000101000");
    }

    #[test]
    fn it_handles_nested_operator_packets() {
        let packet = hexadecimal_to_binary("8A004A801A8002F478");
        let packet = parse_operator_packet(&packet);
        dbg!(packet);
    }
}
