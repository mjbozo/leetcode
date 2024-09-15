// LeetCode problem 1371: Find the Longest Substring Containing Vowels in Even Counts
// https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/description/

// this one fucken sucks
// solution is not intuitive at all and it makes me upset

fn main() {
    let result = find_the_longest_substring(String::from("fedfghd"));
    println!("Result = {result}");
}

fn find_the_longest_substring(s: String) -> i32 {
    // vec of size 32 to fit all vowel bitmask combinations: 2^5 = 32
    // default to -2 as 'unseen' start location
    let mut mask_starts: Vec<i32> = vec![-2; 32];

    let mut bitmask = 0;
    let mut longest = 0;

    // handle case where solution begins at start of string
    mask_starts[0] = -1;

    for (idx, c) in s.as_bytes().iter().enumerate() {
        // set the bitmask based on the vowels seen
        match c {
            b'a' => bitmask ^= 1,
            b'e' => bitmask ^= 2,
            b'i' => bitmask ^= 4,
            b'o' => bitmask ^= 8,
            b'u' => bitmask ^= 16,
            _ => {}
        }

        if mask_starts[bitmask] == -2 {
            // first time seeing bitmask, set the starting index
            // first time seeing a bitmask must be odd number of vowels
            // so need to start counting from NEXT index when distance is calculated
            mask_starts[bitmask] = idx as i32;
        } else {
            // seen bitmask before, update longest if needed
            longest = std::cmp::max(longest, idx as i32 - mask_starts[bitmask]);
        }
    }

    return longest;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_the_longest_substring(String::from("eleetminicoworoep"));
        assert_eq!(result, 13);
    }

    #[test]
    fn example_two() {
        let result = find_the_longest_substring(String::from("leetcodeisgreat"));
        assert_eq!(result, 5);
    }

    #[test]
    fn example_three() {
        let result = find_the_longest_substring(String::from("bcbcbc"));
        assert_eq!(result, 6);
    }
}
