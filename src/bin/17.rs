use std::collections::{HashMap, HashSet};

// Spawn rock with the coordinates
fn spawn_rock(tallest_level: u64, rock_type: u8) -> Vec<(u64, u64)> {
    let spawn: (u64, u64) = (2, tallest_level + 3);

    match rock_type {
        0 => {
            vec![
                (spawn.0 + 1, spawn.1 + 1),
                (spawn.0 + 2, spawn.1 + 1),
                (spawn.0 + 3, spawn.1 + 1),
                (spawn.0 + 4, spawn.1 + 1),
            ]
        }
        1 => {
            vec![
                (spawn.0 + 2, spawn.1 + 1),
                (spawn.0 + 2, spawn.1 + 2),
                (spawn.0 + 2, spawn.1 + 3),
                (spawn.0 + 1, spawn.1 + 2),
                (spawn.0 + 3, spawn.1 + 2),
            ]
        }
        2 => {
            vec![
                (spawn.0 + 1, spawn.1 + 1),
                (spawn.0 + 2, spawn.1 + 1),
                (spawn.0 + 3, spawn.1 + 1),
                (spawn.0 + 3, spawn.1 + 2),
                (spawn.0 + 3, spawn.1 + 3),
            ]
        }
        3 => {
            vec![
                (spawn.0 + 1, spawn.1 + 1),
                (spawn.0 + 1, spawn.1 + 2),
                (spawn.0 + 1, spawn.1 + 3),
                (spawn.0 + 1, spawn.1 + 4),
            ]
        }
        4 => {
            vec![
                (spawn.0 + 1, spawn.1 + 1),
                (spawn.0 + 1, spawn.1 + 2),
                (spawn.0 + 2, spawn.1 + 1),
                (spawn.0 + 2, spawn.1 + 2),
            ]
        }
        _ => panic!("idk again"),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // Width of chamber
    const WIDTH: u64 = 7;
    // Number of rocks to drop
    const ROCKS_TO_DROP: u16 = 2022;

    // Track rocks fallen
    let mut fallen_rocks: HashSet<(u64, u64)> = HashSet::new();
    // Tallest rock level
    let mut tallest_level: u64 = 0;

    // Track rev rock fallen
    let mut rock_type: u8 = 0;
    // If spawning new rock
    let mut rock: Vec<(u64, u64)> = spawn_rock(tallest_level, rock_type);
    let mut new_rock: bool = false;

    let input_vec: &[u8] = input.as_bytes();

    // Iterate over cycles
    let mut dropped_rocks: u16 = 0;
    let mut cycle: usize = 0;
    while dropped_rocks < ROCKS_TO_DROP {
        // Iterate cycle
        let c: u8 = input_vec[cycle];
        cycle = (cycle + 1) % (input_vec.len() - 1);

        // Jet of gas push rock horizontally
        let mut can_shift: bool = true;
        match c {
            b'<' => {
                // Check if rock can move left
                for (x, y) in rock.iter() {
                    if *x == 1 || fallen_rocks.contains(&(*x - 1, *y)) {
                        can_shift = false;
                        break;
                    }
                }

                if can_shift {
                    for (x, _) in rock.iter_mut() {
                        *x -= 1;
                    }
                }
            }
            b'>' => {
                // Check if rock can move left
                for (x, y) in rock.iter() {
                    if *x == WIDTH || fallen_rocks.contains(&(*x + 1, *y)) {
                        can_shift = false;
                        break;
                    }
                }

                if can_shift {
                    for (x, _) in rock.iter_mut() {
                        *x += 1;
                    }
                }
            }
            _ => panic!("idk"),
        }

        // Check if rock can drop
        for (x, y) in rock.iter() {
            if *y == 1 || fallen_rocks.contains(&(*x, *y - 1)) {
                new_rock = true;
                break;
            }
        }

        // Record rock to fallen rocks and spawn new rock; otherwise, drop rock 1 unit
        if new_rock {
            for coord in rock.iter() {
                // Record new tallest level
                if tallest_level < coord.1 {
                    tallest_level = coord.1;
                }
                fallen_rocks.insert(*coord);
            }
            dropped_rocks += 1;

            // Spawn new rock
            rock_type = (rock_type + 1) % 5;
            rock = spawn_rock(tallest_level, rock_type);
            new_rock = false;
        } else {
            // Drop rock by 1
            for (_, y) in rock.iter_mut() {
                *y -= 1;
            }
        }
    }

    Some(tallest_level)
}

// Width of chamber
const WIDTH: u64 = 7;

#[derive(Eq, Hash, PartialEq)]
struct RockState {
    rock_type: u8,
    jet_cycle: usize,
    top_view_rocks: [u64; WIDTH as usize],
}

struct RockStateRecord {
    num_rocks_dropped: u64,
    height: u64,
    times_seen: u8,
}

pub fn part_two(input: &str) -> Option<u64> {
    // Number of rocks to drop
    const ROCKS_TO_DROP: u64 = 1000000000000;

    // Track rocks fallen
    let mut fallen_rocks: HashSet<(u64, u64)> = HashSet::new();
    // Tallest rock level
    let mut tallest_level: u64 = 0;

    // Track rev rock fallen
    let mut rock_type: u8 = 0;
    // If spawning new rock
    let mut rock: Vec<(u64, u64)> = spawn_rock(tallest_level, rock_type);
    let mut new_rock: bool = false;

    let input_vec: &[u8] = input.as_bytes();

    // Iterate over cycles
    let mut dropped_rocks: u64 = 0;
    let mut cycle: usize = 0;

    // Track state
    let mut cached_states: HashMap<RockState, RockStateRecord> = HashMap::new();
    // The total height of the repeated cycle
    let mut repeated_total_height: u64 = 0;

    while dropped_rocks < ROCKS_TO_DROP {
        // Iterate cycle
        let c: u8 = input_vec[cycle];
        cycle = (cycle + 1) % (input_vec.len() - 1);

        // Jet of gas push rock horizontally
        let mut can_shift: bool = true;
        match c {
            b'<' => {
                // Check if rock can move left
                for (x, y) in rock.iter() {
                    if *x == 1 || fallen_rocks.contains(&(*x - 1, *y)) {
                        can_shift = false;
                        break;
                    }
                }

                if can_shift {
                    for (x, _) in rock.iter_mut() {
                        *x -= 1;
                    }
                }
            }
            b'>' => {
                // Check if rock can move left
                for (x, y) in rock.iter() {
                    if *x == WIDTH || fallen_rocks.contains(&(*x + 1, *y)) {
                        can_shift = false;
                        break;
                    }
                }

                if can_shift {
                    for (x, _) in rock.iter_mut() {
                        *x += 1;
                    }
                }
            }
            _ => panic!("idk"),
        }

        // Check if rock can drop
        for (x, y) in rock.iter() {
            if *y == 1 || fallen_rocks.contains(&(*x, *y - 1)) {
                new_rock = true;
                break;
            }
        }

        // Record rock to fallen rocks and spawn new rock; otherwise, drop rock 1 unit
        if new_rock {
            for coord in rock.iter() {
                // Record new tallest level
                if tallest_level < coord.1 {
                    tallest_level = coord.1;
                }
                fallen_rocks.insert(*coord);
            }
            dropped_rocks += 1;

            // Spawn new rock
            rock_type = (rock_type + 1) % 5;
            rock = spawn_rock(tallest_level, rock_type);
            new_rock = false;

            // Get top view
            let mut top_view_arr: [u64; WIDTH as usize] = [tallest_level; WIDTH as usize];
            for (i, t) in top_view_arr.iter_mut().enumerate() {
                let max_y: u64 = fallen_rocks
                    .iter()
                    .filter(|(x, _)| *x == (i + 1) as u64)
                    .max_by(|(_, y_i), (_, y_j)| y_i.cmp(y_j))
                    .unwrap_or(&(0, u64::MAX))
                    .1;
                *t = if max_y == u64::MAX {
                    u64::MAX
                } else {
                    *t - max_y
                };
            }

            // Current state
            let state: RockState = RockState {
                rock_type,
                jet_cycle: cycle,
                top_view_rocks: top_view_arr,
            };

            // If state is cached, retrieve number of rocks dropped and tallest height at that state and "fast forward" the iterations; otherwise, record state
            if let Some(recorded_state) = cached_states.get_mut(&state) {
                // If the state has been seen more than once
                if recorded_state.times_seen > 1 {
                    // Number of rocks dropped in the repeated cycle
                    let repeated_num_rocks: u64 = dropped_rocks - recorded_state.num_rocks_dropped;
                    // The number of repeats to "fast forward"
                    let repeated_cycles: u64 =
                        (ROCKS_TO_DROP - dropped_rocks) / (repeated_num_rocks);
                    // Fast forward the iterations
                    dropped_rocks += repeated_num_rocks * repeated_cycles;

                    // Calculate the total height of the heights repeated
                    repeated_total_height =
                        (tallest_level - recorded_state.height) * repeated_cycles;

                    // println!("{:?} {:?} {:?} {:?}", dropped_rocks, repeated_cycle_height, tallest_level, height);

                    // Reset cache so that this entire condition only runs once. This should only need to run once anyways.
                    // Going through this condition a second time will cause repeated_cycles would be 0 in the second execution since the iterations already "fast fowarded" to the max,
                    // so there is no more repeats to calculate or perform.
                    cached_states = HashMap::new();
                } else {
                    // Update the recorded state
                    recorded_state.num_rocks_dropped = dropped_rocks;
                    recorded_state.height = tallest_level;
                    recorded_state.times_seen += 1;
                }
            } else {
                // Cache state
                cached_states.insert(
                    state,
                    RockStateRecord {
                        num_rocks_dropped: dropped_rocks,
                        height: tallest_level,
                        times_seen: 1,
                    },
                );
            }
        } else {
            // Drop rock by 1
            for (_, y) in rock.iter_mut() {
                *y -= 1;
            }
        }
    }

    Some(tallest_level + repeated_total_height)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
