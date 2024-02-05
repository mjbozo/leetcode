// LeetCode problem 0387: First Unique Character in a String
// https://leetcode.com/problems/first-unique-character-in-a-string/description/

fn main () {
    let result = first_uniq_char(String::from("leetcode"));
    println!("Result = {result}");
}

fn first_uniq_char(s: String) -> i32 {
    let mut char_index_counts = vec![(-1, 0); 26]; // (index, count)
    for (index, c) in s.bytes().enumerate() {
        let mut entry = char_index_counts[(c - b'a') as usize];
        entry.1 += 1;
        if entry.1 == 1 {
            entry.0 = index as i32;
        }

        char_index_counts[(c - b'a') as usize] = entry;
    }

    return char_index_counts.iter().filter(|&v| v.1 == 1).map(|&v| v.0).min().unwrap_or(-1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = first_uniq_char(String::from("leetcode"));
        assert_eq!(result, 0);
    }

    #[test]
    fn example_two() {
        let result = first_uniq_char(String::from("loveleetcode"));
        assert_eq!(result, 2);
    }

    #[test]
    fn example_three() {
        let result = first_uniq_char(String::from("aabb"));
        assert_eq!(result, -1);
    }
}
