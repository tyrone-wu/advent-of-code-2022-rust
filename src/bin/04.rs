pub fn part_one(input: &str) -> Option<u32> {
    // Number of pairs
    let mut pairs: u32 = 0;

    // Iterate lines
    for line in input.lines() {
        // Split elfs
        let elfs: Vec<&str> = line.split(',').collect();

        // Get elf one and elf two values
        let elf_one: Vec<u8> = elfs[0]
            .split('-')
            .map(|n| n.parse::<u8>().unwrap())
            .collect();
        let elf_two: Vec<u8> = elfs[1]
            .split('-')
            .map(|n| n.parse::<u8>().unwrap())
            .collect();

        if (elf_one[0] <= elf_two[0] && elf_one[1] >= elf_two[1])
            || (elf_one[0] >= elf_two[0] && elf_one[1] <= elf_two[1])
        {
            pairs += 1;
        }
    }

    Some(pairs)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
