use std::{
    collections::{HashSet, VecDeque},
    ops::RangeInclusive,
};

const ADJACENT_OFFSET: [(i16, i16, i16); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

pub fn part_one(input: &str) -> Option<u16> {
    // Track surface area
    let mut surface_area: u16 = 0;
    // Record map of droplet coordinates
    let mut grid: HashSet<(i16, i16, i16)> = HashSet::new();

    // Parse droplets
    for line in input.lines() {
        let mut it = line.split(',');
        let x: i16 = it.next().unwrap().parse::<i16>().unwrap();
        let y: i16 = it.next().unwrap().parse::<i16>().unwrap();
        let z: i16 = it.next().unwrap().parse::<i16>().unwrap();

        // Area to add
        let mut added_surface: u16 = 6;

        // Check the 6 adjacent sides
        for (dx, dy, dz) in ADJACENT_OFFSET {
            // If adjacent to another droplet
            if grid.contains(&(x + dx, y + dy, z + dz)) {
                surface_area -= 1;
                added_surface -= 1;
            }
        }

        grid.insert((x, y, z));
        surface_area += added_surface;
    }

    Some(surface_area)
}

pub fn part_two(input: &str) -> Option<u16> {
    // Bounds of the droplets coordinates
    let mut x_bound: (i16, i16) = (i16::MAX, 0);
    let mut y_bound: (i16, i16) = (i16::MAX, 0);
    let mut z_bound: (i16, i16) = (i16::MAX, 0);

    // Droplet coordinates
    let grid: HashSet<(i16, i16, i16)> = input
        .lines()
        .map(|line| {
            let mut it = line.split(',');
            let x: i16 = it.next().unwrap().parse::<i16>().unwrap();
            let y: i16 = it.next().unwrap().parse::<i16>().unwrap();
            let z: i16 = it.next().unwrap().parse::<i16>().unwrap();

            x_bound = (x.min(x_bound.0), x.max(x_bound.1));
            y_bound = (y.min(y_bound.0), y.max(y_bound.1));
            z_bound = (z.min(z_bound.0), z.max(z_bound.1));

            (x, y, z)
        })
        .collect();
    // Expand box by 1 in case corner is filled
    let x_range: RangeInclusive<i16> = (x_bound.0 - 1)..=(x_bound.1 + 1);
    let y_range: RangeInclusive<i16> = (y_bound.0 - 1)..=(y_bound.1 + 1);
    let z_range: RangeInclusive<i16> = (z_bound.0 - 1)..=(z_bound.1 + 1);

    // Flood fill surrounding space of droplets
    let mut air_blocks: HashSet<(i16, i16, i16)> = HashSet::new();
    let mut bfs: VecDeque<(i16, i16, i16)> = VecDeque::new();
    bfs.push_back((x_bound.0, y_bound.0, z_bound.0));
    while let Some((x, y, z)) = bfs.pop_front() {
        // Check the 6 adjacent sides
        for (dx, dy, dz) in ADJACENT_OFFSET {
            // Coord of adjacent droplet
            let adj_droplet: (i16, i16, i16) = (x + dx, y + dy, z + dz);

            // If outside bound, skip iteration
            if x_range.contains(&adj_droplet.0)
                && y_range.contains(&adj_droplet.1)
                && z_range.contains(&adj_droplet.2)
            {
                // Skip if adjacent block is part of droplets or has already been visited
                if grid.contains(&adj_droplet) || air_blocks.contains(&adj_droplet) {
                    continue;
                }

                // Record air block
                air_blocks.insert(adj_droplet);
                bfs.push_back(adj_droplet);
            }
        }
    }

    // Find air blocks that are adjacent to droplets and count the touching surfaces
    let surface_area: u16 = air_blocks
        .iter()
        .map(|(ax, ay, az)| {
            // Track touching surfaces of air and droplet
            let mut touching_surfaces: u16 = 0;
            // Check adjacent sides
            for (dx, dy, dz) in ADJACENT_OFFSET {
                if grid.contains(&(*ax + dx, *ay + dy, *az + dz)) {
                    touching_surfaces += 1;
                }
            }

            touching_surfaces
        })
        .sum::<u16>();

    Some(surface_area)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
