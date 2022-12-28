use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    multi::separated_list1,
    sequence::delimited,
    IResult, Parser,
};

#[derive(Eq, PartialEq, Debug)]
struct Valve {
    name: String,
    rate: u32,
    tunnels_to: Vec<String>,
}

#[derive(Eq, PartialEq, Hash)]
struct ValveState {
    current_valve: usize,
    opened_valves: u16,
    time_left: u8,
}

fn parse_valve(input: &str) -> IResult<&str, Valve> {
    let (input, name) = delimited(tag("Valve "), alpha1, tag(" has flow "))(input)?;
    let (input, rate) = delimited(
        tag("rate="),
        complete::u32,
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
    )(input)?;
    let (input, tunnels) = separated_list1(tag(", "), alpha1.map(String::from))(input)?;

    Ok((
        input,
        Valve {
            name: name.to_string(),
            rate,
            tunnels_to: tunnels.iter().map(String::from).collect(),
        },
    ))
}

fn parse_network(input: &str) -> IResult<&str, Vec<Valve>> {
    let (input, valves) = separated_list1(newline, parse_valve)(input)?;

    Ok((input, valves))
}

// Calculate min distance from every pair of valves
fn floyd_warshall(valves: &[Valve], valve_indicies: &HashMap<&str, usize>) -> Vec<Vec<u8>> {
    let mut m: Vec<Vec<u8>> = vec![vec![u8::MAX; valves.len()]; valve_indicies.len()];
    for (i, valve) in valves.iter().enumerate() {
        m[i][i] = 0;
        for to_valve in &valve.tunnels_to {
            let to_idx: usize = *valve_indicies.get(to_valve as &str).unwrap();
            m[i][to_idx] = 1;
        }
    }

    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                if m[i][k] != u8::MAX && m[k][j] != u8::MAX && m[i][k] + m[k][j] < m[i][j] {
                    m[i][j] = m[i][k] + m[k][j];
                }
            }
        }
    }

    m
}

// traveling salesman
fn traveling_salesman(
    valves_info: &[Valve],
    paths: &[Vec<u8>],
    current_valve: usize,
    opened_valves: u16,
    time_left: u8,
    cache_states: &mut HashMap<ValveState, u32>,
) -> u32 {
    // Track max pressure
    let mut max_pressure: u32 = 0;

    // Iterate over valves
    for (to_valve, valve) in valves_info.iter().enumerate() {
        // Check if valve has already been visited
        let valve_bit_rep: u16 = 1 << to_valve;
        if opened_valves & valve_bit_rep != 0 {
            continue;
        }

        // Check if there is enough time to go to valve and open
        let time_taken: u8 = paths[current_valve][to_valve] + 1;
        if time_left < time_taken {
            continue;
        }

        // Calculate time remaining
        let time_remaining: u8 = time_left - time_taken;
        // Calculate accumulated pressure from time remaining
        let cum_pressure: u32 = valve.rate * time_remaining as u32;

        // Open valve bit mask
        let new_opened: u16 = opened_valves | valve_bit_rep;

        // Record valve state
        let valve_state: ValveState = ValveState {
            current_valve: to_valve,
            opened_valves: new_opened,
            time_left: time_remaining,
        };

        // writing down notes if i ever look back on this; idk if this explanation is right tho bc this still feels a bit weird
        // cache stores state of valve traversal - key: (current valve, opened valves, time remaining), value: max pressure
        // when state happens to be in cache, we can retrieve the max pressure at that point without having to calculate rest of the sequence
        // example: A -> B -> C -> D (current state) -> E -> F
        //          A -> B -> C -> D (current state) -> F -> E
        //          A -> C -> B -> D (current state) -> E -> F
        // don't have to calculate rest of the sequence {D, F} IF the state is the same

        // If the state of the valve is in cache, retrieve the max pressure at that state; otherwise, calculate it
        if let Some(cached_pressure) = cache_states.get(&valve_state) {
            if *cached_pressure > max_pressure {
                max_pressure = *cached_pressure;
                // Cache max pressure at the state
                cache_states.insert(valve_state, max_pressure);
            }
        } else {
            // Calculate rest of sequence
            let pressure: u32 = traveling_salesman(
                valves_info,
                paths,
                to_valve,
                new_opened,
                time_remaining,
                cache_states,
            );

            if pressure + cum_pressure > max_pressure {
                max_pressure = pressure + cum_pressure;
                // Cache max pressure at the state
                cache_states.insert(valve_state, max_pressure);
            }
        }
    }

    max_pressure
}

