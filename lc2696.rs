// LeetCode problem 2696: Minimum String Length After Removing Substrings
// https://leetcode.com/problems/minimum-string-length-after-removing-substrings/description/

fn main() {
    let result = min_length(String::from("ABDCACDB"));
    println!("Result = {result}");
}

fn min_length(s: String) -> i32 {
    let mut removed = 0;
    let mut stack = Vec::with_capacity(s.len());

    for c in s.chars() {
        match c {
            'B' => {
                if let Some(x) = stack.last() {
                    if *x == 'A' {
                        stack.pop();
                        removed += 2;
                    } else {
                        stack.push(c);
                    }
                }
            }
            'D' => {
                if let Some(x) = stack.last() {
                    if *x == 'C' {
                        stack.pop();
                        removed += 2;
                    } else {
                        stack.push(c);
                    }
                }
            }
            x => {
                stack.push(x);
            }
        };
    }

    return s.len() as i32 - removed;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = min_length(String::from("ABDCACDB"));
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = min_length(String::from("ACBBD"));
        assert_eq!(result, 5);
    }
}
