// LeetCode problem 0049: Group Anagrams
// https://leetcode.com/problems/group-anagrams/description/

fn main() {
    let result = group_anagrams(build_string_vec(vec!["eat", "tea", "tan", "ate", "nat", "bat"]));
    println!("Result = {result:?}");
}

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: Vec<Vec<String>> = vec![];
    let mut mapped = strs.iter().map(|s| {
        let mut char_counts = vec![0; 26];
        for c in s.bytes() {
            char_counts[(c - b'a') as usize] += 1;
        }
        (s, char_counts)
    }).collect::<Vec<_>>();
    mapped.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut prev = vec![0; 26];
    let mut group = vec![];
    for s in mapped {
        if s.1 == prev {
            group.push(s.0.to_string());
        } else {
            if group.len() > 0 {
                groups.push(group);
            }
            group = vec![s.0.to_string()];
        }
        prev = s.1;
    }

    if group.len() > 0 {
        groups.push(group);
    }

    return groups;
}

fn build_string_vec(strs: Vec<&str>) -> Vec<String> {
    let mut strings = vec![];
    for s in strs {
        strings.push(s.to_string());
    }
    return strings;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = group_anagrams(build_string_vec(vec!["eat", "tea", "tan", "ate", "nat", "bat"]));
        assert!(result.contains(&vec![String::from("bat")]));
        assert!(result.contains(&vec![String::from("tan"), String::from("nat")]));
        assert!(result.contains(&vec![String::from("eat"), String::from("tea"), String::from("ate")]));
    }

    #[test]
    fn example_two() {
        let result = group_anagrams(vec![String::new()]);
        assert_eq!(result, vec![vec![String::new()]]);
    }

    #[test]
    fn example_three() {
        let result = group_anagrams(vec![String::from("a")]);
        assert_eq!(result, vec![vec![String::from("a")]]);
    }
}
