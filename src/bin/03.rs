// Compartment in a rucksack
struct Compartment {
    inventory: [bool; 52]
}

// Methods of Compartment
impl Compartment {

    // Get index of item
    pub fn get_index(item: u8) -> usize {
        match item {
            65..=90 => (item - 65 + 26) as usize,       // Upper case
            97..=122 => (item - 97) as usize,           // Lower case
            _ => panic!("Invalid item."),
        }
    }

    // Insert item into inventory
    fn insert(&mut self, item: u8) {
        // Index to insert in
        let item_idx: usize = Compartment::get_index(item);
        self.inventory[item_idx] = true;
    }

    // Does item exist in inventory
    fn exists(&self, item: u8) -> bool {
        // Index to check
        let item_idx: usize = Compartment::get_index(item);
        self.inventory[item_idx]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    
    // Priority sum
    let mut priority_sum: u32 = 0;
    
    // Iterate lines
    for line in input.lines() {
        // Line as bytes for indexing
        let line_bytes: &[u8] = line.as_bytes();
        // Rucksack bound
        let bound: usize = line.len() / 2;

        // First compartment
        let mut compart_one: Compartment = Compartment {
            inventory: [false; 52]
        };

        // Iterate and store into first compartment
        #[allow(clippy::needless_range_loop)]
        for i in 0..bound {
            compart_one.insert(line_bytes[i]);
        }

        // Iterate second compartment
        #[allow(clippy::needless_range_loop)]
        for i in bound..line.len() {
            if compart_one.exists(line_bytes[i]) {
                priority_sum += (Compartment::get_index(line_bytes[i]) as u32) + 1;
                break;
            }
        }
    }

    Some(priority_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
