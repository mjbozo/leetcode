// LeetCode problem 1790: Check if One String Swap Can Make Strings Equal
// https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/description/

fn main() {
    let result = are_almost_equal("bank".into(), "kanb".into());
    println!("Result = {result}");
}

fn are_almost_equal(s1: String, s2: String) -> bool {
    let mut wrongs = 0;
    let mut left_wrong = 0;
    let mut right_wrong = 0;

    for (c1, c2) in s1.bytes().zip(s2.bytes()) {
        if c1 != c2 {
            if wrongs == 0 {
                left_wrong = c1;
                right_wrong = c2;
            } else if wrongs == 2 || (wrongs == 1 && (c1 != right_wrong || c2 != left_wrong)) {
                return false;
            }

            wrongs += 1;
        }
    }

    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = are_almost_equal("bank".into(), "kanb".into());
        assert!(result);
    }

    #[test]
    fn example_two() {
        let result = are_almost_equal("attack".into(), "defend".into());
        assert!(!result);
    }

    #[test]
    fn example_three() {
        let result = are_almost_equal("kelb".into(), "kelb".into());
        assert!(result);
    }
}
