use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

use std::cmp;

// ----------------------------------------------------------------------------

#[derive(PartialEq, Eq)]
enum Packet {
    Integer(u8),
    List(Vec<Packet>),
}

// Defines total ordering of Packets; https://doc.rust-lang.org/std/cmp/trait.Ord.html
impl Ord for Packet {
    // Ways for ordering two Packets
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            // Directly compare ints
            (Packet::Integer(l), Packet::Integer(r)) => l.cmp(r),
            // Wrap int in vec of Packet and recursively compare
            (Packet::Integer(l), Packet::List(r)) => vec![Packet::Integer(*l)].cmp(r),
            // Wrap int in vec of Packet and recursively compare
            (Packet::List(l), Packet::Integer(r)) => l.cmp(&vec![Packet::Integer(*r)]),
            // Compare vecs of packets
            (Packet::List(l), Packet::List(r)) => l.cmp(r),
        }
    }
}

// Defines partial ordering of Packets; https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
impl PartialOrd for Packet {
    // Ways to orderin two Packets
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// ----------------------------------------------------------------------------

// Parse packet
fn parse_packet(input: &str) -> IResult<&str, Packet> {
    let (input, packet) = alt((
        // Match Integer variant
        complete::u8.map(Packet::Integer),
        // Match inside contents of "[]"
        delimited(
            tag("["), // Discard '['
            separated_list0(
                tag(","),     // Discard ','
                parse_packet, // Match Packet enum recursively
            )
            .map(Packet::List), // Match List variant
            tag("]"), // Discard ']'
        ),
    ))(input)?;

    Ok((input, packet))
}

// Parse pairs of packets
fn parse_pairs_list(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    let (input, pairs): (&str, Vec<(Packet, Packet)>) = separated_list1(
        tag("\n\n"), // Discard 2 newlines
        separated_pair(
            parse_packet, // Match Packet enum
            newline,      // Discard newline
            parse_packet, // Match Packet enum
        ),
    )(input)?;

    Ok((input, pairs))
}

// ----------------------------------------------------------------------------

// Same as comparator but in funciton form to get more understanding
fn order_type(left: &Packet, right: &Packet) -> String {
    match (&left, &right) {
        // Directly compare integers
        (Packet::Integer(l), Packet::Integer(r)) =>
        {
            #[allow(clippy::comparison_chain)]
            if l < r {
                String::from("Less")
            } else if l == r {
                String::from("Equal")
            } else {
                String::from("Greater")
            }
        }
        // Wrap int in vec of Packet and recursively call fn
        (Packet::Integer(l), Packet::List(_)) => {
            order_type(&Packet::List(vec![Packet::Integer(*l)]), right)
        }
        // Wrap int in vec of Packet and recursively call fn
        (Packet::List(_), Packet::Integer(r)) => {
            order_type(left, &Packet::List(vec![Packet::Integer(*r)]))
        }
        // Iterate both vecs and recursively call fn
        (Packet::List(l), Packet::List(r)) => {
            // Iterate left and right
            let mut l_it = l.iter().peekable();
            let mut r_it = r.iter().peekable();

            while l_it.peek().is_some() && r_it.peek().is_some() {
                let order_type: String = order_type(l_it.next().unwrap(), r_it.next().unwrap());
                if order_type != "Equal" {
                    return order_type;
                }
            }

            if l_it.peek().is_none() && r_it.peek().is_some() {
                // Left finishes first
                String::from("Less")
            } else if l_it.peek().is_none() && r_it.peek().is_none() {
                // Both finish same time
                String::from("Equal")
            } else {
                // Rgiht finishes first
                String::from("Greater")
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    // Get vector of pairs
    let (_, pairs): (&str, Vec<(Packet, Packet)>) = parse_pairs_list(input).unwrap();

    let fn_sum: usize = pairs
        .iter()
        .enumerate()
        .map(
            |(i, (l, r))| {
                if order_type(l, r) == "Less" {
                    i + 1
                } else {
                    0
                }
            },
        )
        .sum::<usize>();

    let cmp_sum: usize = pairs
        .iter()
        .enumerate()
        .map(|(i, (l, r))| match l.cmp(r) {
            cmp::Ordering::Less => i + 1,
            _ => 0,
        })
        .sum::<usize>();

    assert_eq!(fn_sum, cmp_sum);

    Some(cmp_sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, pairs): (&str, Vec<(Packet, Packet)>) = parse_pairs_list(input).unwrap();
    let mut packet_vec: Vec<&Packet> = pairs.iter().flat_map(|(l, r)| vec![l, r]).collect();

    // Divider packets
    let divider_two: Packet = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    packet_vec.push(&divider_two);
    let divider_six: Packet = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
    packet_vec.push(&divider_six);

    // Sort and find indices of divider packets
    packet_vec.sort();
    let d_two_idx: usize = packet_vec.iter().position(|&p| p == &divider_two).unwrap() + 1;
    let d_six_idx: usize = packet_vec.iter().position(|&p| p == &divider_six).unwrap() + 1;

    Some(d_two_idx * d_six_idx)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
