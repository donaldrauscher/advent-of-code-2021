use std::cmp;

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

fn parse(bits: &Vec<u8>, start: usize) -> (usize, usize, usize) {
    // determine packet version/type
    let packet_version = bits_to_number(&bits[start..=(start+2)]);
    let packet_type = bits_to_number(&bits[(start+3)..=(start+5)]);

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
        let output = bits_to_number(&lit[..]);
        return (output, packet_version, i);

    // operator
    } else {
        let mut i = start;
        let mut outputs: Vec<usize> = Vec::new();
        let mut packet_version_sum: usize = packet_version;

        let length_type_id = bits[start+6];

        // total length of subpackets
        if length_type_id == 0 {
            let subpacket_length = bits_to_number(&bits[(start+7)..=(start+21)]);
            let mut output;
            let mut pv;
            i += 22;
            while i < (start + subpacket_length + 22) {
                (output, pv, i) = parse(bits, i);
                outputs.push(output);
                packet_version_sum += pv;
            }
        // total number of subpackages
        } else {
            let num_subpackets = bits_to_number(&bits[(start+7)..=(start+17)]);
            let mut output;
            let mut pv;
            i = start + 18;
            for _ in 1..=num_subpackets {
                (output, pv, i) = parse(bits, i);
                outputs.push(output);
                packet_version_sum += pv;
            }
        }

        let output = match packet_type {
            0 => outputs.iter().fold(0, |sum, s| sum + s),
            1 => outputs.iter().fold(1, |prod, s| prod * s),
            2 => outputs.iter().fold(usize::max_value(), |min, s| cmp::min(min, *s)),
            3 => outputs.iter().fold(0, |max, s| cmp::max(max, *s)),
            5 => if outputs[0] > outputs[1] { 1 } else { 0 },
            6 => if outputs[0] < outputs[1] { 1 } else { 0 },
            7 => if outputs[0] == outputs[1] { 1 } else { 0 },
            _ => todo!()
        };
        return (output, packet_version_sum, i);
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

    let (output, package_version_sum, _) = parse(&bits, 0);
    println!("Output: {}", output);
    println!("Package version sum: {}", package_version_sum);
}
