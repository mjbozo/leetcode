// LeetCode problem 0451: Sort Characters By Frequency
// https://leetcode.com/problems/sort-characters-by-frequency/description/

use std::collections::HashMap;

fn main() {
    let result = frequency_sort(String::from("tree"));
    println!("Result = {result}");
}

fn frequency_sort(s: String) -> String {
    let mut frequencies = HashMap::<char, i32>::new();

    for c in s.chars() {
        frequencies.entry(c).and_modify(|ch| *ch += 1).or_insert(1);
    }

    let mut freq_vec = frequencies.iter().collect::<Vec<_>>();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1));

    let mut sorted_string = String::new();
    for c in freq_vec {
        for _ in 0..*c.1 {
            sorted_string.push(*c.0);
        }
    }

    return sorted_string;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = frequency_sort(String::from("tree"));
        assert_eq!(result, String::from("eert"));
    }

    #[test]
    fn exmaple_two() {
        let result = frequency_sort(String::from("cccaaa"));
        assert!(result == String::from("aaaccc") || result == String::from("cccaaa"));
    }

    #[test]
    fn example_three() {
        let result = frequency_sort(String::from("Aabb"));
        assert!(result == String::from("bbaA") || result == String::from("bbAa"));
    }
}
