use std::{cell::RefCell, rc::Rc};

// struct Node {
//     value: i64,
//     prev: Option<Rc<RefCell<Node>>>,
//     next: Option<Rc<RefCell<Node>>>
// }

struct Node<T> {
    value: T,
    idx: usize,
    prev: usize,
    next: usize,
}

fn decrypt_file(file: &mut [Rc<RefCell<Node<i64>>>], num_mix: u8) -> i64 {
    // A perfect cycle is length - 1 moves
    let size: i64 = file.len() as i64 - 1;

    // Number of times to mix
    for _ in 0..num_mix {
        for node in file.iter() {
            // Node to reposition
            let extracted_node: Rc<RefCell<Node<i64>>> = Rc::clone(node);

            // If value is 0, skip iteration
            if (*extracted_node).borrow().value == 0 {
                continue;
            }

            // Collapse prev and next of current
            let prev_idx: usize = (*extracted_node).borrow().prev;
            let next_idx: usize = (*extracted_node).borrow().next;
            (*file[prev_idx]).borrow_mut().next = next_idx;
            (*file[next_idx]).borrow_mut().prev = prev_idx;

            // Calculate number of moves to shift
            let moves: i64 = (*extracted_node).borrow().value.abs() % size;
            let mut cursor: usize = (*extracted_node).borrow().idx;
            if (*extracted_node).borrow().value < 0 {
                // Inclusive range so that so that cursor is the new prev
                for _ in 0..=moves {
                    cursor = file[cursor].borrow().prev;
                }
            } else {
                for _ in 0..moves {
                    cursor = file[cursor].borrow().next;
                }
            }

            // Merge node into new position
            let new_next_idx: usize = (*file[cursor]).borrow().next;
            (*extracted_node).borrow_mut().prev = cursor;
            (*extracted_node).borrow_mut().next = new_next_idx;

            (*file[cursor]).borrow_mut().next = (*extracted_node).borrow().idx;
            (*file[new_next_idx]).borrow_mut().prev = (*extracted_node).borrow().idx;

            // let mut tmp: usize = (*extracted_node).borrow().idx;
            // print!("{:?} after {:?} moves: ", (*extracted_node).borrow().value, moves);
            // for _ in 0..file.len() {
            //     let n = (*file[tmp]).borrow();
            //     print!("{:?} ", n.value);
            //     tmp = n.next;
            // }
            // println!();
        }
    }

    // Find position of 0
    let zero_idx: usize = file
        .iter()
        .find(|&n| (*n).borrow().value == 0)
        .unwrap()
        .borrow()
        .idx;
    // Calc sum of offsets after 0
    [1000_i64, 2000_i64, 3000_i64]
        .iter()
        .map(|offset| {
            let moves: i64 = offset % (size + 1);
            let mut cursor: usize = zero_idx;
            for _ in 0..moves {
                cursor = (*file[cursor]).borrow().next;
            }

            (*file[cursor]).borrow().value
        })
        .sum::<i64>()
}

pub fn part_one(input: &str) -> Option<i64> {
    const KEY: i64 = 1;
    const NUM_MIX: u8 = 1;

    // Generate double linked list backed by a vector of references
    let mut dll: Vec<Rc<RefCell<Node<i64>>>> = input
        .lines()
        .enumerate()
        .map(|(i, n)| {
            Rc::new(RefCell::new(Node {
                value: n.parse::<i64>().unwrap() * KEY,
                idx: i,
                prev: if i == 0 { 0 } else { i - 1 },
                next: i + 1,
            }))
        })
        .collect();
    // Set prev of first element to wrap around the end
    dll[0].borrow_mut().prev = dll.len() - 1;
    // Set next of last element to wrap around the front
    dll[dll.len() - 1].borrow_mut().next = 0;

    Some(decrypt_file(&mut dll, NUM_MIX))
}

pub fn part_two(input: &str) -> Option<i64> {
    const KEY: i64 = 811589153;
    const NUM_MIX: u8 = 10;

    // Generate double linked list backed by a vector of references
    let mut dll: Vec<Rc<RefCell<Node<i64>>>> = input
        .lines()
        .enumerate()
        .map(|(i, n)| {
            Rc::new(RefCell::new(Node {
                value: n.parse::<i64>().unwrap() * KEY,
                idx: i,
                prev: if i == 0 { 0 } else { i - 1 },
                next: i + 1,
            }))
        })
        .collect();
    // Set prev of first element to wrap around the end
    dll[0].borrow_mut().prev = dll.len() - 1;
    // Set next of last element to wrap around the front
    dll[dll.len() - 1].borrow_mut().next = 0;

    Some(decrypt_file(&mut dll, NUM_MIX))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
