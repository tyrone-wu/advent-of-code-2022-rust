use std::collections::BTreeSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

// So that clippy doesn't give me type_complexity warning
struct Coordinate {
    x: i32,
    y: i32,
}

// Parse a sensor and a beacon
fn parse_sensor_beacon(input: &str) -> IResult<&str, (Coordinate, Coordinate)> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, s_coord) = separated_pair(complete::i32, tag(", y="), complete::i32)(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, b_coord) = separated_pair(complete::i32, tag(", y="), complete::i32)(input)?;

    Ok((
        input,
        (
            Coordinate {
                x: s_coord.0,
                y: s_coord.1,
            },
            Coordinate {
                x: b_coord.0,
                y: b_coord.1,
            },
        ),
    ))
}

// Parse sensors and their closest beacons
fn parse_sb_list(input: &str) -> IResult<&str, Vec<(Coordinate, Coordinate)>> {
    let (input, scans) = separated_list1(newline, parse_sensor_beacon)(input)?;

    Ok((input, scans))
}

pub fn part_one(input: &str) -> Option<u32> {
    // Extract sensor and beacon locations
    let (_, scans): (&str, Vec<(Coordinate, Coordinate)>) = parse_sb_list(input).unwrap();
    // Row to check
    const ROW_REPORT: i32 = 2000000;
    // Hold ranges that are in the row
    let mut ranges: BTreeSet<(i32, i32)> = BTreeSet::new();
    // Break points in the ranges
    let mut break_points: BTreeSet<i32> = BTreeSet::new();

    for (sensor, beacon) in scans {
        // Calculate Manhattan distance
        let x_dist: i32 = sensor.x - beacon.x;
        let y_dist: i32 = sensor.y - beacon.y;
        let manh_dist: i32 = (x_dist.abs() + y_dist.abs()) as i32;

        // If the sensors coverage crosses our row of interest
        if ((sensor.y - manh_dist)..=(sensor.y + manh_dist)).contains(&ROW_REPORT) {
            // Calculate range of the sensors coverage along the row
            let rad: i32 = manh_dist - (sensor.y).abs_diff(ROW_REPORT) as i32;
            let mut start: i32 = sensor.x - rad - 1;
            let mut end: i32 = sensor.x + rad;

            // If there is a sensor and/or beacon in the range
            if sensor.y == ROW_REPORT {
                break_points.insert(sensor.x);
            }
            if beacon.y == ROW_REPORT {
                break_points.insert(beacon.x);
            }

            // Merge the range so none of the ranges overlap
            ranges.retain(|&(r_start, r_end)| {
                // Whether to keep or remove the range to overwrite
                let mut retain: bool = true;

                // r_s |----| r_e
                //      s |----| e
                if (r_start..=r_end).contains(&start) && r_end < end {
                    start = r_start;
                    retain = false;
                }
                //  r_s |----| r_e
                // s |----| e
                else if (r_start..=r_end).contains(&end) && start < r_start {
                    end = r_end;
                    retain = false;
                }
                // r_s |---| r_e
                // s |-------| e
                else if start <= r_start && end >= r_end {
                    retain = false;
                }
                // r_s |-------| r_e
                //     s |---| e
                else if start > r_start && end < r_end {
                    start = r_start;
                    end = r_end;
                    retain = false;
                }

                retain
            });

            ranges.insert((start, end));
        }
    }

    // Calculate length of ranges minus the number of spots taken by sensor/beacon
    let spots_taken: u32 = ranges
        .iter()
        .map(|(start, end)| (end - start) as u32)
        .sum::<u32>()
        - break_points.len() as u32;

    Some(spots_taken)
}

// from rust discord: checked the boundaries of the diamonds optimized search
pub fn part_two(input: &str) -> Option<u64> {
    // Extract sensor and beacon locations
    let (_, scans): (&str, Vec<(Coordinate, Coordinate)>) = parse_sb_list(input).unwrap();
    // Max coordinate
    const MAX_COORD: u32 = 4000000;

    for i in 0..=MAX_COORD {
        // Hold ranges that are in the row
        let mut ranges: BTreeSet<(u32, u32)> = BTreeSet::new();

        for (sensor, beacon) in &scans {
            // Calculate Manhattan distance
            let x_dist: i32 = sensor.x - beacon.x;
            let y_dist: i32 = sensor.y - beacon.y;
            let manh_dist: i32 = (x_dist.abs() + y_dist.abs()) as i32;

            // If the sensors coverage crosses our row of interest
            let min_check: u32 = (sensor.y - manh_dist).max(0) as u32;
            let max_check: u32 = ((sensor.y + manh_dist) as u32).min(MAX_COORD);
            if (min_check..=max_check).contains(&i) {
                // Calculate range of the sensors coverage along the row
                let rad: i32 = manh_dist - (sensor.y).abs_diff(i as i32) as i32;
                let mut start: u32 = (sensor.x - rad - 1).max(0) as u32;
                let mut end: u32 = ((sensor.x + rad) as u32).min(MAX_COORD);

                // Merge the range so none of the ranges overlap
                ranges.retain(|&(r_start, r_end)| {
                    // Whether to keep or remove the range to overwrite
                    let mut retain: bool = true;

                    // If range needs to merge
                    if (r_start..=r_end).contains(&start) && r_end <= end {
                        start = r_start;
                        retain = false;
                    } else if (r_start..=r_end).contains(&end) && start <= r_start {
                        end = r_end;
                        retain = false;
                    } else if start <= r_start && end >= r_end {
                        retain = false;
                    } else if start > r_start && end < r_end {
                        start = r_start;
                        end = r_end;
                        retain = false;
                    }

                    retain
                });

                ranges.insert((start, end));
            }
        }

        // If there's an open spot in the ranges
        let r: (u32, u32) = *ranges.iter().next().unwrap();
        // Open spot is somewhere in middle
        if ranges.len() > 1 {
            let x: u32 = r.1 + 1;
            return Some((x as u64) * 4000000 + i as u64);
        }
        // Open spot is along the boundary
        else if r.0 != 0 || r.1 != MAX_COORD {
            let x: u32 = if r.0 != 0 { 0 } else { MAX_COORD };
            return Some((x as u64) * 4000000 + i as u64);
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_part_one() {
        let _input = advent_of_code::read_file("examples", 15);
        // assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let _input = advent_of_code::read_file("examples", 15);
        // assert_eq!(part_two(&input), Some(56000011));
    }
}
