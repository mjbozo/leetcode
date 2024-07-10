// LeetCode problem 1598: Crawler Log Folder
// https://leetcode.com/problems/crawler-log-folder/description/

fn main() {
    let result = min_operations(vec!["d1/".into(), "d2/".into(), "../".into(), "d21/".into(), "./".into()]);
    println!("Result = {result}");
}

fn min_operations(logs: Vec<String>) -> i32 {
    let mut depth = 0;
    for log in logs.iter() {
        match &log[0..2] {
            ".." => {
                depth = (depth - 1).max(0);
            },
            "./" => {},
            _ => depth += 1,
        };
    }
    return depth;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let result = min_operations(vec!["d1/".into(), "d2/".into(), "../".into(), "d21/".into(), "./".into()]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = min_operations(vec!["d1/".into(), "d2/".into(), "./".into(), "d3/".into(), "../".into(), "d31/".into()]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_three() {
        let result = min_operations(vec!["d1/".into(), "../".into(), "../".into(), "../".into()]);
        assert_eq!(result, 0);
    }
}
