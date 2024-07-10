// LeetCode problem 0076: Minimum Window Substring
// https://leetcode.com/problems/minimum-window-substring/description/

fn main() {
    let result = min_window(String::from("ADOBECODEBANC"), String::from("ABC"));
    println!("Result = {result}");
}

fn min_window(s: String, t: String) -> String {
    let m = s.len();
    let n = t.len();

    if m < n {
        return String::new();
    }

    let mut t_counts = vec![0; (b'z' - b'A') as usize + 1];
    for b in t.bytes() {
        t_counts[(b - b'A') as usize] += 1;
    }

    let mut left = 0;
    let mut right = 0;
    let mut required = t.len();
    let mut win = None;
    let s_bytes = s.as_bytes();
    
    while right < s.len() {
        if t_counts[(s_bytes[right] - b'A') as usize] > 0 {
            required -= 1;
        }
        t_counts[(s_bytes[right] - b'A') as usize] -= 1;
        right += 1;

        while required == 0 {
            if right - left < win.map_or(usize::MAX, |(left, right)| right - left) {
                win = Some((left, right));
            }

            t_counts[(s_bytes[left] - b'A') as usize] += 1;
            if t_counts[(s_bytes[left] - b'A') as usize] > 0 {
                required += 1;
            }
            left += 1;
        }
    }

    return win.map_or(String::new(), |(left, right)| s[left..right].into());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = min_window(String::from("ADOBECODEBANC"), String::from("ABC"));
        assert_eq!(result, String::from("BANC"));
    }

    #[test]
    fn example_two() {
        let result = min_window(String::from("a"), String::from("a"));
        assert_eq!(result, String::from("a"));
    }

    #[test]
    fn example_three() {
        let result = min_window(String::from("a"), String::from("aa"));
        assert_eq!(result, String::new());
    }
}
