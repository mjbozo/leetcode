// LeetCode problem 1684: Count the Number of Consistent Strings
// https://leetcode.com/problems/count-the-number-of-consistent-strings/description/

fn main() {
    let result = count_consistent_strings(
        "ab".into(),
        vec![
            "ad".into(),
            "db".into(),
            "aaab".into(),
            "baa".into(),
            "badab".into(),
        ],
    );
    println!("Result = {result}");
}

fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let mut chars = [0; 26];
    let offset = 'a' as u8;
    for c in allowed.as_bytes() {
        chars[(c - offset) as usize] = 1;
    }

    return words
        .iter()
        .filter(|w| w.bytes().all(|x| chars[(x - offset) as usize] == 1))
        .collect::<Vec<_>>()
        .len() as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = count_consistent_strings(
            "ab".into(),
            vec![
                "ad".into(),
                "db".into(),
                "aaab".into(),
                "baa".into(),
                "badab".into(),
            ],
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn example_two() {
        let result = count_consistent_strings(
            "abc".into(),
            vec![
                "a".into(),
                "b".into(),
                "c".into(),
                "ab".into(),
                "ac".into(),
                "bc".into(),
                "abc".into(),
            ],
        );
        assert_eq!(result, 7);
    }

    #[test]
    fn example_three() {
        let result = count_consistent_strings(
            "cad".into(),
            vec![
                "cc".into(),
                "acd".into(),
                "b".into(),
                "ba".into(),
                "bac".into(),
                "bad".into(),
                "ac".into(),
                "d".into(),
            ],
        );
        assert_eq!(result, 4);
    }
}
