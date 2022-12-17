use std::collections::{BinaryHeap, VecDeque};

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, alphanumeric1, newline, space1},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

//  Monkey operation
struct Operation {
    left: u64,
    op: char,
    right: u64,
}

// Monkey throw condition
struct Test {
    div_by: u64,
    true_throw: usize,
    false_throw: usize,
}

// stinky monke
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    inspect_count: u64,
}

impl Monkey {
    // Perform basic arithmetic operation
    fn perform_operation(left: u64, op: char, right: u64) -> u64 {
        match op {
            '+' => left + right,
            '-' => left - right,
            '*' => left * right,
            '/' => left / right,
            _ => panic!("idk this: {op}"),
        }
    }

    // Inspect item
    fn inspect(&mut self, relief: u64) -> u64 {
        // Inspect item
        let mut worry: u64 = self.items.pop_front().unwrap();

        let left: u64 = if self.operation.left == 0 {
            worry
        } else {
            self.operation.left
        };
        let right: u64 = if self.operation.right == 0 {
            worry
        } else {
            self.operation.right
        };
        worry = Monkey::perform_operation(left, self.operation.op, right);

        // Relief check
        worry / relief
    }

    // Receive item
    fn receive_item(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

// Monkey inspect and throw
fn inspect_throw_items(monkeys: &mut [Monkey], curr_idx: usize, relief: u64, reset: u64) {
    // Update inspect count
    monkeys[curr_idx].inspect_count += monkeys[curr_idx].items.len() as u64;

    // Inspect and throw all items
    while !monkeys[curr_idx].items.is_empty() {
        // New worry level
        let new: u64 = monkeys[curr_idx].inspect(relief) % reset;

        // Test condition of where to throw item
        let cond: &Test = &monkeys[curr_idx].test;
        if new % cond.div_by == 0 {
            monkeys[cond.true_throw].receive_item(new);
        } else {
            monkeys[cond.false_throw].receive_item(new);
        }
    }
}

// ----------------------------------------------------------------------------

// Parse starting items of monkey
fn parse_start_items(input: &str) -> IResult<&str, VecDeque<u64>> {
    let (input, items) = delimited(
        tag("  Starting items: "),                 // Discard matching text
        separated_list1(tag(", "), complete::u64), // Get space separated u64 as Vec
        newline,                                   // Discard newline
    )(input)?;

    Ok((input, VecDeque::from_iter(items)))
}

// Parse operation
fn parse_operation(input: &str) -> IResult<&str, Operation> {
    // Parse left side of operation
    let (input, left) = preceded(
        tag("  Operation: new = "), // Discard matching text
        alphanumeric1,              // Match alphanumeric
    )(input)?;

    // Parse operand
    let (input, op) = preceded(
        space1,            // Discard space
        complete::anychar, // Match character
    )(input)?;

    // Parse right side of operation
    let (input, right) = delimited(
        space1,        // Discard space
        alphanumeric1, // Match alphanumeric
        newline,       // Discard newline
    )(input)?;

    Ok((
        input,
        Operation {
            left: if left == "old" {
                0
            } else {
                left.parse::<u64>().unwrap()
            },
            op,
            right: if right == "old" {
                0
            } else {
                right.parse::<u64>().unwrap()
            },
        },
    ))
}

// Parse test
fn parse_test(input: &str) -> IResult<&str, Test> {
    // Parse divide by line
    let (input, div_by) = delimited(
        tag("  Test: divisible by "), // Discard text
        complete::u64,                // Match u64
        newline,                      // Discard newline
    )(input)?;

    // Parse true
    let (input, true_throw) = delimited(
        tag("    If true: throw to monkey "), // Discard text
        complete::u8,                         // Match usize
        newline,                              // Discard newline
    )(input)?;

    // Parse false
    let (input, false_throw) = delimited(
        tag("    If false: throw to monkey "), // Discard text
        complete::u8,                          // Match usize
        newline,                               // Discard newline
    )(input)?;

    Ok((
        input,
        Test {
            div_by,
            true_throw: true_throw as usize,
            false_throw: false_throw as usize,
        },
    ))
}

// Parse monkey
fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    // Discard first line
    let (input, _) = take_until("  Starting")(input)?;

    let (input, items) = parse_start_items(input)?;
    let (input, operation) = parse_operation(input)?;
    let (input, test) = parse_test(input)?;

    Ok((
        input,
        Monkey {
            items,
            operation,
            test,
            inspect_count: 0,
        },
    ))
}

// Parse list of monke
fn parse_monkey_list(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, monke_vec) = separated_list1(newline, parse_monkey)(input)?;

    Ok((input, monke_vec))
}

// ----------------------------------------------------------------------------

pub fn part_one(input: &str) -> Option<u64> {
    let (_, mut monkey_vec) = parse_monkey_list(input).unwrap();

    let mut reset: u64 = 1;
    for d in monkey_vec.iter() {
        reset *= d.test.div_by;
    }

    // 20 rounds
    for _ in 0..20 {
        // Each monke
        for m in 0..monkey_vec.len() {
            inspect_throw_items(&mut monkey_vec, m, 3, reset);
        }
    }

    let mut monkey_business: BinaryHeap<u64> = monkey_vec.iter().map(|m| m.inspect_count).collect();
    Some(monkey_business.pop().unwrap() * monkey_business.pop().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, mut monkey_vec) = parse_monkey_list(input).unwrap();

    let mut reset_mod: u64 = 1;
    for d in monkey_vec.iter() {
        reset_mod *= d.test.div_by;
    }

    // 10000 rounds
    for _ in 0..10000 {
        // Each monke
        for m in 0..monkey_vec.len() {
            inspect_throw_items(&mut monkey_vec, m, 1, reset_mod);
        }
    }

    let mut monkey_business: BinaryHeap<u64> = monkey_vec.iter().map(|m| m.inspect_count).collect();
    Some(monkey_business.pop().unwrap() * monkey_business.pop().unwrap())
}

// ----------------------------------------------------------------------------

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
