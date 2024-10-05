// LeetCode problem 0567: Permutation in String
// https://leetcode.com/problems/permutation-in-string/description/

fn main() {
    let result = check_inclusion("adc".into(), "dcda".into());
    println!("Result = {result}");
}

fn check_inclusion(s1: String, s2: String) -> bool {
    let n1 = s1.len();
    let n2 = s2.len();
    if n1 > n2 {
        return false;
    }

    let mut set1 = vec![0; 26];
    let mut set2 = vec![0; 26];

    for x in s1.bytes() {
        set1[(x - b'a') as usize] += 1;
    }

    // prefill n1 letters in set2
    let s2_bytes = s2.bytes().collect::<Vec<_>>();
    for i in 0..n1 {
        set2[(s2_bytes[i] - b'a') as usize] += 1;
    }

    let mut left = 0;
    let mut right = n1 - 1;

    while right < n2 {
        if set1 == set2 {
            return true;
        }

        set2[(s2_bytes[left] - b'a') as usize] -= 1;

        left += 1;
        right += 1;

        if right == n2 {
            break;
        }

        set2[(s2_bytes[right] - b'a') as usize] += 1;
    }

    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = check_inclusion("ab".into(), "eidbaooo".into());
        assert_eq!(result, true);
    }

    #[test]
    fn example_two() {
        let result = check_inclusion("ab".into(), "eidboaoo".into());
        assert_eq!(result, false);
    }
}
