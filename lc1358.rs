// LeetCode problem 1358: Number of Substrings Containing All Three Characters
// https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/description/

fn main() {
    let result = number_of_substrings("abcabc".into());
    println!("Result = {result}");
}

fn number_of_substrings(s: String) -> i32 {
    let n = s.len();
    let b = s.as_bytes();
    let mut counts = vec![0; 3];
    let mut subs = 0;
    let mut left = 0;
    let mut right = 0;

    while right < n {
        counts[(b[right] - b'a') as usize] += 1;
        let mut current = 0;
        while counts[0] > 0 && counts[1] > 0 && counts[2] > 0 {
            current += n - right;
            counts[(b[left as usize] - b'a') as usize] -= 1;
            left += 1;
        }
        subs += current;
        right += 1;
    }

    return subs as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = number_of_substrings("abcabc".into());
        assert_eq!(result, 10);
    }

    #[test]
    fn example_two() {
        let result = number_of_substrings("aaacb".into());
        assert_eq!(result, 3);
    }

    #[test]
    fn example_three() {
        let result = number_of_substrings("abc".into());
        assert_eq!(result, 1);
    }
}
