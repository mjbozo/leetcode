// LeetCode problem 0921: Minimum Add to Make Parentheses Valid
// https://leetcode.com/problems/minimum-add-to-make-parenthese-valid/description/

fn main() {
    let result = min_add_to_make_valid("())".into());
    println!("Result = {result}");
}

fn min_add_to_make_valid(s: String) -> i32 {
    let mut open = 0;
    let mut unmatched = 0;

    for c in s.chars() {
        if c == '(' {
            open += 1;
        } else if open > 0 {
            open -= 1;
        } else {
            unmatched += 1;
        }
    }

    return unmatched + open;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = min_add_to_make_valid("())".into());
        assert_eq!(result, 1);
    }

    #[test]
    fn example_two() {
        let result = min_add_to_make_valid("(((".into());
        assert_eq!(result, 3);
    }
}
