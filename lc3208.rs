// LeetCode problem 3208: Alternating Groups II
// https://leetcode.com/problems/alternative-groups-ii/description/

fn main() {
    let result = number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3);
    println!("Result = {result}");
}

fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
    let mut c = colors.iter().cycle();
    let mut result = 0;
    let mut current = 1;
    let mut prev = c.next();
    for _ in 1..(colors.len() + k as usize - 1) {
        let x = c.next();
        if x != prev {
            current += 1;
        } else {
            current = 1;
        }

        if current >= k {
            result += 1;
        }
        prev = x;
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_three() {
        let result = number_of_alternating_groups(vec![1, 1, 0, 1], 4);
        assert_eq!(result, 0);
    }
}
