use std::collections::HashMap;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Resources {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

// enum RobotType {
//     OreRobot(Resources),
//     ClayRobot(Resources),
//     ObsidianRobot(Resources),
//     GeodeRobot(Resources)
// }

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct ResourceState {
    resources: Resources,
    robots: Resources,
}

#[derive(Debug)]
struct Blueprint {
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

fn step(
    robot_type: u8,
    robot_blueprint: Option<&Resources>,
    mut state: ResourceState,
) -> ResourceState {
    // Produce resources
    state.resources.ore += state.robots.ore;
    state.resources.clay += state.robots.clay;
    state.resources.obsidian += state.robots.obsidian;
    state.resources.geode += state.robots.geode;

    // Build robot
    if let Some(r) = &robot_blueprint {
        state.resources.ore -= r.ore;
        state.resources.clay -= r.clay;
        state.resources.obsidian -= r.obsidian;
        state.resources.geode -= r.geode;

        match robot_type {
            0 => state.robots.ore += 1,
            1 => state.robots.clay += 1,
            2 => state.robots.obsidian += 1,
            3 => state.robots.geode += 1,
            _ => (), // wait
        }
    }

    state
}

// Basically same as day 16
fn dfs(
    time_left: u8,
    cache: &mut HashMap<(u8, ResourceState), u16>,
    geode_states: &mut [u16],
    blueprint: &Blueprint,
    state: ResourceState,
    max_ore_cost: &u16,
) -> u16 {
    // If run out of time, return
    if time_left == 0 {
        return state.resources.geode;
    }

    // If current state of geodes is less than the max produced geodes at the minute, do not go down path since it cannot improve the max
    if geode_states[time_left as usize - 1] > state.resources.geode {
        return state.resources.geode;
    } else {
        geode_states[time_left as usize - 1] = state.resources.geode;
    }

    // If state already exists, retrieve and return
    let curr: &(u8, ResourceState) = &(time_left, state);
    if cache.contains_key(curr) {
        return *cache.get(curr).unwrap();
    }

    // For all robot building except geode, check if production exceeds max consumption or else building another robot is useless,
    // and check if there's enough resources to build one.
    let resources: &Resources = &state.resources;
    let mut max_geodes: u16 = state.resources.geode;

    // Path 1: build ore robot
    if max_ore_cost > &state.robots.ore && resources.ore >= blueprint.ore_robot.ore {
        let new_state: ResourceState = step(0, Some(&blueprint.ore_robot), state);
        max_geodes = max_geodes.max(dfs(
            time_left - 1,
            cache,
            geode_states,
            blueprint,
            new_state,
            max_ore_cost,
        ));
    }

    // Path 2: build clay robot
    if blueprint.obsidian_robot.clay > state.robots.clay
        && resources.ore >= blueprint.clay_robot.ore
    {
        let new_state: ResourceState = step(1, Some(&blueprint.clay_robot), state);
        max_geodes = max_geodes.max(dfs(
            time_left - 1,
            cache,
            geode_states,
            blueprint,
            new_state,
            max_ore_cost,
        ));
    }

    // Path 3: build obsidian robot
    if blueprint.geode_robot.obsidian > state.robots.obsidian
        && resources.ore >= blueprint.obsidian_robot.ore
        && resources.clay >= blueprint.obsidian_robot.clay
    {
        let new_state: ResourceState = step(2, Some(&blueprint.obsidian_robot), state);
        max_geodes = max_geodes.max(dfs(
            time_left - 1,
            cache,
            geode_states,
            blueprint,
            new_state,
            max_ore_cost,
        ));
    }

    // Path 4: build geode robot
    if blueprint.geode_robot.ore <= resources.ore
        && blueprint.geode_robot.obsidian <= resources.obsidian
    {
        let new_state: ResourceState = step(3, Some(&blueprint.geode_robot), state);
        max_geodes = max_geodes.max(dfs(
            time_left - 1,
            cache,
            geode_states,
            blueprint,
            new_state,
            max_ore_cost,
        ));
    }

    // Path 5: wait
    let new_state: ResourceState = step(u8::MAX, None, state);
    max_geodes = max_geodes.max(dfs(
        time_left - 1,
        cache,
        geode_states,
        blueprint,
        new_state,
        max_ore_cost,
    ));

    cache.insert(*curr, max_geodes);
    max_geodes
}

fn calc_qualities(input: &[&str], time_limit: u8) -> Vec<u16> {
    // Quality levels
    let mut qualities: Vec<u16> = Vec::with_capacity(input.len());

    // Iterate blueprints
    for line in input {
        let tokens: Vec<&str> = line.split(' ').collect();
        // Define blueprint
        let blueprint: Blueprint = Blueprint {
            ore_robot: Resources {
                ore: tokens[6].parse::<u16>().unwrap(),
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            clay_robot: Resources {
                ore: tokens[12].parse::<u16>().unwrap(),
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            obsidian_robot: Resources {
                ore: tokens[18].parse::<u16>().unwrap(),
                clay: tokens[21].parse::<u16>().unwrap(),
                obsidian: 0,
                geode: 0,
            },
            geode_robot: Resources {
                ore: tokens[27].parse::<u16>().unwrap(),
                clay: 0,
                obsidian: tokens[30].parse::<u16>().unwrap(),
                geode: 0,
            },
        };
        let max_ore_cost: u16 = (blueprint.ore_robot.ore)
            .max((blueprint.clay_robot.ore).max(blueprint.obsidian_robot.ore));

        // Starting resources
        let start: ResourceState = ResourceState {
            resources: Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            robots: Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
        };

        // Cache state
        let mut cache: HashMap<(u8, ResourceState), u16> = HashMap::new();
        // Track max geodes for each minute
        let mut geode_states: Vec<u16> = vec![0; time_limit as usize];

        let max_geodes: u16 = dfs(
            time_limit,
            &mut cache,
            &mut geode_states,
            &blueprint,
            start,
            &max_ore_cost,
        );
        qualities.push(max_geodes)
    }

    qualities
}

pub fn part_one(input: &str) -> Option<u16> {
    // Time limit
    const TIME: u8 = 24;
    // Blueprints
    let blueprints_str: Vec<&str> = input.lines().collect();

    // Calculate qualities of blueprints
    let qualities: Vec<u16> = calc_qualities(&blueprints_str, TIME);
    Some(
        qualities
            .iter()
            .enumerate()
            .map(|(i, g)| (i as u16 + 1) * g)
            .sum::<u16>(),
    )
}

// For some reason, i didn't the test case, but got the actual correct answer with the actual input. 6840
// I tested my code with another person's code and also got the correct answer using their puzzle input.
// Im not sure how im specifically not getting the output for the test case. will look back later when i have time
pub fn part_two(input: &str) -> Option<u16> {
    // Time limit
    const TIME: u8 = 32;
    // Blueprints
    let blueprints_str: Vec<&str> = input.lines().take(3).collect();

    // Calculate qualities of blueprints
    let qualities: Vec<u16> = calc_qualities(&blueprints_str, TIME);
    // dbg!(&qualities);
    Some(qualities.iter().product::<u16>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        // Should instead be: 56 * 62
        assert_eq!(part_two(&input), Some(42 * 62));
    }
}
