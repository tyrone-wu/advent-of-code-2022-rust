use std::collections::LinkedList;

// Parse stack drawing
fn parse_stacks(crates_input: &str) -> Vec<LinkedList<u8>> {
    // Split drawing into lines
    let crate_lines: Vec<&str> = crates_input.lines().collect();
    // Get number of stacks
    let num_stacks: usize = crate_lines
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    // Hold crates
    let mut stack_model: Vec<LinkedList<u8>> = vec![LinkedList::new(); num_stacks];

    // Iterate stack level starting from bottom
    for &level in crate_lines[0..crate_lines.len() - 1].iter().rev() {
        // Offset first char '[' and iterate every 4th char
        let crate_it = level[1..level.len() - 1].as_bytes().iter().step_by(4);

        // Iterate crates
        for (i, c) in crate_it.enumerate() {
            // Push char if
            if *c != b' ' {
                stack_model[i].push_back(*c);
            }
        }
    }

    stack_model
}

// Move crates
fn move_crates(moves_input: &str, stack_model: &mut [LinkedList<u8>]) {
    // Iterate crate moves
    for line in moves_input.lines() {
        // Split into words
        let tokens: Vec<&str> = line.split_whitespace().collect();

        // Number of crates to move
        let num_crates: u8 = tokens[1].parse::<u8>().unwrap();
        // Source stack
        let src_stack: usize = tokens[3].parse::<usize>().unwrap() - 1;
        // Destination stack
        let dst_stack: usize = tokens[5].parse::<usize>().unwrap() - 1;

        // Move crates
        for _ in 0..num_crates {
            let c: u8 = stack_model[src_stack].pop_back().unwrap();
            stack_model[dst_stack].push_back(c);
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    // Split crates drawing and move instructions
    let input_split: Vec<&str> = input.split("\n\n").collect();

    // Generate stack model
    let mut stack_model: Vec<LinkedList<u8>> = parse_stacks(input_split[0]);
    // Move crates according to instructions
    move_crates(input_split[1], &mut stack_model);

    // Get crates at top
    let mut top_crates: String = String::new();
    for stack in &stack_model {
        top_crates.push(*stack.back().unwrap() as char);
    }

    Some(top_crates)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
