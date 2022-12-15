use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    multi::separated_list1,
    IResult,
};

// ----------------------------------------------------------------------------

fn parse_moves(input: &str) -> IResult<&str, (&str, u8)> {
    let (input, direction) = alpha1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, steps) = complete::u8(input)?;

    Ok((input, (direction, steps)))
}

fn parse_instructions(instr: &str) -> IResult<&str, Vec<(&str, u8)>> {
    let (instr, instr_vec) = separated_list1(newline, parse_moves)(instr)?;

    Ok((instr, instr_vec))
}

// ----------------------------------------------------------------------------

fn move_knot(
    (dir, steps): (&str, u8),
    visited_coord: &mut HashSet<(i32, i32)>,
    head: &mut (i32, i32),
    mut tail: (i32, i32),
) -> (i32, i32) {
    // Perform moves
    for _ in 0..steps {
        let head_prev_loc: (i32, i32) = *head;

        // Move head a single step
        match dir {
            "U" => head.1 += 1,
            "D" => head.1 -= 1,
            "R" => head.0 += 1,
            "L" => head.0 -= 1,
            _ => panic!("Invalid direction: {dir}"),
        }

        // Check if tail needs to move
        if tail.0.abs_diff(head.0) > 1 || tail.1.abs_diff(head.1) > 1 {
            tail = head_prev_loc;
            visited_coord.insert(tail);
        }
    }

    tail
}

// ----------------------------------------------------------------------------

pub fn part_one(input: &str) -> Option<usize> {
    // Track current coordinate of the tail
    let mut head_coord: (i32, i32) = (0, 0);
    let mut tail_coord: (i32, i32) = (0, 0);

    // Track visited coordinates for the tail
    let mut visited_coord: HashSet<(i32, i32)> = HashSet::new();
    visited_coord.insert((0, 0));

    // Parse list of move instrucitons
    let (_, instructions): (&str, Vec<(&str, u8)>) = parse_instructions(input).unwrap();

    for instr in instructions {
        tail_coord = move_knot(instr, &mut visited_coord, &mut head_coord, tail_coord);
    }

    Some(visited_coord.len())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
