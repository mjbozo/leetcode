// LeetCode problem 0010: Regular Expression Matching
// https://leetcode.com/problems/regular-expression-matching/description/

fn main() {
    let result = is_match(String::from("aaa"), String::from("a*a"));
    println!("Result = {result}");
}

fn is_match(s: String, p: String) -> bool {
    let s_vec = s.chars().collect::<Vec<char>>();
    let p_vec = p.chars().collect::<Vec<char>>();
    let mut s_index = 0;
    for (p_index, c) in p_vec.iter().enumerate() {
        if c.is_ascii_alphabetic() {
            if let Some(x) = p_vec.get(p_index + 1) {
                if *x == '*' {
                    while s_vec.get(s_index) == Some(c) {
                        s_index += 1;
                    }
                    continue;
                }
            }
            
            if s_vec.get(s_index) == Some(c) {
                s_index += 1;
                continue;
            } else {
                return false;
            }
        }

        if *c == '.' {
            if let Some(x) = p_vec.get(p_index + 1) {
                if *x == '*' {
                    if let Some(y) = p_vec.get(p_index + 2) {
                        while s_vec.get(s_index) != Some(y) && s_vec.get(s_index) != None {
                            s_index += 1;
                        }
                    } else {
                        return true;
                    }
                }
            }

            s_index += 1;
        }
    }

    if s_index < s.len() {
        return false;
    }

    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = is_match(String::from("aa"), String::from("a"));
        assert!(!result);
    }

    #[test]
    fn example_two() {
        let result = is_match(String::from("aa"), String::from("a*"));
        assert!(result);
    }

    #[test]
    fn example_three() {
        let result = is_match(String::from("ab"), String::from(".*"));
        assert!(result);
    }
}
