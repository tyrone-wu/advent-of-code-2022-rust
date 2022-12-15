use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};

// ----------------------------------------------------------------------------

enum InstructionType {
    Noop,
    Addx(i32),
}

// Parse single instruction
fn parse_instr(input: &str) -> IResult<&str, InstructionType> {
    let (input, instr) = alt((
        tag("noop").map(|_| InstructionType::Noop),
        preceded(tag("addx "), complete::i32).map(InstructionType::Addx),
    ))(input)?;

    Ok((input, instr))
}

// Parse list of instruction
fn parse_instr_list(input: &str) -> IResult<&str, Vec<InstructionType>> {
    let (input, instr_vec) = separated_list1(newline, parse_instr)(input)?;

    Ok((input, instr_vec))
}

// ----------------------------------------------------------------------------

pub fn part_one(input: &str) -> Option<i32> {
    // X register
    let mut x: i32 = 1;
    // Number of cycles
    let mut cycles: u32 = 0;
    // Sum of signal strengths
    let mut sum_signals: i32 = 0;
    // Cycle interval
    let mut interval: u32 = 20;

    // List of instructions
    let (_, instr_vec): (&str, Vec<InstructionType>) = parse_instr_list(input).unwrap();

    for i in instr_vec {
        if interval <= 220 {
            // Match instruction
            match i {
                InstructionType::Noop => {
                    cycles += 1;

                    if cycles == interval {
                        // Accumulate signal strength
                        sum_signals += x * (interval as i32);
                        // Next interval to check
                        interval += 40;
                    }
                }
                InstructionType::Addx(n) => {
                    cycles += 2;

                    if cycles >= interval {
                        // Accumulate signal strength
                        sum_signals += x * (interval as i32);
                        // Next interval to check
                        interval += 40;
                    }

                    x += n;
                }
            }
        } else {
            break;
        }
    }

    Some(sum_signals)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
