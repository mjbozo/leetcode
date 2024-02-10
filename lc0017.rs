// LeetCode problem 0017: Letter Combinations of a Phone Number
// https://leetcode.com/problems/letter-combinations-of-a-phone-number/description/

fn main() {
    let result = letter_combinations(String::from("234"));
    println!("Result = {result:?}");
}

fn letter_combinations(digits: String) -> Vec<String> {
    let mut combinations = vec![];
    for digit in digits.bytes() {
        let chars = match digit {
            b'2' => vec!["a", "b", "c"],
            b'3' => vec!["d", "e", "f"],
            b'4' => vec!["g", "h", "i"],
            b'5' => vec!["j", "k", "l"],
            b'6' => vec!["m", "n", "o"],
            b'7' => vec!["p", "q", "r", "s"],
            b'8' => vec!["t", "u", "v"],
            b'9' => vec!["w", "x", "y", "z"],
            _ => vec![]
        };

        if combinations.len() == 0 {
            for c in chars {
                combinations.push(String::from(c));
            }
            continue;
        }

        let mut next_combinations = vec![];
        for combination in combinations {
            for c in &chars {
                let mut new_combo = combination.clone();
                new_combo.push_str(c);
                next_combinations.push(new_combo);
            }
        }
        combinations = next_combinations;
    }

    return combinations;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = letter_combinations(String::from("23"));
        assert_eq!(result, vec![
            String::from("ad"),
            String::from("ae"),
            String::from("af"),
            String::from("bd"),
            String::from("be"),
            String::from("bf"),
            String::from("cd"),
            String::from("ce"),
            String::from("cf"),
        ]);
    }

    #[test]
    fn example_two() {
        let result = letter_combinations(String::from(""));
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn example_three() {
        let result = letter_combinations(String::from("2"));
        assert_eq!(result, vec![String::from("a"), String::from("b"), String::from("c")]);
    }
}
