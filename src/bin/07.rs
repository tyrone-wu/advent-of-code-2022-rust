use std::collections::{BTreeMap, VecDeque};

// Directory struct
struct Directory {
    // name: String,                             // Name of dir
    parent_dir: Option<usize>,                // Index of parent dir
    size: u64,                                // Size of dir
    subdirs: Option<BTreeMap<String, usize>>, // Key pairs of dir name with their index
}

impl Directory {
    fn add_size(&mut self, add_size: u64) {
        self.size += add_size;
    }

    fn set_subdirs(&mut self, subdirs: BTreeMap<String, usize>) {
        self.subdirs = Some(subdirs);
    }
}

// ----------------------------------------------------------------------------

fn generate_fs(input: &str) -> VecDeque<Directory> {
    // Stores the filesystem structure with only sizes
    let mut filesystem: VecDeque<Directory> = VecDeque::new();

    // Add root dir
    filesystem.push_back(Directory {
        // name: String::from("/"),
        parent_dir: None,
        size: 0,
        subdirs: None,
    });
    // Initial position of the filesystem
    let mut curr_position: usize = 0;

    // Iterate over cli commands
    let cli_it = input.split("\n$ ").skip(1);
    for command in cli_it {
        // Line iterator
        let command_vec: Vec<&str> = command.lines().collect();
        let command_type: &str = command_vec[0];

        if command_type.starts_with("cd") {
            // Get dir name to change into
            let dir_name: &str = command_type.rsplit_once(' ').unwrap().1;

            // Set current directory of the filesystem
            match dir_name {
                "/" => curr_position = 0,
                ".." => curr_position = filesystem[curr_position].parent_dir.unwrap(),
                _ => {
                    curr_position = *filesystem[curr_position]
                        .subdirs
                        .clone()
                        .unwrap()
                        .get(dir_name)
                        .unwrap()
                }
            }
        } else if command_type.starts_with("ls") {
            // Ensures that "ls" output is not double counted
            if filesystem[curr_position].subdirs.is_none() {
                // If "ls" output is not empty
                if command_vec.len() != 1 {
                    // Store subdirs
                    let mut subdirs: BTreeMap<String, usize> = BTreeMap::new();
                    // Store size
                    let mut dir_size: u64 = 0;

                    // Parse "ls" output
                    for out in command_vec[1..].iter() {
                        // Output is dir
                        if out.starts_with("dir") {
                            // Get dir name
                            let dir_name: &str = out.split_whitespace().last().unwrap();
                            // Index of the dir to insert
                            let insert_idx: usize = filesystem.len();

                            // Insert sub dir
                            filesystem.push_back(Directory {
                                // name: dir_name.to_string(),
                                parent_dir: Some(curr_position),
                                size: 0,
                                subdirs: None,
                            });

                            // Record inserted sub dir
                            subdirs.insert(dir_name.to_string(), insert_idx);
                        } else {
                            // Output is file
                            let file_size: u64 = out
                                .split_whitespace()
                                .next()
                                .unwrap()
                                .parse::<u64>()
                                .unwrap();
                            dir_size += file_size;
                        }
                    }

                    // Set sub dirs and size
                    filesystem[curr_position].set_subdirs(subdirs);
                    filesystem[curr_position].add_size(dir_size);

                    // Propogate dir size to parent dirs
                    let mut parent_position: Option<usize> = filesystem[curr_position].parent_dir;
                    while parent_position.is_some() {
                        // Add sub dir size to parent dir
                        filesystem[parent_position.unwrap()].add_size(dir_size);
                        // Move to parent dir
                        parent_position = filesystem[parent_position.unwrap()].parent_dir;
                    }
                }
            }
        }
    }

    filesystem
}

// ----------------------------------------------------------------------------

pub fn part_one(input: &str) -> Option<u64> {
    // Filesystem as Vector of Directories
    let filesystem: VecDeque<Directory> = generate_fs(input);

    // Calculate sum of sizes < 100000
    let mut total_size: u64 = 0;
    for d in filesystem.iter() {
        // println!("--- Directory ---");
        // println!("Index: {i}");
        // println!("Name: {}", d.name);
        // println!("Parent Directory: {}", d.parent_dir.unwrap_or(0));
        // println!("Size: {}", d.size);
        // println!("------ Subdir ---");
        // for (n, di) in d.subdirs.clone().unwrap() {
        //     println!("    Name: {n}\tIndex: {di}");
        // }

        if d.size <= 100000 {
            total_size += d.size;
        }
    }

    Some(total_size)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Filesystem as Vector of Directories
    let filesystem: VecDeque<Directory> = generate_fs(input);

    // Total capacity of disk
    let capacity: u64 = 70000000;
    // Needed space
    let req_space: u64 = 30000000;
    // Current used space
    let used_space: u64 = capacity - filesystem[0].size;

    // Track min dir size
    let mut min_size: u64 = u64::MAX;
    for d in filesystem.iter() {
        // If deleting dir is gives enough space, and dir is a new minimum
        if (used_space + d.size >= req_space) && (d.size < min_size) {
            min_size = d.size;
        }
    }

    Some(min_size)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
