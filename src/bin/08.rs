// Generate grid map from input
fn generate_map(input: &str) -> Vec<Vec<u8>> {
    // Initialize empty 2d vector
    let mut map: Vec<Vec<u8>> = Vec::new();

    for line in input.lines() {
        // Append vector of digits to map
        map.push(line.bytes().map(|c| c - b'0').collect());
    }

    map
}

// If tree is visible
fn is_visible(map: &[Vec<u8>], (x, y): (usize, usize)) -> bool {
    // Bottom
    for i in (x + 1)..map.len() {
        if map[x][y] <= map[i][y] {
            break;
        } else if i == (map.len() - 1) {
            return true;
        }
    }
    // Top
    for i in (0..x).rev() {
        if map[x][y] <= map[i][y] {
            break;
        } else if i == 0 {
            return true;
        }
    }
    // Right
    for i in (y + 1)..map[0].len() {
        if map[x][y] <= map[x][i] {
            break;
        } else if i == (map[0].len() - 1) {
            return true;
        }
    }
    // Left
    for i in (0..y).rev() {
        if map[x][y] <= map[x][i] {
            break;
        } else if i == 0 {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    // Input to 2d array
    let map: Vec<Vec<u8>> = generate_map(input);
    // Total visible trees, with initial values of padding
    let mut vis_trees: u32 = 2 * ((map.len() + map[0].len()) as u32) - 4;

    // Iterate over each tree in map except edges
    for x in 1..(map.len() - 1) {
        for y in 1..(map[0].len() - 1) {
            if is_visible(&map, (x, y)) {
                vis_trees += 1;
            }
        }
    }

    Some(vis_trees)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
