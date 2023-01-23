use std::collections::HashSet;

struct Position {
    x: u8,
    y: u8
}

struct Me {
    position: Position,

}

fn bfs() ->  {

}

pub fn part_one(input: &str) -> Option<u32> {
    // 2d vec map
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // Get current and goal position
    let mut curr_pos: Position = Position { x: map[0].iter().position(|&c| c == '.').unwrap() as u8, y: 0 };
    let goal: Position = Position { x: map[map.len() - 1].iter().position(|&c| c == '.').unwrap() as u8, y: map.len() as u8 };

    // // Step until reached goal
    // let mut reached_end: bool = false;
    // let mut visited: HashSet<(Position, u8)> = 
    // while !reached_end {

        
    // }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), None);
    }
}
