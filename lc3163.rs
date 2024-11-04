// LeetCode problem 3163: String Compression III
// https://leetcode.com/problems/string-compression-iii/description/

fn main() {
    let result = compressed_string("abcde".into());
    println!("Result = {result}");
}

fn compressed_string(word: String) -> String {
    let mut current = (0, 0);
    let mut comp = String::new();

    for c in word.as_bytes() {
        if *c == current.0 {
            current.1 += 1;

            if current.1 == 9 {
                comp += &format!("9{}", current.0 as char);
                current = (0, 0);
            }
        } else {
            if current.1 > 0 {
                comp += &format!("{}{}", current.1, current.0 as char);
            }

            current = (*c, 1);
        }
    }

    if current.1 > 0 {
        comp += &format!("{}{}", current.1, current.0 as char);
    }

    return comp;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = compressed_string("abcde".into());
        assert_eq!(result, String::from("1a1b1c1d1e"));
    }

    #[test]
    fn example_two() {
        let result = compressed_string("aaaaaaaaaaaaaabb".into());
        assert_eq!(result, String::from("9a5a2b"));
    }
}
