// LeetCode problem 0412: Fizz Buz
// https://leetcode.com/problems/fizz-buzz/description/

fn main() {
    let result = fizz_buzz(15);
    println!("Result = {result:?}");
}

fn fizz_buzz(n: i32) -> Vec<String> {
    let mut answer = Vec::with_capacity(n as usize);
    for i in 1..=n {
        match i {
            x if x % 3 == 0 && x % 5 == 0 => answer.push(String::from("FizzBuzz")),
            x if x % 3 == 0 => answer.push(String::from("Fizz")),
            x if x % 5 == 0 => answer.push(String::from("Buzz")),
            _ => answer.push(format!("{i}")),
        };
    }
    return answer;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = fizz_buzz(3);
        assert_eq!(result, vec!["1", "2", "Fizz"]);
    }

    #[test]
    fn example_two() {
        let result = fizz_buzz(5);
        assert_eq!(result, vec!["1", "2", "Fizz", "4", "Buzz"]);
    }

    #[test]
    fn example_three() {
        let result = fizz_buzz(15);
        assert_eq!(
            result,
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}
