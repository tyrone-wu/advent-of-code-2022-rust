use std::collections::{HashSet, VecDeque};

#[derive(Eq, Hash, PartialEq, Clone)]
struct MapState {
    x: u8,
    y: u8,
    map: Vec<Vec<u8>>,
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    match c {
                        '.' => 0,      // empty
                        '#' => 1 << 0, // wall
                        '^' => 1 << 1, // north
                        'v' => 1 << 2, // south
                        '>' => 1 << 3, // east
                        '<' => 1 << 4, // west
                        _ => panic!("invalid map: {:?}", c),
                    }
                })
                .collect()
        })
        .collect()
}

fn increment_map(map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    // Write moves onto new map bc
    let mut new_map: Vec<Vec<u8>> = vec![vec![0; map[0].len()]; map.len()];
    let y_end: usize = map.len() - 2;
    let x_end: usize = map[0].len() - 2;
    for (y, row) in map.iter().enumerate() {
        for (x, state) in row.iter().enumerate() {
            // wall or empty; otherwise blizzard
            if *state == 0 || *state == 1 {
                new_map[y][x] |= state;
            } else {
                // north
                if 2 & state != 0 {
                    let new_y: usize = if y == 1 { y_end } else { y - 1 };
                    new_map[new_y][x] |= 2;
                }
                // south
                if 4 & state != 0 {
                    let new_y: usize = if y == y_end { 1 } else { y + 1 };
                    new_map[new_y][x] |= 4;
                }
                // east
                if 8 & state != 0 {
                    let new_x: usize = if x == x_end { 1 } else { x + 1 };
                    new_map[y][new_x] |= 8;
                }
                // west
                if 16 & state != 0 {
                    let new_x: usize = if x == 1 { x_end } else { x - 1 };
                    new_map[y][new_x] |= 16;
                }
            }
        }
    }

    new_map
}

fn path_find(
    initial_map: Vec<Vec<u8>>,
    start: (u8, u8),
    goal: (u8, u8),
) -> Option<(u16, MapState)> {
    let y_end: u8 = initial_map.len() as u8 - 2;
    let x_end: u8 = initial_map[0].len() as u8 - 2;

    // cache states that we've already seen so that we don't run into loop
    let mut cache: HashSet<MapState> = HashSet::new();
    // bfs path finding
    let mut bfs: VecDeque<(u16, MapState)> = VecDeque::new();
    bfs.push_back((
        0,
        MapState {
            x: start.0,
            y: start.1,
            map: initial_map,
        },
    ));
    while let Some((mut time, map_state)) = bfs.pop_front() {
        let map: &Vec<Vec<u8>> = &map_state.map;
        let x: u8 = map_state.x;
        let y: u8 = map_state.y;

        // If standing not standing in empty spot aka blizzard or wall, discard
        if map[y as usize][x as usize] != 0 {
            continue;
        }

        // Exit if reached goal
        if (x, y) == goal {
            return Some((time, map_state));
        }

        // If looping back to state, discard
        if cache.contains(&map_state) {
            continue;
        } else {
            cache.insert(map_state.clone());
        }

        // Increment state
        let new_map: Vec<Vec<u8>> = increment_map(map);
        time += 1;

        // move south
        if y <= y_end && new_map[y as usize + 1][x as usize] == 0 {
            bfs.push_back((
                time,
                MapState {
                    x,
                    y: y + 1,
                    map: new_map.to_vec(),
                },
            ));
        }
        // move east
        if x <= x_end && new_map[y as usize][x as usize + 1] == 0 {
            bfs.push_back((
                time,
                MapState {
                    x: x + 1,
                    y,
                    map: new_map.to_vec(),
                },
            ));
        }
        // move west
        if x != 0 && new_map[y as usize][x as usize - 1] == 0 {
            bfs.push_back((
                time,
                MapState {
                    x: x - 1,
                    y,
                    map: new_map.to_vec(),
                },
            ));
        }
        // move north
        if y != 0 && new_map[y as usize - 1][x as usize] == 0 {
            bfs.push_back((
                time,
                MapState {
                    x,
                    y: y - 1,
                    map: new_map.to_vec(),
                },
            ));
        }
        // stand still
        if new_map[y as usize][x as usize] == 0 {
            bfs.push_back((
                time,
                MapState {
                    x,
                    y,
                    map: new_map.to_vec(),
                },
            ));
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u16> {
    // 2d vec bit representation map
    let initial_map: Vec<Vec<u8>> = parse_input(input);
    // Get current and goal position
    let start: (u8, u8) = (
        initial_map[0].iter().position(|&c| c == 0).unwrap() as u8,
        0,
    );
    let goal: (u8, u8) = (
        initial_map[initial_map.len() - 1]
            .iter()
            .position(|&c| c == 0)
            .unwrap() as u8,
        initial_map.len() as u8 - 1,
    );

    let path_find: Option<(u16, MapState)> = path_find(initial_map, start, goal);
    if let Some((time, _)) = path_find {
        Some(time)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u16> {
    // 2d vec bit representation map
    let initial_map: Vec<Vec<u8>> = parse_input(input);
    // Get current and goal position
    let start: (u8, u8) = (
        initial_map[0].iter().position(|&c| c == 0).unwrap() as u8,
        0,
    );
    let goal: (u8, u8) = (
        initial_map[initial_map.len() - 1]
            .iter()
            .position(|&c| c == 0)
            .unwrap() as u8,
        initial_map.len() as u8 - 1,
    );

    // start -> goal -> start -> goal
    let mut total_time: u16 = 0;
    let (time, map): (u16, MapState) = path_find(initial_map, start, goal).unwrap();
    total_time += time;

    let (time, map): (u16, MapState) = path_find(map.map, goal, start).unwrap();
    total_time += time;

    let (time, _): (u16, MapState) = path_find(map.map, start, goal).unwrap();
    total_time += time;

    Some(total_time)
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
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
