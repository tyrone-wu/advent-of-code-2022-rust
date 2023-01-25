fn decimal_to_snafu(mut decimal: u64) -> String {
    let mut snafu_bfive: Vec<u8> = Vec::new();
    while decimal != 0 {
        snafu_bfive.push((decimal % 5) as u8);
        decimal /= 5;
    }

    // example: 4890 base10 -> 124030 base5
    // 124030 -> 1241=0 -> 12-1=0 -> 13-1=0 -> 2=-1=0
    // right to left; if greater than 2, wrap around to -2, -1 and increment next digit; otherwise, keep
    let mut snafu: Vec<char> = Vec::with_capacity(snafu_bfive.len() * 2);
    let mut carry: u8 = 0;
    for d in snafu_bfive.iter_mut() {
        // Add the carry from prev snafu digit
        *d += carry;

        carry = u8::from(*d >= 3);
        match d {
            (0..=2) => {
                snafu.push((*d + b'0') as char);
            }
            3 => {
                snafu.push('=');
            }
            4 => {
                snafu.push('-');
            }
            _ => {
                snafu.push(((*d % 5) + b'0') as char);
            }
        }
    }
    // Edge case
    if carry > 0 {
        snafu.push((carry + b'0') as char);
    }

    snafu.iter().rev().collect()
}

fn snafu_to_decimal(snafu: &str) -> u64 {
    let mut decimal: i64 = 0;
    for (place, snafu_digit) in snafu.chars().rev().enumerate() {
        let digit: i64 = match snafu_digit {
            snafu_digit if (['0', '1', '2']).contains(&snafu_digit) => {
                (snafu_digit as u8 - b'0') as i64
            }
            '-' => -1,
            '=' => -2,
            _ => panic!("invalid snafu digit: {:?}", snafu_digit),
        };
        decimal += digit * 5_i64.pow(place as u32);
    }
    decimal as u64
}

pub fn part_one(input: &str) -> Option<String> {
    let sum: u64 = input.lines().map(snafu_to_decimal).sum::<u64>();
    Some(decimal_to_snafu(sum))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some(String::from("2=-1=0")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
