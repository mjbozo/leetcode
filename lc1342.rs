// LeetCode problem 1342: Number of Steps to Reduce a Number to Zero
// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/description/

fn main() {
    let result = number_of_steps(14);
    println!("Result = {result}");
}

fn number_of_steps(num: i32) -> i32 {
    let mut num = num;
    let mut steps = 0;
    while num != 0 {
        if num % 2 == 0 {
            steps += 1;
            num = num >> 1;
        } else if num == 1 {
            steps += 1;
            num = 0;
        } else {
            steps += 2;
            num = (num - 1) >> 1;
        }
    }
    return steps;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = number_of_steps(14);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_two() {
        let result = number_of_steps(8);
        assert_eq!(result, 4);
    }
}
