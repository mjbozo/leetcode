// LeetCode problem 0383: Ransom Note
// https://leetcode.com/problems/ransom-note/description/

fn main() {
    let result = can_construct(String::from("a"), String::from("b"));
    println!("Result = {result}");
}

fn can_construct(ransom_note: String, magazine: String) -> bool {
    if ransom_note.len() > magazine.len() {
        return false;
    }

    let mut ransom_letters = vec![0; 26];
    for b in (ransom_note).bytes() {
        ransom_letters[(b - b'a') as usize] += 1;
    }

    let mut magazine_letters = vec![0; 26];
    for b in magazine.bytes() {
        magazine_letters[(b - b'a') as usize] += 1;
    }

    for (r, m) in ransom_letters.iter().zip(&magazine_letters) {
        if r > m {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = can_construct(String::from("a"), String::from("b"));
        assert!(!result);
    }

    #[test]
    fn example_two() {
        let result = can_construct(String::from("aa"), String::from("ab"));
        assert!(!result);
    }

    #[test]
    fn example_one() {
        let result = can_construct(String::from("aa"), String::from("aab"));
        assert!(result);
    }
}
