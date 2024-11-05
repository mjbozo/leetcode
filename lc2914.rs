// LeetCode problem 2914: Minimum Number of Changes to Make Binary String Beautiful
// https://leetcode.com/problems/minimum-number-of-changes-to-make-binary-string-beautiful/description/

fn main() {
    let result = min_changes("1001".into());
    println!("Result = {result}");
}

fn min_changes(s: String) -> i32 {
    let mut changes = 0;

    for c in s.as_bytes().chunks(2) {
        if c[0] != c[1] {
            changes += 1;
        }
    }

    return changes;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = min_changes("1001".into());
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = min_changes("10".into());
        assert_eq!(result, 1);
    }

    #[test]
    fn example_three() {
        let result = min_changes("0000".into());
        assert_eq!(result, 0);
    }
}
