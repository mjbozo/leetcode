// LeetCode problem 2406: Divide Intervals Into Minimum Number of Groups
// https://leetcode.com/problems/divide-intervals-into-minimum-number-of-groups/description/

fn main() {
    let result = min_groups(vec![
        vec![5, 10],
        vec![6, 8],
        vec![1, 5],
        vec![2, 3],
        vec![1, 10],
    ]);
    println!("Result = {result}");
}

fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
    let mut events = vec![];
    for interval in intervals {
        events.push((interval[0], 1));
        events.push((interval[1] + 1, -1));
    }

    events.sort_unstable();

    let mut max_groups = 0;
    let mut current_groups = 0;

    for event in events {
        current_groups += event;
        max_groups = std::cmp::max(max_groups, current_groups);
    }

    return max_groups;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = min_groups(vec![
            vec![5, 10],
            vec![6, 8],
            vec![1, 5],
            vec![2, 3],
            vec![1, 10],
        ]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = min_groups(vec![vec![1, 3], vec![5, 6], vec![8, 10], vec![11, 13]]);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_three() {
        let result = min_groups(vec![vec![4, 5], vec![5, 14], vec![1, 3], vec![7, 8]]);
        assert_eq!(result, 2);
    }
}
