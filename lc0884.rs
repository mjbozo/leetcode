// LeetCode problem 0884: Uncommon Words from Two Sentences
// https://leetcode.com/problems/uncommon-words-from-two-sentences/description/

use std::collections::HashMap;

fn main() {
    let result = uncommon_from_sentences(String::from("fd kss fd"), String::from("fd fd kss"));
    println!("Result = {result:?}");
}

fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let mut word_counts: HashMap<&str, i32> = HashMap::new();
    for word in s1.split(" ") {
        word_counts
            .entry(word)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for word in s2.split(" ") {
        word_counts
            .entry(word)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    return word_counts
        .into_iter()
        .filter(|&(_, value)| value == 1)
        .map(|(key, _)| String::from(key))
        .collect();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = uncommon_from_sentences(
            String::from("this apple is sweet"),
            String::from("this apple is sour"),
        );
        assert_eq!(result, vec![String::from("sweet"), String::from("sour")]);
    }

    #[test]
    fn example_two() {
        let result = uncommon_from_sentences(String::from("apple apple"), String::from("banana"));
        assert_eq!(result, vec![String::from("banana")]);
    }
}
