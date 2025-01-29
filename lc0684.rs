// LeetCode problem 0684: Redundant Connection
// https://leetcode.com/problems/redundant-connection/description/

fn main() {
    let result = find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]);
    println!("Result = {result:?}");
}

fn find(forest: &mut Vec<i32>, node: i32) -> i32 {
    if forest[node as usize] != node {
        return find(forest, forest[node as usize]);
    }
    return node;
}

fn union(forest: &mut Vec<i32>, x: i32, y: i32) -> (bool, Vec<i32>) {
    let p1 = find(forest, x);
    let p2 = find(forest, y);
    if p1 == p2 {
        return (false, vec![x, y]);
    }

    forest[p1 as usize] = p2;
    return (true, vec![]);
}

fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut redundant = vec![-1, -1];
    let mut forest = vec![0; edges.len() + 1];

    for i in 0..forest.len() {
        forest[i] = i as i32;
    }

    for edge in edges {
        let (merged, e) = union(&mut forest, edge[0], edge[1]);
        if !merged {
            redundant = e;
        }
    }

    return redundant;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn example_two() {
        let result = find_redundant_connection(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![1, 4],
            vec![1, 5],
        ]);
        assert_eq!(result, vec![1, 4]);
    }
}
