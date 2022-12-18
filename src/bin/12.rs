use std::collections::{BinaryHeap, LinkedList};

// If current node has an incoming edge to the other node
fn is_neighbor(mut curr_node: u8, mut other_node: u8) -> bool {
    if curr_node == b'S' {
        curr_node = b'a';
    } else if curr_node == b'E' {
        curr_node = b'z';
    }

    if other_node == b'S' {
        other_node = b'a';
    } else if other_node == b'E' {
        other_node = b'z';
    }

    curr_node <= other_node + 1
}

// Parse map into adjacency list; find start and end node
fn generate_adjacency_list(
    input: &str,
    part_two: bool,
) -> (Vec<LinkedList<usize>>, Vec<usize>, usize) {
    // 2D vector of chars
    let map: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    // Adjacency list
    let mut adj_list: Vec<LinkedList<usize>> = Vec::with_capacity(map.len() * map[0].len());

    // Start nodes
    let mut starts: Vec<usize> = Vec::new();
    // End node
    let mut end: usize = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            // Edge list for node (i,j)
            let mut edges: LinkedList<usize> = LinkedList::new();

            if map[i][j] == b'S' || (part_two && map[i][j] == b'a') {
                // Set starting node
                starts.push(adj_list.len());
            } else if map[i][j] == b'E' {
                // End starting node
                end = adj_list.len();
            }

            // Edge to top node
            if i > 0 && is_neighbor(map[i][j], map[i - 1][j]) {
                edges.push_back((i - 1) * map[0].len() + j);
            }
            // Edge to bottom node
            if i < (map.len() - 1) && is_neighbor(map[i][j], map[i + 1][j]) {
                edges.push_back((i + 1) * map[0].len() + j);
            }
            // Edge to left node
            if j > 0 && is_neighbor(map[i][j], map[i][j - 1]) {
                edges.push_back(i * map[0].len() + j - 1);
            }
            // Edge to right node
            if j < (map[0].len() - 1) && is_neighbor(map[i][j], map[i][j + 1]) {
                edges.push_back(i * map[0].len() + j + 1);
            }

            // Add edge list to node
            adj_list.push(edges);
        }
    }

    (adj_list, starts, end)
}

// Get shorest path from S to E using Dijkstra's algorithm
fn shortest_distance_dijkstra(
    adj_list: &[LinkedList<usize>],
    start: usize,
    end: &[usize],
) -> Option<u32> {
    // Keep track of visitied nodes
    let mut nodes_visited: Vec<bool> = vec![false; adj_list.len()];
    // // Keep track of previous nodes taken to our node of interest
    // let mut prev_nodes: Vec<Option<usize>> = vec![None; adj_list.len()];

    // Distance array from start node to every other node
    let mut dist_nodes: Vec<u32> = vec![u32::MAX; adj_list.len()];
    dist_nodes[start] = 0; // Initialize starting node

    // Priority queue of dist and node; use (MAX - dist) to turn min heap into max heap
    let mut pq: BinaryHeap<(u32, usize)> = BinaryHeap::new();
    pq.push((u32::MAX, start)); // Initialize starting node

    // Until priority queue is empty
    while let Some((mut dist, node)) = pq.pop() {
        dist = u32::MAX - dist;
        // Visit node
        nodes_visited[node] = true;

        // Skip node if going through the node already results in greater distance
        if dist_nodes[node] < dist {
            continue;
        }

        // Iterate incoming edges of node
        for e_node in &adj_list[node] {
            // Go to next iteration if edge to node is already visited
            if nodes_visited[*e_node] {
                continue;
            }

            // Taking the path increments distance by 1
            let new_dist: u32 = dist_nodes[node] + 1;
            // If taking edge results in less total distance to the node
            if new_dist < dist_nodes[*e_node] {
                // Set new distance to the node
                dist_nodes[*e_node] = new_dist;
                // // Update path for the node to take
                // prev_nodes[*e_node] = Some(node);
                // Add node to priority queue
                pq.push((u32::MAX - new_dist, *e_node));
            }
        }

        // Return early if end node is reached
        if end.contains(&node) {
            return Some(dist_nodes[node]);
        }
    }

    None
}

// ----------------------------------------------------------------------------

pub fn part_one(input: &str) -> Option<u32> {
    let (adj_list, starts, end): (Vec<LinkedList<usize>>, Vec<usize>, usize) =
        generate_adjacency_list(input, false);

    shortest_distance_dijkstra(&adj_list, end, &starts)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (adj_list, starts, end): (Vec<LinkedList<usize>>, Vec<usize>, usize) =
        generate_adjacency_list(input, true);

    shortest_distance_dijkstra(&adj_list, end, &starts)
}

// ----------------------------------------------------------------------------

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
