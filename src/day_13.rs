use std::{error::Error, fmt::Display, fs::File, io::Read, time::Instant};

use log::info;

use crate::ProblemPart;

pub fn solve(puzzle_input: &str, part: ProblemPart) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(puzzle_input)?;
    let mut puzzle_input = String::new();
    file.read_to_string(&mut puzzle_input)?;

    let result = match part {
        ProblemPart::One => {
            info!("Start solving part 1");
            let start = Instant::now();
            let result = solve_pt1(puzzle_input)?;
            let duration = start.elapsed().as_secs();
            info!("Solved part 1 in {duration} seconds.");
            result
        }
        ProblemPart::Two => {
            info!("Start solving part 2");
            let start = Instant::now();
            let result = solve_pt2(puzzle_input)?;
            let duration = start.elapsed().as_secs();
            info!("Solved part 2 in {duration} seconds.");
            result
        }
    };
    info!("Problem solution is {}", result);
    Ok(())
}

#[derive(Debug, PartialEq, Clone)]
enum PacketElement {
    Num(u32),
    Pack(Packet),
}

impl Display for PacketElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketElement::Num(num) => write!(f, "{num}"),
            PacketElement::Pack(pack) => write!(f, "{pack}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Packet {
    content: Vec<PacketElement>,
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[").unwrap();
        for el in self.content.iter() {
            write!(f, "{el},").unwrap();
        }
        write!(f, "]")
    }
}

impl Packet {
    fn from_string(input: &str) -> (usize, Packet) {
        let mut content: Vec<PacketElement> = Vec::new();
        let mut elems = 0;
        let mut content_iter = input.chars().skip(1).enumerate();
        // by scanning chars we ignore numbers with more than one digits
        // therefore, we save the chars to this variable and whenever we
        // read [ ] or , we close the number and we add it to the content list
        let mut num_to_build = String::new();
        while let Some((i, el)) = content_iter.next() {
            if let Some(num) = el.to_digit(10) {
                //content.push(PacketElement::Num(num));
                num_to_build.push_str(&num.to_string());
            } else if el == '[' {
                if !num_to_build.is_empty() {
                    content.push(PacketElement::Num(num_to_build.parse::<u32>().unwrap()));
                    num_to_build = String::new();
                }
                let (n, pack) = Packet::from_string(&input[i + 1..]);
                content.push(PacketElement::Pack(pack));
                // we skip the number of chars that composed the created packet
                for _ in 0..=n {
                    content_iter.next();
                }
            } else if el == ']' {
                if !num_to_build.is_empty() {
                    content.push(PacketElement::Num(num_to_build.parse::<u32>().unwrap()));
                    // useless because we close the loop after
                    //num_to_build = String::new();
                }
                elems = i + 1;
                break;
            } else {
                // we read a comma
                if !num_to_build.is_empty() {
                    content.push(PacketElement::Num(num_to_build.parse::<u32>().unwrap()));
                    num_to_build = String::new();
                }
            }
        }
        (elems, Packet { content })
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.content.eq(&other.content)
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut self_iter = self.content.iter();
        let mut other_iter = other.content.iter();

        let mut self_elem = self_iter.next();
        let mut other_elem = other_iter.next();

        while other_elem.is_some() & self_elem.is_some() {
            match (self_elem, other_elem) {
                (Some(PacketElement::Num(self_num)), Some(PacketElement::Num(other_num))) => {
                    match self_num.cmp(other_num) {
                        std::cmp::Ordering::Equal => {}
                        ord => return ord,
                    };
                    // they are equal so we continue checking
                }
                (
                    Some(PacketElement::Pack(self_packet)),
                    Some(PacketElement::Pack(other_packet)),
                ) => {
                    let cmp_result = self_packet.cmp(other_packet);
                    if cmp_result != std::cmp::Ordering::Equal {
                        return cmp_result;
                    }
                    // they are equal so we continue checking
                }
                (Some(self_num), Some(PacketElement::Pack(other_packet))) => {
                    let self_packet = Packet {
                        content: vec![(*self_num).clone()],
                    };
                    let cmp_result = self_packet.cmp(other_packet);
                    if cmp_result != std::cmp::Ordering::Equal {
                        return cmp_result;
                    }
                    // they are equal so we continue checking
                }
                (Some(PacketElement::Pack(self_packet)), Some(other_num)) => {
                    let other_packet = Packet {
                        content: vec![(*other_num).clone()],
                    };
                    let cmp_result = self_packet.cmp(&other_packet);
                    if cmp_result != std::cmp::Ordering::Equal {
                        return cmp_result;
                    }
                    // they are equal so we continue checking
                }
                (_, _) => break,
            }
            self_elem = self_iter.next();
            other_elem = other_iter.next();
        }

        // if right side run out of tiems then self is greater than it
        if other_elem.is_none() & self_elem.is_some() {
            std::cmp::Ordering::Greater
        } else if other_elem.is_some() & self_elem.is_none() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

fn parse_input(puzzle_input: String) -> Vec<(Packet, Packet)> {
    let mut pairs = Vec::new();

    for group in puzzle_input
        .lines()
        .filter(|z| !z.is_empty())
        .collect::<Vec<&str>>()
        .chunks(2)
    {
        let first = Packet::from_string(group[0]).1;
        let second = Packet::from_string(group[1]).1;
        pairs.push((first, second));
    }
    pairs
}

fn solve_pt1(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let pairs = parse_input(puzzle_input);
    let mut right_order_pairs = Vec::new();
    for (i, (left, right)) in pairs.iter().enumerate() {
        if left < right {
            println!("\n\nLEFT\n{}", left);
            println!("RIGHT\n{}", right);
            right_order_pairs.push(i as i32 + 1);
            let _ = left.cmp(right);
        }
    }
    Ok(right_order_pairs.iter().sum::<i32>().to_string())
}

fn solve_pt2(puzzle_input: String) -> Result<String, Box<dyn Error>> {
    let pairs = parse_input(puzzle_input);
    let start_divider = Packet {
        content: vec![PacketElement::Pack(Packet {
            content: vec![PacketElement::Num(2)],
        })],
    };
    let end_divider = Packet {
        content: vec![PacketElement::Pack(Packet {
            content: vec![PacketElement::Num(6)],
        })],
    };
    let mut packets: Vec<Packet> = vec![start_divider.clone(), end_divider.clone()];
    for (left, right) in pairs {
        packets.push(left);
        packets.push(right);
    }
    packets.sort();

    let start_divider_index = packets.iter().position(|x| *x == start_divider).unwrap();
    let end_divider_index = packets.iter().position(|x| *x == end_divider).unwrap();
    Ok(((start_divider_index + 1) * (end_divider_index + 1)).to_string())
}

#[cfg(test)]
mod test {
    use std::{error::Error, fs::File, io::Read};

    use super::{solve_pt1, solve_pt2};

    #[test]
    fn test_pt1() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_13_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt1(puzzle_input)?;

        assert_eq!("13".to_string(), result);

        Ok(())
    }

    #[test]
    fn test_pt2() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("inputs/day_13_example.txt")?;
        let mut puzzle_input = String::new();
        file.read_to_string(&mut puzzle_input)?;
        let result = solve_pt2(puzzle_input)?;

        assert_eq!("140".to_string(), result);

        Ok(())
    }
}
