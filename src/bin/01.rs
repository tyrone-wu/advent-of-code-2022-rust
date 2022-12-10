pub fn part_one(input: &str) -> Option<u32> {

    // Max calories count
    let mut max_calories: u32 = 0;
    // Holds total calories to accumulate
    let mut cum_calories: u32 = 0;

    // Iterate over each line
    for line in input.lines() {

        // Accumulate calories for the elf
        if !line.is_empty() {
            let calories: u32 = line.parse().expect("Expected 64-bit unsigned integer.");
            cum_calories += calories;
        } 
        // Finished accumulating calories for elf
        else {
            // If the current elf is the fattest, set as fattest elf
            if max_calories < cum_calories {
                max_calories = cum_calories;
            }

            // Reset accumulated calories for the next elf
            cum_calories = 0;
        }
    }

    // Fence post check since last elf isn't followed by empty line
    if max_calories < cum_calories {
        max_calories = cum_calories;
    }

    Some(max_calories)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
