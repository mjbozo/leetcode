// LeetCode problem 0150: Evaluate Reverse Polish Notation
// https://leetcode.com/problems/evaluate-reverse-polish-notation/description/

fn main() {
    let tokens = stringify_vec(vec!["2", "1", "+", "3", "*"]);
    let result = eval_rpn(tokens);
    println!("Result = {result}");
}

fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];

    for token in tokens {
        match token.as_str() {
            "+" => {
                let right = stack.pop().expect("Stack is empty");
                let left = stack.pop().expect("Stack is empty");
                stack.push(left + right);
            }
            "-" => {
                let right = stack.pop().expect("Stack is empty");
                let left = stack.pop().expect("Stack is empty");
                stack.push(left - right);
            }
            "*" => {
                let right = stack.pop().expect("Stack is empty");
                let left = stack.pop().expect("Stack is empty");
                stack.push(left * right);
            }
            "/" => {
                let right = stack.pop().expect("Stack is empty");
                let left = stack.pop().expect("Stack is empty");
                stack.push(left / right);
            }
            _ => {
                let x = token.parse::<i32>().expect("{token} not a valid number");
                stack.push(x);
            }
        }
    }

    let result = stack.pop().expect("No result left in stack");
    return result;
}

fn stringify_vec(strs: Vec<&str>) -> Vec<String> {
    let mut output = vec![];
    for s in strs {
        output.push(String::from(s))
    }
    return output;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let tokens = stringify_vec(vec!["2", "1", "+", "3", "*"]);
        let result = eval_rpn(tokens);
        assert_eq!(result, 9);
    }

    #[test]
    fn example_two() {
        let tokens = stringify_vec(vec!["4", "13", "5", "/", "+"]);
        let result = eval_rpn(tokens);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_three() {
        let tokens = stringify_vec(vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]);
        let result = eval_rpn(tokens);
        assert_eq!(result, 22);
    }
}
