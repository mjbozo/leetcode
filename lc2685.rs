// LeetCode problem 2685: Count the Number of Complete Components
// https://leetcode.com/problems/count-the-number-of-complete-components/description/

use std::collections::{HashSet, VecDeque};

fn main() {
    let result = count_complete_components(6, vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]]);
    println!("Result = {result}");
}

fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut adj = vec![vec![]; n as usize];

    for edge in edges {
        adj[edge[0] as usize].push(edge[1]);
        adj[edge[1] as usize].push(edge[0]);
    }

    let mut processed = HashSet::new();
    let mut components = vec![];
    let mut component_map = vec![0; n as usize];
    for i in 0..n {
        if !processed.contains(&i) {
            processed.insert(i);

            let c = components.len();
            components.push(vec![i]);
            component_map[i as usize] = c;

            let mut q = VecDeque::new();
            for n in &adj[i as usize] {
                q.push_back(*n);
            }

            while q.len() > 0 {
                let current = q.pop_front().unwrap();
                if !processed.contains(&current) {
                    processed.insert(current);
                    components[c].push(current);
                    component_map[current as usize] = c;
                    for n in &adj[current as usize] {
                        q.push_back(*n);
                    }
                }
            }
        }
    }

    let mut connecteds: Vec<usize> = vec![0; components.len()];
    for i in 0..n {
        if components[component_map[i as usize]].len() - 1 == adj[i as usize].len() {
            connecteds[component_map[i as usize]] += 1;
        }
    }

    let mut result = 0;
    for i in 0..connecteds.len() {
        if connecteds[i] == components[i].len() {
            result += 1;
        }
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result =
            count_complete_components(6, vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = count_complete_components(
            6,
            vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]],
        );
        assert_eq!(result, 1);
    }
}
