// LeetCode problem 1750: Minimum Length of String After Deleting Similar Ends
// https://leetcode.com/problems/minimum-length-of-string-after-deleting-similar-ends/description/

fn main() {
    let result = minimum_length(String::from("ca"));
    println!("Result = {result}");
}

fn minimum_length(s: String) -> i32 {
    let mut left = 0;
    let mut right = s.len() - 1;
    let bytes = s.as_bytes();

    while left < right {
        let c = bytes[left];
        if bytes[right] != c {
            break;
        }

        while bytes[left] == c {
            left += 1;
            if left == right {
                return 0;
            }
        }

        while bytes[right] == c {
            right -= 1;
        }
    }

    return (right - left + 1) as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = minimum_length(String::from("ca"));
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = minimum_length(String::from("cabaabac"));
        assert_eq!(result, 0);
    }

    #[test]
    fn example_three() {
        let result = minimum_length(String::from("aabccabba"));
        assert_eq!(result, 3);
    }
}
