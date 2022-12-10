pub fn part_one(input: &str) -> Option<u32> {

    // Score
    let mut score: u32 = 0;

    // Iterate lines
    for line in input.lines() {
        if line.len() == 3 {
            // Line as bytes for indexing
            let line_bytes: &[u8] = line.as_bytes();
            // Your move
            let my_move: u8 = line_bytes[2] - 88;       // ASCII offset X
            // Opps move
            let opp_move: u8 = line_bytes[0] - 65;      // ASCII offset A

            // Add my move to score
            score += my_move as u32 + 1;

            // Round result
            if my_move == opp_move {
                score += 3;             // Draw
            } else if (opp_move + 1) % 3 == my_move {   // Use remainder to calculate r<p<s<r cycle
                score += 6;             // Win
            }
        } else {
            panic!("Invalid line.");
        }
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
