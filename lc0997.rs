// LeetCode problem 0997: Find the Town Judge
// https://leetcode.com/problems/find-the-town-judge/description/

fn main() {
    let result = find_judge(1, vec![]);
    println!("Result = {result}");
}

fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    if n == 1 {
        return 1;
    }

    let mut trustworthiness = vec![0; n as usize + 1];
    for tp in trust {
        trustworthiness[tp[0] as usize] -= 1;
        trustworthiness[tp[1] as usize] += 1;
    }

    for (i, &t) in trustworthiness.iter().enumerate() {
        if t == n - 1 {
            return i as i32;
        }
    }

    return -1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_judge(2, vec![vec![1, 2]]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = find_judge(3, vec![vec![1, 3], vec![2, 3]]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_three() {
        let result = find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]);
        assert_eq!(result, -1);
    }
}
