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

fn sig_p1(sum_signals: &mut i32, x: &i32, interval: &mut u32) {
    // Accumulate signal strength
    *sum_signals += x * (*interval as i32);
    // Next interval to check
    *interval += 40;
}

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
        // Leave early after obtaining signals of interest
        if interval <= 220 {
            // Match instruction
            match i {
                InstructionType::Noop => {
                    // Increment cycle
                    cycles += 1;

                    // Obtain signal of interest
                    if cycles == interval {
                        sig_p1(&mut sum_signals, &x, &mut interval);
                    }
                }
                InstructionType::Addx(n) => {
                    // Increment cycle
                    cycles += 2;

                    // Obtain signal of interest
                    if cycles >= interval {
                        sig_p1(&mut sum_signals, &x, &mut interval);
                    }

                    // Add X register
                    x += n;
                }
            }
        } else {
            break;
        }
    }

    Some(sum_signals)
}

// ----------------------------------------------------------------------------

// Write pixel and increment a cycle
fn pixel_cycle(x: i32, cycles: &mut usize, row_pixels: &mut [char; 40]) {
    // Check if X is on pixel cycle
    if ((x - 1)..=(x + 1)).contains(&(*cycles as i32)) {
        row_pixels[*cycles] = '#';
    }

    // Increment cycle
    *cycles += 1;
}

// Print pixels; reset row and cycle
fn print_reset_cycle(cycles: &mut usize, row_pixels: &mut [char; 40]) {
    // Reset cycles
    *cycles = 0;
    // Print pixels and reset
    println!("{}", String::from_iter(*row_pixels));
    *row_pixels = ['.'; 40];
}

pub fn part_two(input: &str) -> Option<String> {
    // X register
    let mut x: i32 = 1;
    // Number of cycles
    let mut cycles: usize = 0;

    // List of instructions
    let (_, instr_vec): (&str, Vec<InstructionType>) = parse_instr_list(input).unwrap();

    // String builder
    let mut row_pixels: [char; 40] = ['.'; 40];

    for i in instr_vec {
        // Match instruction
        match i {
            InstructionType::Noop => {
                // One pixel cycle
                pixel_cycle(x, &mut cycles, &mut row_pixels);

                // Print pixels and reset row
                if cycles == 40 {
                    print_reset_cycle(&mut cycles, &mut row_pixels);
                }
            }
            InstructionType::Addx(n) => {
                // Number of cycles to perform
                for i in 0..2 {
                    // One pixel cycle
                    pixel_cycle(x, &mut cycles, &mut row_pixels);

                    // If cycle is completed, add X register
                    if i == 1 {
                        x += n;
                    }

                    // Print pixels and reset row
                    if cycles == 40 {
                        print_reset_cycle(&mut cycles, &mut row_pixels);
                    }
                }
            }
        }
    }

    Some(String::from("End of Output"))
}

// ----------------------------------------------------------------------------

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
        assert_eq!(part_two(&input), Some(String::from("End of Output")));
    }
}
