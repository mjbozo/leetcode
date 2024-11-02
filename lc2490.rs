// LeetCode problem 2490: Circular Sentence
// https://leetcode.com/problems/circular-sentence/description/

fn main() {
    let result = is_circular_sentence("leetcode exercises sound delightful".into());
    println!("Result = {result}");
}

fn is_circular_sentence(sentence: String) -> bool {
    let bytes = sentence.as_bytes();
    for w in bytes.windows(3) {
        if w[1] == b' ' && w[0] != w[2] {
            return false;
        }
    }

    return bytes[0] == bytes[sentence.len() - 1];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = is_circular_sentence("leetcode exercises sound delightful".into());
        assert!(result);
    }

    #[test]
    fn example_two() {
        let result = is_circular_sentence("eetcode".into());
        assert!(result);
    }

    #[test]
    fn example_three() {
        let result = is_circular_sentence("Leetcode is cool".into());
        assert!(!result);
    }
}
