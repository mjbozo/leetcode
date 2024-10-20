// LeetCode problem 1106: Parsing a Boolean Expression
// https://leetcode.com/problems/parsing-a-boolean-expression/description/

fn main() {
    let result = parse_bool_expr(String::from("&(|(f,f,f,&(t,t,!(t)),t),|(t),|(f,f,!(t)))"));
    println!("Result = {result}");
}

fn parse_bool_expr(expression: String) -> bool {
    let mut chars = expression.chars();
    return match chars.nth(0).unwrap() {
        't' => true,
        'f' => false,
        '&' => eval_and_subexpr(expression[2..].to_owned()),
        '|' => eval_or_subexpr(expression[2..].to_owned()),
        '!' => invert_subexpr(expression[2..].to_owned()),
        _ => panic!("invalid starting char"),
    };
}

fn eval_and_subexpr(subexpr: String) -> bool {
    let mut skip = false;
    let mut counter = 0;
    for (i, c) in subexpr.chars().enumerate() {
        if skip {
            if c == '(' {
                counter += 1;
            } else if c == ')' {
                counter -= 1;
            }

            if counter == 0 {
                skip = false
            }

            continue;
        }

        match c {
            'f' => return false,
            '&' => {
                if !eval_and_subexpr(subexpr[i + 2..].to_owned()) {
                    return false;
                }
                skip = true;
            }
            '|' => {
                if !eval_or_subexpr(subexpr[i + 2..].to_owned()) {
                    return false;
                }
                skip = true;
            }
            '!' => {
                if !invert_subexpr(subexpr[i + 2..].to_owned()) {
                    return false;
                }
                skip = true;
            }
            ')' => {
                return true;
            }
            _ => {}
        }
    }

    return true;
}

fn eval_or_subexpr(subexpr: String) -> bool {
    let mut skip = false;
    let mut counter = 0;

    for (i, c) in subexpr.chars().enumerate() {
        if skip {
            if c == '(' {
                counter += 1;
            } else if c == ')' {
                counter -= 1;
            }

            if counter == 0 {
                skip = false
            }

            continue;
        }

        match c {
            't' => return true,
            '&' => {
                if eval_and_subexpr(subexpr[i + 2..].to_owned()) {
                    return true;
                }
                skip = true;
            }
            '|' => {
                if eval_or_subexpr(subexpr[i + 2..].to_owned()) {
                    return true;
                }
                skip = true;
            }
            '!' => {
                if invert_subexpr(subexpr[i + 2..].to_owned()) {
                    return true;
                }
                skip = true;
            }
            ')' => {
                return false;
            }
            _ => {}
        }
    }

    return false;
}

fn invert_subexpr(subexpr: String) -> bool {
    return !parse_bool_expr(subexpr);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = parse_bool_expr(String::from("&(|(f))"));
        assert_eq!(result, false)
    }

    #[test]
    fn example_two() {
        let result = parse_bool_expr(String::from("|(f, f, f, t)"));
        assert_eq!(result, true);
    }

    #[test]
    fn example_three() {
        let result = parse_bool_expr(String::from("!(&(f, t))"));
        assert_eq!(result, true);
    }
}
