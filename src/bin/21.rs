use std::collections::HashMap;

// Have string be tuple of chars bc of Copy issues
#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Variable {
    name: (char, char, char, char),
}

impl Variable {
    fn to_variable(input: &str) -> Variable {
        let name: Vec<char> = input.chars().collect();
        Variable {
            name: (name[0], name[1], name[2], name[3]),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Yell {
    Value(f64),
    Operation((Variable, bool), char, (Variable, bool)),
}

fn parse_input(input: &str, yells: &mut HashMap<Variable, Yell>) {
    for line in input.lines() {
        let line_split: Vec<&str> = line.split(": ").collect();
        let variable: Variable = Variable::to_variable(line_split[0]);

        let rhs_split: Vec<&str> = line_split[1].split(' ').collect();
        if rhs_split.len() == 1 {
            yells.insert(variable, Yell::Value(rhs_split[0].parse::<f64>().unwrap()));
        } else {
            yells.insert(
                variable,
                Yell::Operation(
                    (Variable::to_variable(rhs_split[0]), false),
                    rhs_split[1].chars().next().unwrap(),
                    (Variable::to_variable(rhs_split[2]), false),
                ),
            );
        }
    }
}

fn solve(lhs: f64, op: char, rhs: f64) -> f64 {
    match op {
        '*' => lhs * rhs,
        '/' => lhs / rhs,
        '+' => lhs + rhs,
        '-' => lhs - rhs,
        _ => panic!("invalid op"),
    }
}

fn evaluate(
    yells: &mut HashMap<Variable, Yell>,
    variable: &Variable,
    part_two: bool,
) -> Option<f64> {
    // Signal recursive call that branch contains humn if part two
    if part_two && variable == &Variable::to_variable("humn") {
        return None;
    }

    // Recursively evaluate AST
    let yell: Yell = yells.get(variable).copied().unwrap();
    match yell {
        Yell::Value(value) => Some(value),
        Yell::Operation((lhs, _), op, (rhs, _)) => {
            let lhs_yell: Option<f64> = evaluate(yells, &lhs, part_two);
            let rhs_yell: Option<f64> = evaluate(yells, &rhs, part_two);

            // If evaluate if doing part one
            if let (Some(lhs_val), Some(rhs_val)) = (lhs_yell, rhs_yell) {
                Some(solve(lhs_val, op, rhs_val))
            } else {
                // For part two, don't evaluate branch if it contains humn. Set true to denote branch contains humn.
                if lhs_yell.is_none() {
                    *yells.get_mut(variable).unwrap() =
                        Yell::Operation((lhs, true), op, (rhs, false));
                } else {
                    *yells.get_mut(variable).unwrap() =
                        Yell::Operation((lhs, false), op, (rhs, true));
                }
                None
            }
        }
    }
}

fn reverse_op(op: char) -> char {
    match op {
        '*' => '/',
        '/' => '*',
        '+' => '-',
        '-' => '+',
        _ => panic!("invalid op"),
    }
}

fn eval_part_two(
    mut root_val: f64,
    yells: &mut HashMap<Variable, Yell>,
    variable: &Variable,
) -> f64 {
    // Recursively eval branch that does not contain humn, and perform reverse operation on root.
    // Then go down branch that does contains humn and repeat.
    let yell: Yell = yells.get(variable).copied().unwrap();
    match yell {
        Yell::Value(_) => root_val,
        Yell::Operation(lhs, op, rhs) => {
            let (has_humn, no_humn): (&Variable, &Variable) = if lhs.1 {
                (&lhs.0, &rhs.0)
            } else {
                (&rhs.0, &lhs.0)
            };
            let no_humn_val: f64 = evaluate(yells, no_humn, true).unwrap();

            // Make sure solving for order
            if (op == '/' || op == '-') && rhs.1 {
                root_val = solve(no_humn_val, op, root_val);
            } else {
                root_val = solve(root_val, reverse_op(op), no_humn_val);
            }

            eval_part_two(root_val, yells, has_humn)
        }
    }
}

pub fn part_one(input: &str) -> Option<f64> {
    let mut yells: HashMap<Variable, Yell> = HashMap::new();
    parse_input(input, &mut yells);
    evaluate(&mut yells, &Variable::to_variable("root"), false)
}

pub fn part_two(input: &str) -> Option<f64> {
    let mut yells: HashMap<Variable, Yell> = HashMap::new();
    parse_input(input, &mut yells);

    let root: Variable = Variable::to_variable("root");
    if let Yell::Operation((lhs, _), _, (rhs, _)) = yells.get(&root).copied().unwrap() {
        let lhs_yell = evaluate(&mut yells, &lhs, true);
        let rhs_yell = evaluate(&mut yells, &rhs, true);

        if let Some(root_val) = lhs_yell {
            Some(eval_part_two(root_val, &mut yells, &rhs))
        } else {
            Some(eval_part_two(rhs_yell.unwrap(), &mut yells, &lhs))
        }
    } else {
        panic!("root is not an operation");
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152_f64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301_f64));
    }
}
