// LeetCode problem 2516: Take K of Each Character From Left and Right
// https://leetcode.com/problems/take-k-of-each-character-from-left-and-right/description/

fn main() {
    let result = take_characters("aabaaaacaabc".into(), 2);
    println!("Result = {result}");
}

fn take_characters(s: String, k: i32) -> i32 {
    if k == 0 {
        return 0;
    }

    let chars = s.bytes().collect::<Vec<_>>();
    let mut char_counts = vec![0; 3];

    let mut left = 0_i32;
    let mut right = s.len() - 1;

    while left < s.len() as i32 {
        char_counts[(chars[left as usize] - b'a') as usize] += 1;
        if char_counts.iter().all(|v| *v >= k) {
            break;
        }
        left += 1;
    }

    if char_counts.iter().any(|v| *v < k) {
        return -1;
    }

    let mut minimum = left + 1;
    while left >= 0 {
        char_counts[(chars[left as usize] - b'a') as usize] -= 1;
        left -= 1;

        while char_counts[(chars[(left + 1) as usize] - b'a') as usize] < k {
            char_counts[(chars[right] - b'a') as usize] += 1;
            right -= 1;
        }

        let chars_taken = left + 1 + (s.len() - right - 1) as i32;
        minimum = std::cmp::min(minimum, chars_taken);
    }

    return minimum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = take_characters("aabaaaacaabc".into(), 2);
        assert_eq!(result, 8);
    }

    #[test]
    fn example_two() {
        let result = take_characters("a".into(), 1);
        assert_eq!(result, -1);
    }
}
