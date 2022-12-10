pub fn part_one(input: &str) -> Option<u32> {
    // Max calories count
    let mut max_calories: u32 = 0;
    // Accumulated calories for a single elf
    let mut cum_calories: u32 = 0;

    // Iterate over each line
    for line in input.lines() {
        // Accumulate calories for the elf
        if !line.is_empty() {
            let calories: u32 = line.parse().expect("Expected 32-bit unsigned integer.");
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
    // Top 3 calories
    let mut top_calories: [u32; 3] = [0; 3];
    // Index of the minimum top calories
    let mut index_min: usize = 0;
    // Accumulated calories for a single elf
    let mut cum_calories: u32 = 0;

    // Iterate each line
    for line in input.lines() {
        // Accumulate calories for elf
        if !line.is_empty() {
            let calories: u32 = line.parse().expect("Expected 32-bit unsigned integer.");
            cum_calories += calories;
        }
        // Finished accumulating calories for elf
        else {
            // Replace the min top calories value if accumulated calories is greater
            if top_calories[index_min] < cum_calories {
                top_calories[index_min] = cum_calories;

                // Re-compute the index of the min top calories
                for (i, calories) in top_calories.iter().enumerate() {
                    if calories < &cum_calories {
                        index_min = i;
                        cum_calories = *calories;
                    }
                }
            }

            // Reset accumulated calories for the next elf
            cum_calories = 0;
        }
    }

    // Fence post check
    if top_calories[index_min] < cum_calories {
        top_calories[index_min] = cum_calories;
    }

    Some(top_calories.iter().sum())
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
        assert_eq!(part_two(&input), Some(45000));
    }
}
