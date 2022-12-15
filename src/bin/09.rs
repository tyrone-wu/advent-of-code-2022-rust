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

pub fn part_one(input: &str) -> Option<usize> {
    // Track current coordinate of head and tail
    let mut head_coord: (i32, i32) = (0, 0);
    let mut tail_coord: (i32, i32) = (0, 0);

    // Track visited coordinates for the tail
    let mut visited_coord: HashSet<(i32, i32)> = HashSet::new();
    visited_coord.insert((0, 0));

    // Parse list of move instrucitons
    let (_, instructions): (&str, Vec<(&str, u8)>) = parse_instructions(input).unwrap();

    for (dir, steps) in instructions {
        // Perform moves
        for _ in 0..steps {
            let head_prev_loc: (i32, i32) = head_coord;

            // Move head a single step
            match dir {
                "U" => head_coord.1 += 1,
                "D" => head_coord.1 -= 1,
                "R" => head_coord.0 += 1,
                "L" => head_coord.0 -= 1,
                _ => panic!("Invalid direction: {dir}"),
            }

            // Check if tail needs to move
            if tail_coord.0.abs_diff(head_coord.0) > 1 || tail_coord.1.abs_diff(head_coord.1) > 1 {
                tail_coord = head_prev_loc;
                visited_coord.insert(tail_coord);
            }
        }
    }

    Some(visited_coord.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    // Track coordinates of rope
    let mut rope: [(i32, i32); 10] = [(0, 0); 10];

    // Track visited coordinates for the tail
    let mut visited_coord: HashSet<(i32, i32)> = HashSet::new();
    visited_coord.insert((0, 0));

    // Parse list of move instrucitons
    let (_, instructions): (&str, Vec<(&str, u8)>) = parse_instructions(input).unwrap();

    for (dir, steps) in instructions {
        // Perform moves
        for _ in 0..steps {
            // Move head a single step
            match dir {
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                "R" => rope[0].0 += 1,
                "L" => rope[0].0 -= 1,
                _ => panic!("Invalid direction: {dir}"),
            }

            // Move knots
            for i in 1..rope.len() {
                // Types of gap closers
                if rope[i].0.abs_diff(rope[i - 1].0) > 1 && rope[i].1 == rope[i - 1].1 {
                    // Move x axis
                    rope[i].0 += (rope[i - 1].0 - rope[i].0).signum();
                } else if rope[i].0 == rope[i - 1].0 && rope[i].1.abs_diff(rope[i - 1].1) > 1 {
                    // Move y axis
                    rope[i].1 += (rope[i - 1].1 - rope[i].1).signum();
                } else if (rope[i].0.abs_diff(rope[i - 1].0) + rope[i].1.abs_diff(rope[i - 1].1))
                    > 2
                {
                    // Diagonal move
                    rope[i].0 += (rope[i - 1].0 - rope[i].0).signum();
                    rope[i].1 += (rope[i - 1].1 - rope[i].1).signum();
                } else {
                    break;
                }
            }

            // Add tail coordinate
            visited_coord.insert(rope[9]);
        }
    }

    Some(visited_coord.len())
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
        let input_one = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input_one), Some(1));

        let input_two: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        assert_eq!(part_two(input_two), Some(36));
    }
}
