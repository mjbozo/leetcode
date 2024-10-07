// LeetCode problem 1813: Sentence Similarity III
// https://leetcode.com/problems/sentence-similarity-iii/description/

fn main() {
    let result = are_sentences_similar("Frog cool".into(), "Frogs are cool".into());
    println!("Result = {result}");
}

fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
    let mut words1 = sentence1.split_whitespace().collect::<Vec<_>>();
    let mut words2 = sentence2.split_whitespace().collect::<Vec<_>>();

    if words1.len() > words2.len() {
        words2 = sentence1.split_whitespace().collect::<Vec<_>>();
        words1 = sentence2.split_whitespace().collect::<Vec<_>>();
    }

    let mut longest_prefix = 0;
    let mut longest_suffix = 0;

    while words1[longest_prefix] == words2[longest_prefix] {
        longest_prefix += 1;
        if longest_prefix == words1.len() {
            break;
        }
    }

    while words1[words1.len() - longest_suffix - 1] == words2[words2.len() - longest_suffix - 1] {
        longest_suffix += 1;
        if longest_suffix == words1.len() {
            break;
        }
    }

    return longest_prefix + longest_suffix >= words1.len();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = are_sentences_similar("My name is Haley".into(), "My Haley".into());
        assert_eq!(result, true);
    }

    #[test]
    fn example_two() {
        let result = are_sentences_similar("of".into(), "A lot of words".into());
        assert_eq!(result, false);
    }

    #[test]
    fn example_three() {
        let result = are_sentences_similar("Eating right now".into(), "Eating".into());
        assert_eq!(result, true);
    }

    #[test]
    fn example_four() {
        let result = are_sentences_similar("A".into(), "a A b A".into());
        assert_eq!(result, true);
    }
}
