// LeetCode problem 1405: Longest Happy String
// https://leetcode.com/problems/longest-happy-string/description/

fn main() {
    let result = longest_diverse_string(1, 1, 7);
    println!("Result = {result}");
}

// how the fuck was this shit accepted
fn longest_diverse_string(mut a: i32, mut b: i32, mut c: i32) -> String {
    let mut next;

    if a > b && a > c {
        next = "a";
    } else if b > a && b > c {
        next = "b";
    } else {
        next = "c";
    }

    let mut longest = String::new();

    loop {
        match next {
            "a" => {
                longest += "a";
                a -= 1;
                if a > b && a > c && a > 0 {
                    longest += "a";
                    a -= 1;
                }

                if b >= c && b > 0 {
                    next = "b";
                } else if c > 0 {
                    next = "c";
                } else {
                    break;
                }
            }
            "b" => {
                longest += "b";
                b -= 1;
                if b > a && b > c && b > 0 {
                    longest += "b";
                    b -= 1;
                }

                if a >= c && a > 0 {
                    next = "a";
                } else if c > 0 {
                    next = "c";
                } else {
                    break;
                }
            }
            "c" => {
                longest += "c";
                c -= 1;
                if c > a && c > b && c > 0 {
                    longest += "c";
                    c -= 1;
                }

                if a >= b && a > 0 {
                    next = "a";
                } else if b > 0 {
                    next = "b";
                } else {
                    break;
                }
            }
            _ => {}
        };
    }

    return longest;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = longest_diverse_string(1, 1, 7);
        assert_eq!(result, String::from("ccaccbcc"));
    }

    #[test]
    fn example_two() {
        let result = longest_diverse_string(7, 1, 0);
        assert_eq!(result, String::from("aabaa"));
    }
}
