// LeetCode problem 2924: Find Champion II
// https://leetcode.com/problems/find-champion-ii/description/

fn main() {
    let result = find_champion(3, vec![vec![0, 1], vec![1, 2]]);
    println!("Result = {result}");
}

fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut teams = vec![true; n as usize];
    for edge in edges {
        teams[edge[1] as usize] = false;
    }

    let champs = teams
        .iter()
        .enumerate()
        .filter(|&(_, v)| *v)
        .map(|v| v.0)
        .collect::<Vec<_>>();
    if champs.len() > 1 {
        return -1;
    }

    return champs[0] as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_champion(3, vec![vec![0, 1], vec![1, 2]]);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_two() {
        let result = find_champion(4, vec![vec![0, 2], vec![1, 3], vec![1, 2]]);
        assert_eq!(result, -1);
    }
}
