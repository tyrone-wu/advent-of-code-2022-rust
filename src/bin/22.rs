use nom::{
    branch::alt,
    character::complete::{self, alpha1},
    multi::many1,
    IResult, Parser,
};

#[derive(Debug)]
enum Instruction {
    Steps(u32),
    Turn(char),
}

#[derive(Debug)]
struct Me {
    x: usize,
    y: usize,
    direction: i8,
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions): (&str, Vec<Instruction>) = many1(alt((
        (complete::u32).map(Instruction::Steps),
        alpha1.map(|c: &str| Instruction::Turn(c.chars().next().unwrap())),
    )))(input)?;

    Ok((input, instructions))
}

pub fn part_one(input: &str) -> Option<u32> {
    let split: Vec<&str> = input.split("\n\n").collect();
    // Parse map
    let mut map: Vec<Vec<char>> = split[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Get the horizontal bounds of the map
    let horizontal_bounds: Vec<(usize, usize)> = map
        .iter()
        .map(|h_line| {
            (
                h_line.iter().position(|&c| c != ' ').unwrap(),
                h_line.len() - 1,
            )
        })
        .collect();

    // Pad lines with space so it's a complete rectangle
    let max_hor: usize = map.iter().map(|h_line| h_line.len()).max().unwrap();
    for h_line in map.iter_mut() {
        let start: usize = h_line.len();
        for _ in start..max_hor {
            h_line.push(' ');
        }
    }
    // Get the vertical bounds of the  map
    let mut vertical_bounds: Vec<(usize, usize)> = Vec::with_capacity(map[0].len());
    for i in 0..map[0].len() {
        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;

        #[allow(clippy::needless_range_loop)]
        for j in 0..map.len() {
            if map[j][i] != ' ' {
                if start.is_none() {
                    start = Some(j);
                } else {
                    end = Some(j);
                }
            }
        }
        vertical_bounds.push((start.unwrap(), end.unwrap()));
    }

    let mut me: Me = Me {
        x: horizontal_bounds[0].0,
        y: 0,
        direction: 0,
    };

    // Parse instructions
    let (_, instructions): (&str, Vec<Instruction>) = parse_instructions(split[1]).unwrap();
    for instr in &instructions {
        match instr {
            Instruction::Steps(steps) => {
                for _ in 0..*steps {
                    match me.direction {
                        0 => {
                            let mut front: usize = me.x + 1;
                            if front > horizontal_bounds[me.y].1 {
                                front = horizontal_bounds[me.y].0;
                            }
                            if map[me.y][front] != '#' {
                                me.x = front;
                            }
                        }
                        1 => {
                            let mut front: usize = me.y + 1;
                            if front > vertical_bounds[me.x].1 {
                                front = vertical_bounds[me.x].0;
                            }
                            if map[front][me.x] != '#' {
                                me.y = front;
                            }
                        }
                        2 => {
                            let mut front: i16 = me.x as i16 - 1;
                            if front < horizontal_bounds[me.y].0 as i16 {
                                front = horizontal_bounds[me.y].1 as i16;
                            }
                            if map[me.y][front as usize] != '#' {
                                me.x = front as usize;
                            }
                        }
                        3 => {
                            let mut front: i16 = me.y as i16 - 1;
                            if front < vertical_bounds[me.x].0 as i16 {
                                front = vertical_bounds[me.x].1 as i16;
                            }
                            if map[front as usize][me.x] != '#' {
                                me.y = front as usize;
                            }
                        }
                        _ => panic!("invalid dir: {:?}", me.direction),
                    }
                }
            }
            Instruction::Turn(dir) => {
                match dir {
                    'L' => me.direction -= 1,
                    'R' => me.direction += 1,
                    _ => panic!("Invalid direction: {:?}", dir),
                }
                me.direction = me.direction.rem_euclid(4);
            }
        }
    }
    // dbg!(&me);

    Some(1000 * (me.y + 1) as u32 + 4 * (me.x + 1) as u32 + me.direction as u32)
}

// Too lazy to refactor part one
#[derive(Debug)]
struct MeCube {
    x: i16,
    y: i16,
    direction: i8,
}

pub fn part_two(input: &str) -> Option<u32> {
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut map: Vec<Vec<char>> = split[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    // Pad lines with space so it's a complete rectangle
    let max_hor: usize = map.iter().map(|h_line| h_line.len()).max().unwrap();
    for h_line in map.iter_mut() {
        let start: usize = h_line.len();
        for _ in start..max_hor {
            h_line.push(' ');
        }
    }

    let mut me: MeCube = MeCube {
        x: map[0].iter().position(|&c| c != ' ').unwrap() as i16,
        y: 0,
        direction: 0,
    };

    // Parse instructions
    let (_, instructions): (&str, Vec<Instruction>) = parse_instructions(split[1]).unwrap();
    for instr in &instructions {
        match instr {
            Instruction::Steps(steps) => {
                for _ in 0..*steps {
                    // New coord position and direction to track
                    let mut x_front: i16 = me.x;
                    let mut y_front: i16 = me.y;
                    let mut new_dir: i8 = me.direction;

                    // Stepping in direction; hard-coded wrapping rules :P
                    match me.direction {
                        0 => {
                            x_front += 1;
                            // x bound 149 to 150; only 1 case
                            if x_front == 150 && (0..50).contains(&me.y) {
                                x_front = 99;
                                y_front = 149 - me.y;
                                new_dir = 2;
                            }
                            // x bound 99 to 100; 2 cases
                            else if x_front == 100 {
                                if (100..150).contains(&me.y) {
                                    x_front = 149;
                                    y_front = 149 - me.y;
                                    new_dir = 2;
                                } else if (50..100).contains(&me.y) {
                                    x_front = me.y + 50;
                                    y_front = 49;
                                    new_dir = 3;
                                }
                            }
                            // x bound 49 to 50; only 1 case
                            else if x_front == 50 && (150..200).contains(&me.y) {
                                x_front = me.y - 100;
                                y_front = 149;
                                new_dir = 3;
                            }
                        }
                        1 => {
                            y_front += 1;
                            // y bound 199 to 200; only 1 case
                            if y_front == 200 && (0..50).contains(&me.x) {
                                x_front = me.x + 100;
                                y_front = 0;
                            }
                            // y bound 49 to 50; only 1 case
                            else if y_front == 50 && (100..150).contains(&me.x) {
                                x_front = 99;
                                y_front = me.x - 50;
                                new_dir = 2;
                            }
                            // y bound 149 to 150; only 1 case
                            else if y_front == 150 && (50..100).contains(&me.x) {
                                x_front = 49;
                                y_front = me.x + 100;
                                new_dir = 2;
                            }
                        }
                        2 => {
                            x_front -= 1;
                            // x bound 0 to -1; 2 cases
                            if x_front == -1 {
                                if (150..200).contains(&me.y) {
                                    x_front = me.y - 100;
                                    y_front = 0;
                                    new_dir = 1;
                                } else if (100..150).contains(&me.y) {
                                    x_front = 50;
                                    y_front = 149 - me.y;
                                    new_dir = 0;
                                }
                            }
                            // x bound 50 to 49; 2 cases
                            else if x_front == 49 {
                                if (50..100).contains(&me.y) {
                                    x_front = me.y - 50;
                                    y_front = 100;
                                    new_dir = 1;
                                } else if (0..50).contains(&me.y) {
                                    x_front = 0;
                                    y_front = 149 - me.y;
                                    new_dir = 0;
                                }
                            }
                        }
                        3 => {
                            y_front -= 1;
                            // y bound 0 to -1; 2 cases
                            if y_front == -1 {
                                if (50..100).contains(&me.x) {
                                    x_front = 0;
                                    y_front = me.x + 100;
                                    new_dir = 0;
                                } else if (100..150).contains(&me.x) {
                                    x_front = me.x - 100;
                                    y_front = 199;
                                }
                            }
                            // y bound 100 to 99; only 1 case
                            else if y_front == 99 && (0..50).contains(&me.x) {
                                x_front = 50;
                                y_front = me.x + 50;
                                new_dir = 0;
                            }
                        }
                        _ => panic!("invalid dir: {:?}", me.direction),
                    }

                    // If front is not blocked, take step
                    if map[y_front as usize][x_front as usize] == '.' {
                        me.x = x_front;
                        me.y = y_front;
                        me.direction = new_dir;
                    }
                }
            }
            Instruction::Turn(dir) => {
                match dir {
                    'L' => me.direction -= 1,
                    'R' => me.direction += 1,
                    _ => panic!("Invalid direction: {:?}", dir),
                }
                me.direction = me.direction.rem_euclid(4);
            }
        }
    }

    Some(1000 * (me.y + 1) as u32 + 4 * (me.x + 1) as u32 + me.direction as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let _input = advent_of_code::read_file("examples", 22);
        // assert_eq!(part_two(&input), Some(5031));
    }
}
