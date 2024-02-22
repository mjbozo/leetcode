// LeetCode problem 0038: Count and Say
// https://leetcode.com/problems/count-and-say/description/

fn main() {
    let result = count_and_say(8);
    println!("Result = {result}");
}

fn count_and_say(n: i32) -> String {
    let mut dp = vec![String::from("0"), String::from("1")];
    for i in 2..=n {
        let previous = &dp[i as usize - 1];
        let mut current = String::new();

        let mut prev_char = None;
        let mut char_count = 0;
        for c in previous.chars() {
            if prev_char.is_none() {
                char_count = 1;
                prev_char = Some(c);
            } else {
                if Some(c) == prev_char {
                    char_count += 1;
                } else {
                    current.push_str(format!("{char_count}{}", prev_char.unwrap()).as_str());
                    char_count = 1;
                    prev_char = Some(c);
                }
            }
        }

        if char_count > 0 {
            current.push_str(format!("{char_count}{}", prev_char.unwrap()).as_str());
        }

        dp.push(current);
    }

    return dp[dp.len() - 1].clone();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = count_and_say(1);
        assert_eq!(result, String::from("1"))
    }

    #[test]
    fn example_two() {
        let result = count_and_say(4);
        assert_eq!(result, String::from("1211"));
    }
}
