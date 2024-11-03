// LeetCode problem 0796: Rotate String
// https://leetcode.com/problems/rotate-string/description/

fn main() {
    let result = rotate_string("abcde".into(), "cdeab".into());
    println!("Result = {result}");
}

fn rotate_string(mut s: String, goal: String) -> bool {
    let n = s.len();
    let g = goal.len();
    if n != g {
        return false;
    }

    unsafe {
        let s_bytes = s.as_mut_vec();
        let goal_bytes = goal.into_bytes();

        for _ in 0..g {
            if *s_bytes == goal_bytes {
                return true;
            }

            // rotate s
            let first = s_bytes[0];
            for i in 0..n - 1 {
                s_bytes[i] = s_bytes[i + 1];
            }
            s_bytes[n - 1] = first;
        }
    }

    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = rotate_string("abcde".into(), "cdeab".into());
        assert!(result);
    }

    #[test]
    fn example_two() {
        let result = rotate_string("abcde".into(), "abced".into());
        assert!(!result);
    }
}
