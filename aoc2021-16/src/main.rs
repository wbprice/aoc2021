fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Packet {
    raw: String,
    length_type: Option<usize>,
    version: usize,
    type_id: usize,
    contents: Option<String>,
    bits: usize,
    subpackets: Option<Vec<Packet>>,
}

fn parse_packet_version(packet: &str) -> usize {
    usize::from_str_radix(&packet[0..3], 2).unwrap()
}

// Type 4 - Literal
// Other types - operator
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

fn parse_type_id_4_packet_contents(packet: &str) -> (String, usize) {
    let mut content = "".to_string();
    let mut bits_read = 6;
    loop {
        // read 5 bit chunks
        // The signal to continue or quit is the first bit
        // The data is the following 4 bits
        let chunk = &packet[bits_read..bits_read + 5];
        let last_chunk = &chunk.chars().next().unwrap() == &'0';
        content += &chunk[1..];
        bits_read += 5;
        if last_chunk {
            break;
        }
    }

    // // Account for any 0 padding
    // while bits_read % 4 > 0 {
    //     bits_read += 1;
    // }

    (content, bits_read)
}

fn parse_operator_packet_length_type_0_bit_length(packet: &str) -> usize {
    usize::from_str_radix(&packet[7..22], 2).unwrap()
}

fn parse_operator_packet_length_type_1_subpackets_count(packet: &str) -> usize {
    usize::from_str_radix(&packet[7..18], 2).unwrap()
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

fn sum_packet_versions(packets: Vec<Packet>, memo: u32) -> u32 {
    let mut output = 0;

    for packet in packets {
        output += packet.version as u32;

        if let Some(subpackets) = packet.subpackets {
            output += sum_packet_versions(subpackets, output);
        }
    }

    output
}

fn parse_packets(packet: &str) -> Vec<Packet> {
    let version = parse_packet_version(packet);
    let type_id = parse_packet_type_id(packet);
    let mut output: Vec<Packet> = vec![];

    // Is this an literal value packet or an operator packet?
    if type_id == 4 {
        // A literal packet
        let (contents, bits_read) = parse_type_id_4_packet_contents(&packet);
        output.push(Packet {
            version,
            type_id,
            contents: Some(contents),
            subpackets: None,
            bits: bits_read,
            raw: packet[0..bits_read].to_string(),
            length_type: None,
        });
    } else {
        // An operator packet
        // Does this packet have length type 0 or 1?
        let length_type = parse_length_type_id(packet);
        if length_type == 0 {
            // length type 0
            // The next n bits have subpackets
            // Read subpackets until enough bits have been read
            let subpackets_bits = parse_operator_packet_length_type_0_bit_length(&packet);
            let subpackets_last_bit = subpackets_bits + 22;
            let subpackets_slice = &packet[22..subpackets_last_bit];
            let mut bits_read = 0;
            let mut subpackets: Vec<Packet> = vec![];

            while bits_read < subpackets_bits {
                let mut packets = parse_packets(&subpackets_slice[bits_read..]);
                bits_read += packets.iter().fold(0, |acc, p| acc + p.bits);
                subpackets.append(&mut packets);
            }

            output.push(Packet {
                version,
                type_id,
                contents: None,
                length_type: Some(length_type),
                subpackets: Some(subpackets),
                bits: bits_read,
                raw: subpackets_slice.to_string(),
            });
        } else {
            // length type 1
            // The next set of bits has a number of subpackets
            // Read subpackets until enough subpackets have been read
            let subpackets_count = parse_operator_packet_length_type_1_subpackets_count(&packet);
            let subpackets_slice = &packet[18..];
            let mut packets_read = 0;
            let mut bits_read = 0;
            let mut subpackets: Vec<Packet> = vec![];

            while packets_read < subpackets_count {
                let mut packets = parse_packets(&subpackets_slice[bits_read..]);
                bits_read += packets.iter().fold(0, |acc, p| acc + p.bits);
                packets_read += packets.len();
                subpackets.append(&mut packets);
            }

            // Account for any 0 padding
            while bits_read % 4 > 0 {
                bits_read += 1;
            }

            output.push(Packet {
                version,
                type_id,
                length_type: Some(length_type),
                contents: None,
                subpackets: Some(subpackets),
                bits: bits_read,
                raw: subpackets_slice.to_string(),
            });
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let packets = parse_packets(&packet);
        assert_eq!(packets[0].contents, Some("011111100101".to_string()));
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
        let packets = parse_packets(&packet);
        let subpackets = packets[0].subpackets.as_ref().unwrap();
        assert_eq!(subpackets.len(), 2);
        assert_eq!(subpackets[0].contents, Some("1010".to_string()));
        assert_eq!(subpackets[1].contents, Some("00010100".to_string()));
    }

    #[test]
    fn it_parses_packet_type_id_3_with_length_type_id_1() {
        let packet = hexadecimal_to_binary("EE00D40C823060");
        let packets = parse_packets(&packet);
        dbg!(&packets);
        let subpackets = packets[0].subpackets.as_ref().unwrap();
        assert_eq!(subpackets.len(), 3);
    }

    #[test]
    fn it_converts_hexadecimal_to_binary() {
        let packet = "D2FE28";
        let output = hexadecimal_to_binary(packet);
        assert_eq!(output, "110100101111111000101000");
    }

    #[test]
    fn it_sums_nested_packet_versions() {
        let packet = hexadecimal_to_binary("8A004A801A8002F478");
        let packets = parse_packets(&packet);
        let sum = sum_packet_versions(packets, 0);
        assert_eq!(sum, 16);
    }

    #[test]
    fn it_sums_packet_versions_v2() {
        let packet = hexadecimal_to_binary("620080001611562C8802118E34");
        let packets = parse_packets(&packet);
        dbg!(&packets);
        let sum = sum_packet_versions(packets, 0);
        assert_eq!(sum, 12);
    }

    #[test]
    fn it_sums_packet_versions_v3() {
        let packet = hexadecimal_to_binary("C0015000016115A2E0802F182340");
        let packets = parse_packets(&packet);
        let sum = sum_packet_versions(packets, 0);
        assert_eq!(sum, 23);
    }

    #[test]
    fn it_sums_packet_versions_v4() {
        let packet = hexadecimal_to_binary("A0016C880162017C3686B18A3D4780");
        let packets = parse_packets(&packet);
        let sum = sum_packet_versions(packets, 0);
        assert_eq!(sum, 31);
    }
}
