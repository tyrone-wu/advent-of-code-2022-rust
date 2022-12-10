pub fn part_one(input: &str) -> Option<u32> {
    // Score
    let mut score: u32 = 0;

    // Iterate lines
    for line in input.lines() {
        if line.len() == 3 {
            // Line as bytes for indexing
            let line_bytes: &[u8] = line.as_bytes();
            // Your move
            let my_move: u8 = line_bytes[2] - 88; // ASCII offset X
                                                  // Opps move
            let opp_move: u8 = line_bytes[0] - 65; // ASCII offset A

            // Add my move to score
            score += my_move as u32 + 1;

            // Round result
            if my_move == opp_move {
                score += 3; // Draw
            } else if (opp_move + 1) % 3 == my_move {
                // Use remainder to calculate r<p<s<r cycle
                score += 6; // Win
            }
        } else {
            panic!("Invalid line.");
        }
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Score
    let mut score: u32 = 0;

    // Iterate lines
    for line in input.lines() {
        if line.len() == 3 {
            // Line as bytes for indexing
            let line_bytes: &[u8] = line.as_bytes();
            // Opps move
            let opp_move: u8 = line_bytes[0] - 65; // ASCII offset A
                                                   // Round result
            let round_result: u8 = line_bytes[2] - 88; // ASCII offset X

            // Add round result to score
            score += round_result as u32 * 3;

            // Round result
            // if round_result == 0 {          // Lose
            //     score += (opp_move as u32 + 2) % 3 + 1;
            // } else if round_result == 1 {   // Draw
            //     score += opp_move as u32 + 1;
            // } else if round_result == 2 {   // Win
            //     score += (opp_move as u32 + 1) % 3 + 1;
            // }
            match round_result {
                0 => score += (opp_move as u32 + 2) % 3 + 1, // Lose
                1 => score += opp_move as u32 + 1,           // Draw
                2 => score += (opp_move as u32 + 1) % 3 + 1, // Win
                _ => panic!("Round result invalid."),
            }
        } else {
            panic!("Invalid line.");
        }
    }

    Some(score)
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
        assert_eq!(part_two(&input), Some(12));
    }
}
