use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> HashSet<(i32, i32)> {
    let mut elves: HashSet<(i32, i32)> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }

    elves
}

fn propose_move(elves: &HashSet<(i32, i32)>, elf: (i32, i32), mut cycle: u8) -> (i32, i32) {
    let x: i32 = elf.0;
    let y: i32 = elf.1;

    // If surrounding is empty, don't move
    if !elves.contains(&(x, y - 1))
        && !elves.contains(&(x + 1, y - 1))
        && !elves.contains(&(x + 1, y))
        && !elves.contains(&(x + 1, y + 1))
        && !elves.contains(&(x, y + 1))
        && !elves.contains(&(x - 1, y + 1))
        && !elves.contains(&(x - 1, y))
        && !elves.contains(&(x - 1, y - 1))
    {
        return (x, y);
    }

    // Check cardinal directions if elf can move
    for _ in 0..4 {
        match cycle {
            0 => {
                // Move North if (N, NE, NW) empty
                if !elves.contains(&(x, y - 1))
                    && !elves.contains(&(x + 1, y - 1))
                    && !elves.contains(&(x - 1, y - 1))
                {
                    return (x, y - 1);
                }
            }
            1 => {
                // Move South if (S, SE, SW) empty
                if !elves.contains(&(x, y + 1))
                    && !elves.contains(&(x + 1, y + 1))
                    && !elves.contains(&(x - 1, y + 1))
                {
                    return (x, y + 1);
                }
            }
            2 => {
                // Move West if (W, NW, SW) empty
                if !elves.contains(&(x - 1, y))
                    && !elves.contains(&(x - 1, y - 1))
                    && !elves.contains(&(x - 1, y + 1))
                {
                    return (x - 1, y);
                }
            }
            3 => {
                // Move East if (E, NE, SE) empty
                if !elves.contains(&(x + 1, y))
                    && !elves.contains(&(x + 1, y - 1))
                    && !elves.contains(&(x + 1, y + 1))
                {
                    return (x + 1, y);
                }
            }
            _ => panic!("invalid cycle"),
        }

        cycle = (cycle + 1) % 4;
    }

    // Cardinal direction check are not empty, don't move
    (x, y)
}

pub fn part_one(input: &str) -> Option<u32> {
    // Initial positions of the elves
    let mut elves: HashSet<(i32, i32)> = parse_input(input);
    let size: usize = elves.len();

    // Rounds
    let mut cycle: u8 = 0;
    for _ in 0..10 {
        // First half
        let mut proposed_positions: HashMap<(i32, i32), u8> = HashMap::with_capacity(size);
        for elf in elves.iter() {
            // Propose a new position to move
            let new_position: (i32, i32) = propose_move(&elves, *elf, cycle);

            // Add proposed move to set
            proposed_positions
                .entry(new_position)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        // Second half
        let original_positions: HashSet<(i32, i32)> = elves.clone();
        // Remove elfs that have no conflicting proposed moves and add new position to temp vector
        let mut add_pos: Vec<(i32, i32)> = Vec::with_capacity(size);
        elves.retain(|&elf| {
            // Propose a new position to move
            let new_position: (i32, i32) = propose_move(&original_positions, elf, cycle);
            // Retain elf if they have conflicting moves; otherwise, remove and add new position to vec
            if elf != new_position && *proposed_positions.get(&new_position).unwrap() == 1 {
                add_pos.push(new_position);
                false
            } else {
                true
            }
        });
        // Add new positions of elfs that moved
        for elf in add_pos.iter() {
            elves.insert(*elf);
        }

        cycle = (cycle + 1) % 4;
    }

    // Get rectangle
    let mut min_x: i32 = i32::MAX;
    let mut max_x: i32 = i32::MIN;
    let mut min_y: i32 = i32::MAX;
    let mut max_y: i32 = i32::MIN;
    for (x, y) in elves.iter() {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }

    Some(((max_x - min_x) as u32 + 1) * ((max_y - min_y) as u32 + 1) - elves.len() as u32)
}

pub fn part_two(input: &str) -> Option<u16> {
    // Initial positions of the elves
    let mut elves: HashSet<(i32, i32)> = parse_input(input);
    let size: usize = elves.len();

    // Rounds
    let mut cycle: u8 = 0;
    let mut not_converged: bool = true;
    let mut rounds: u16 = 0;
    while not_converged {
        // First half
        let mut proposed_positions: HashMap<(i32, i32), u8> = HashMap::with_capacity(size);
        for elf in elves.iter() {
            // Propose a new position to move
            let new_position: (i32, i32) = propose_move(&elves, *elf, cycle);

            // Add proposed move to set
            proposed_positions
                .entry(new_position)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        // Second half
        let original_positions: HashSet<(i32, i32)> = elves.clone();
        // Remove elfs that have no conflicting proposed moves and add new position to temp vector
        let mut add_pos: Vec<(i32, i32)> = Vec::with_capacity(size);
        elves.retain(|&elf| {
            // Propose a new position to move
            let new_position: (i32, i32) = propose_move(&original_positions, elf, cycle);
            // Retain elf if they have conflicting moves; otherwise, remove and add new position to vec
            if elf != new_position && *proposed_positions.get(&new_position).unwrap() == 1 {
                add_pos.push(new_position);
                false
            } else {
                true
            }
        });
        // Add new positions of elfs that moved
        for elf in add_pos.iter() {
            elves.insert(*elf);
        }

        cycle = (cycle + 1) % 4;
        rounds += 1;

        if add_pos.is_empty() {
            not_converged = false;
        }
    }

    Some(rounds)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
