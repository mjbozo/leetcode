// LeetCode problem 2092: Find All People With Secret
// https://leetcode.com/problems/find-all-people-with-secret/description/

use std::collections::{BinaryHeap, HashMap};

fn main() {
    let result = find_all_people(6, vec![vec![1,2,5], vec![2,3,8], vec![1,5,5]], 1);
    println!("Result = {result:?}");
}

fn find_all_people(_n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
    let mut has_secret = HashMap::<i32, i32>::new();
    has_secret.insert(0, 0);
    has_secret.insert(first_person, 0);
    let mut graph: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

    for meeting in meetings {
        graph.entry(meeting[0]).and_modify(|v| v.push((meeting[1], meeting[2])))
            .or_insert(vec![(meeting[1], meeting[2])]);

        graph.entry(meeting[1]).and_modify(|v| v.push((meeting[0], meeting[2])))
            .or_insert(vec![(meeting[0], meeting[2])]);
    }

    // bfs to share secret
    let mut open_set = BinaryHeap::new();
    open_set.push((0, 0));
    open_set.push((first_person, 0));
    while let Some(n) = open_set.pop() {
        if let Some(edges) = graph.get(&n.0) {
            for edge in edges {
                if edge.1 < n.1 {
                    continue;
                }

                if let Some(existing) = has_secret.get_mut(&edge.0) {
                    if edge.1 < *existing {
                        *existing = edge.1;
                        open_set.push((edge.0, edge.1));
                    }
                } else {
                    has_secret.insert(edge.0, edge.1);
                    open_set.push((edge.0, edge.1));
                }
            }
        }
    }
    // sort final list for easier testing
    let mut secret_holders = has_secret.into_iter().map(|v| v.0).collect::<Vec<_>>();
    secret_holders.sort();
    return secret_holders;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_all_people(6, vec![vec![1,2,5], vec![2,3,8], vec![1,5,10]], 1);
        assert_eq!(result, vec![0,1,2,3,5]);
    }

    #[test]
    fn example_two() {
        let result = find_all_people(4, vec![vec![3,1,3], vec![1,2,2], vec![0,3,3]], 3);
        assert_eq!(result, vec![0,1,3]);
    }

    #[test]
    fn example_three() {
        let result = find_all_people(5, vec![vec![3,4,2], vec![2,3,1], vec![1,2,1]], 1);
        assert_eq!(result, vec![0,1,2,3,4]);
    }

    #[test]
    fn example_four() {
        let result = find_all_people(6, vec![vec![0,2,1], vec![1,3,1], vec![4,5,1]], 1);
        assert_eq!(result, vec![0,1,2,3]);
    }
}
