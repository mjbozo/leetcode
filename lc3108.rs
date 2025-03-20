// LeetCode problem 3108: Minimum Cost Walk in Weighted Graph
// https://leetcode.com/problems/minimum-cost-walk-in-weighted-graph/description/

use std::collections::{HashSet, VecDeque};

fn main() {
    let result = minimum_cost(
        5,
        vec![vec![0, 1, 7], vec![1, 3, 7], vec![1, 2, 1]],
        vec![vec![0, 3], vec![3, 4]],
    );
    println!("Result = {result:?}");
}

fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
    let mut adj = vec![vec![]; n as usize];
    for edge in edges {
        adj[edge[0] as usize].push((edge[1], edge[2]));
        adj[edge[1] as usize].push((edge[0], edge[2]));
    }

    let mut seen: HashSet<usize> = HashSet::new();
    let mut cluster_maps = vec![0; n as usize];
    let mut cluster_walks = vec![];

    for (i, node) in adj.iter().enumerate() {
        if !seen.contains(&i) {
            seen.insert(i);
            let c = cluster_walks.len();
            cluster_walks.push(u32::MAX);
            cluster_maps[i] = c;

            if node.len() == 0 {
                continue;
            }

            let mut queue = VecDeque::new();
            for n in node {
                queue.push_back(n)
            }

            while queue.len() > 0 {
                let current = queue.pop_front().unwrap();
                cluster_walks[c] &= current.1 as u32;
                if !seen.contains(&(current.0 as usize)) {
                    seen.insert(current.0 as usize);
                    cluster_maps[current.0 as usize] = c;
                    for n in &adj[current.0 as usize] {
                        queue.push_back(&n);
                    }
                }
            }
        }
    }

    let mut result = vec![];
    for q in query {
        if cluster_maps[q[0] as usize] == cluster_maps[q[1] as usize] {
            result.push(cluster_walks[cluster_maps[q[0] as usize]] as i32);
        } else {
            result.push(-1);
        }
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = minimum_cost(
            5,
            vec![vec![0, 1, 7], vec![1, 3, 7], vec![1, 2, 1]],
            vec![vec![0, 3], vec![3, 4]],
        );
        assert_eq!(result, vec![1, -1]);
    }

    #[test]
    fn example_two() {
        let result = minimum_cost(
            3,
            vec![vec![0, 2, 7], vec![0, 1, 15], vec![1, 2, 6], vec![1, 2, 1]],
            vec![vec![1, 2]],
        );
        assert_eq!(result, vec![0]);
    }
}