pub fn part_one(input: &str) -> Option<u32> {
    // Parse valves
    let (_, mut valves): (&str, Vec<Valve>) = parse_network(input).unwrap();
    const TIME_LIMIT: u8 = 30;

    // Get indicies of valves and relevant valves
    let mut valve_indicies: HashMap<&str, usize> = HashMap::with_capacity(valves.len());
    let mut irrelevant_valves: Vec<usize> = Vec::new();
    for (i, valve) in valves.iter().enumerate() {
        valve_indicies.insert(&valve.name, i);
        if valve.name != "AA" && valve.rate == 0 {
            irrelevant_valves.push(i);
        }
    }
    // Get shortest paths from all pairs of nodes
    let mut condensed_paths: Vec<Vec<u8>> = floyd_warshall(&valves, &valve_indicies);

    // Reduce valves to only relevant valves
    for i in irrelevant_valves.iter().rev() {
        valves.remove(*i);
        for path in condensed_paths.iter_mut().rev() {
            path.remove(*i);
        }
        condensed_paths.remove(*i);
    }

    // Get starting position
    let (start_valve, _): (usize, &Valve) = valves
        .iter()
        .enumerate()
        .find(|(_, v)| v.name == "AA")
        .unwrap();
    // Find max pressure
    let max_pressure: u32 = traveling_salesman(
        &valves,
        &condensed_paths,
        start_valve,
        1 << start_valve,
        TIME_LIMIT,
        &mut HashMap::new(),
    );

    Some(max_pressure)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Parse valves
    let (_, mut valves): (&str, Vec<Valve>) = parse_network(input).unwrap();
    const TIME_LIMIT: u8 = 26;

    // Get indicies of valves and relevant valves
    let mut valve_indicies: HashMap<&str, usize> = HashMap::with_capacity(valves.len());
    let mut irrelevant_valves: Vec<usize> = Vec::new();
    for (i, valve) in valves.iter().enumerate() {
        valve_indicies.insert(&valve.name, i);
        if valve.name != "AA" && valve.rate == 0 {
            irrelevant_valves.push(i);
        }
    }
    // Get shortest paths from all pairs of nodes
    let mut condensed_paths: Vec<Vec<u8>> = floyd_warshall(&valves, &valve_indicies);

    // Reduce valves to only relevant valves
    for i in irrelevant_valves.iter().rev() {
        valves.remove(*i);
        for path in condensed_paths.iter_mut().rev() {
            path.remove(*i);
        }
        condensed_paths.remove(*i);
    }

    // Get starting position
    let (start_valve, _): (usize, &Valve) = valves
        .iter()
        .enumerate()
        .find(|(_, v)| v.name == "AA")
        .unwrap();

    // Cache
    let mut cache_states: HashMap<ValveState, u32> = HashMap::new();

    // thx hyper-neutrino! https://www.youtube.com/watch?v=bLMj50cpOug <3
    // Iterate over every possible partitions
    let max_partitions: u16 = ((1 << valves.len()) as u32 - 1) as u16;
    let mut max_pressure: u32 = 0;
    for i in 1..(max_partitions / 2) {
        let me: u32 = traveling_salesman(
            &valves,
            &condensed_paths,
            start_valve,
            i,
            TIME_LIMIT,
            &mut cache_states,
        );
        let elephant: u32 = traveling_salesman(
            &valves,
            &condensed_paths,
            start_valve,
            max_partitions ^ i,
            TIME_LIMIT,
            &mut cache_states,
        );
        max_pressure = max_pressure.max(me + elephant);
    }

    Some(max_pressure)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
