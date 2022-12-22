use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

// Parse rock paths from input of ranges
fn parse_ranges(input: &str) -> IResult<&str, Vec<Vec<(u16, u8)>>> {
    // Alternate between sep to discard, and f to extract
    let (input, lines) = separated_list1(
        newline, // Discard newline
        // Alternate between sep to discard, and f to extract
        separated_list1(
            tag(" -> "), // Discard arrow
            // Extract pair separated by ,
            separated_pair(
                complete::u16, // Match x
                tag(","),      // Match and discard ,
                complete::u8,  // Match y
            ),
        ),
    )(input)?;

    Ok((input, lines))
}

// Generate all points for the rocks
fn generate_map(parsed_input: Vec<Vec<(u16, u8)>>) -> HashSet<(u16, u8)> {
    HashSet::from_iter(parsed_input.iter().flat_map(|ranges| {
        ranges.iter().enumerate().flat_map(|(i, (x_b, y_b))| {
            // Generate vector of points for the line of rocks
            let mut points: Vec<(u16, u8)> = Vec::new();

            // Skip first since we look previous element
            if i != 0 {
                // If line is vertical
                if ranges[i - 1].0 == *x_b {
                    // Order stand and end in a valid range
                    if ranges[i - 1].1 < *y_b {
                        for y in (ranges[i - 1].1)..=(*y_b) {
                            points.push((*x_b, y));
                        }
                    } else {
                        for y in (*y_b)..=(ranges[i - 1].1) {
                            points.push((*x_b, y));
                        }
                    }
                }
                // If line is horizontal
                else {
                    // Order stand and end in a valid range
                    #[allow(clippy::collapsible_else_if)]
                    if ranges[i - 1].0 < *x_b {
                        for x in (ranges[i - 1].0)..=(*x_b) {
                            points.push((x, *y_b));
                        }
                    } else {
                        for x in (*x_b)..=(ranges[i - 1].0) {
                            points.push((x, *y_b));
                        }
                    }
                }
            }
            points
        })
    }))
}

// bc i can't do it while generating the map at the same time
fn get_deepest_level(map: &HashSet<(u16, u8)>) -> u8 {
    map.iter()
        .max_by(|(_, y_i), (_, y_j)| y_i.cmp(y_j))
        .unwrap()
        .1
}

// Drops sand from spawn location and returns point at rest
fn drop_sand(map: &mut HashSet<(u16, u8)>, mut sand_coord: (u16, u8), deepest_y: &u8) -> bool {
    // If sand is at the deepest level
    if sand_coord.1 >= *deepest_y {
        return false;
    }

    // Sand fall rules
    // Down
    if !map.contains(&(sand_coord.0, sand_coord.1 + 1)) {
        // Update sand coord
        sand_coord.1 += 1;

        // Drop sand on next step until it rests or is at the deepest level
        drop_sand(map, sand_coord, deepest_y)
    }
    // Down left
    else if !map.contains(&(sand_coord.0 - 1, sand_coord.1 + 1)) {
        // Update sand coord
        sand_coord.0 -= 1;
        sand_coord.1 += 1;

        // Drop sand on next step until it rests or is at the deepest level
        drop_sand(map, sand_coord, deepest_y)
    }
    // Down right
    else if !map.contains(&(sand_coord.0 + 1, sand_coord.1 + 1)) {
        // Update sand coord
        sand_coord.0 += 1;
        sand_coord.1 += 1;

        // Drop sand on next step until it rests or is at the deepest level
        drop_sand(map, sand_coord, deepest_y)
    } else {
        // Add sand to map if can't fall anymore
        map.insert(sand_coord)
    }
}

// Sand spawn location
const SAND_SPAWN: (u16, u8) = (500, 0);

pub fn part_one(input: &str) -> Option<u16> {
    // Generate the Set of points on the map
    let (_, ranges): (&str, Vec<Vec<(u16, u8)>>) = parse_ranges(input).unwrap();
    let mut map: HashSet<(u16, u8)> = generate_map(ranges);

    // Deepest y level
    let deepest_level: u8 = get_deepest_level(&map);
    // Track number of sands placed
    let mut sand_count: u16 = 0;

    // Drop sand until it reaches deepest level
    while drop_sand(&mut map, SAND_SPAWN, &deepest_level) {
        sand_count += 1;
    }

    Some(sand_count)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), None);
    }
}
