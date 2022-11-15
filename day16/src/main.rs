
fn decode(c: char) -> String {
    match c {
        '0' => "0000".to_string(),
        '1' => "0001".to_string(),
        '2' => "0010".to_string(),
        '3' => "0011".to_string(),
        '4' => "0100".to_string(),
        '5' => "0101".to_string(),
        '6' => "0110".to_string(),
        '7' => "0111".to_string(),
        '8' => "1000".to_string(),
        '9' => "1001".to_string(),
        'A' => "1010".to_string(),
        'B' => "1011".to_string(),
        'C' => "1100".to_string(),
        'D' => "1101".to_string(),
        'E' => "1110".to_string(),
        'F' => "1111".to_string(),
        _ => todo!()
    }
}

fn bits_to_number(bits: &[u8]) -> usize {
    let mut out: usize = 0;
    let base: usize = 2;
    for (i, b) in bits.iter().rev().enumerate() {
        if b > &0 {
            out += base.pow(i.try_into().unwrap());
        }
    }
    return out
}

fn parse(bits: &Vec<u8>, versions: &mut Vec<usize>, start: usize) -> usize {
    // determine packet version/type
    let packet_version = bits_to_number(&bits[start..=(start+2)]);
    let packet_type = bits_to_number(&bits[(start+3)..=(start+5)]);

    // add packet version
    versions.push(packet_version);

    // literal
    if packet_type == 4 {
        let mut i = start + 6;
        let mut exit: bool = false;
        let mut lit: Vec<u8> = Vec::new();
        while !exit {
            if bits[i] == 0 {
                exit = true;
            }
            lit.extend_from_slice(&bits[(i+1)..=(i+4)]);
            i += 5;
        }
        // println!("Literal: {:?}", bits_to_number(&lit[..]));
        return i;

    // operator
    } else {
        let length_type_id = bits[start+6];
        // total length of subpackets
        if length_type_id == 0 {
            let subpacket_length = bits_to_number(&bits[(start+7)..=(start+21)]);
            let mut i = start + 22;
            while i < (start + subpacket_length + 22) {
                i = parse(bits, versions, i);
            }
            return i;
        // total number of subpackages
        } else {
            let num_subpackets = bits_to_number(&bits[(start+7)..=(start+17)]);
            let mut i = start + 18;
            for _ in 1..=num_subpackets {
                i = parse(bits, versions, i);
            }
            return i;
        }
    }
}

fn main() {
    let bits = include_str!("../input.txt")
        .trim()
        .chars()
        .flat_map(|c| {
            decode(c)
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    let mut versions: Vec<usize> = Vec::new();
    parse(&bits, &mut versions, 0);
    println!("Version Total: {}", versions.iter().fold(0, |total, v| total + v));
}
