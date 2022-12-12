// Number of characters until start-of-packet marker for variable length sequence
fn message_marker(input: &str, seq_len: usize) -> Option<u32> {
    // Input as array of chars
    let input_arr = input.as_bytes();

    // Start pointer
    let mut start: usize = 0;

    // Iterate packets
    for (end, char) in input_arr.iter().enumerate() {
        // So that start pointer and end pointer do not count as duplicate
        if start != end {
            // Check that char at end pointer is unique from start pointer
            #[allow(clippy::needless_range_loop)]
            for i in start..end {
                // If char is duplicate, move start pointer to the char after the duplicate
                if char == &input_arr[i] {
                    start = i + 1;
                    break;
                }
            }

            // When 4 unique chars in sequence is reached
            if (end - start + 1) == seq_len {
                return Some((end as u32) + 1);
            }
        }
    }

    // Solution doesn't exist
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    message_marker(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    message_marker(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
