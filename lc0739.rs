// LeetCode problem 0739: Daily Temperatures
// https://leetcode.com/problems/daily-temperatures/description/

fn main() {
    let result = daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]);
    println!("Result = {result:?}");
}

fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut answer: Vec<i32> = vec![0; temperatures.len()];
    let mut stack: Vec<i32> = vec![0];

    for i in 1..temperatures.len() {
        while let Some(&t) = stack.last() {
            if temperatures[t as usize] < temperatures[i] {
                let index = stack.pop().expect("Stack should not be empty");
                answer[index as usize] = i as i32 - t;
            } else {
                break;
            }
        }

        stack.push(i as i32);
    }

    return answer;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]);
        assert_eq!(result, vec![1, 1, 4, 2, 1, 1, 0, 0]);
    }

    #[test]
    fn example_two() {
        let result = daily_temperatures(vec![30, 40, 50, 60]);
        assert_eq!(result, vec![1, 1, 1, 0]);
    }

    #[test]
    fn example_three() {
        let result = daily_temperatures(vec![30, 60, 90]);
        assert_eq!(result, vec![1, 1, 0]);
    }
}
