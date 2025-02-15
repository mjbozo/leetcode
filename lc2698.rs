// LeetCode problem 2698: Find the Punishment Number of an Integer
// https://leetcode.com/problems/find-the-punishment-number-of-an-integer/description/

fn main() {
    let result = punishment_number(10);
    println!("Result = {result}");
}

fn punishment_number(n: i32) -> i32 {
    // used recursive algorithm to generate these numbers ahead of time
    let good_nums = vec![
        1, 9, 10, 36, 45, 55, 82, 91, 99, 100, 235, 297, 369, 370, 379, 414, 657, 675, 703, 756,
        792, 909, 918, 945, 964, 990, 991, 999, 1000,
    ];

    let mut sum = 0;
    for num in good_nums {
        if num > n {
            break;
        }

        sum += num * num;
    }

    return sum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = punishment_number(10);
        assert_eq!(result, 182);
    }

    #[test]
    fn example_two() {
        let result = punishment_number(37);
        assert_eq!(result, 1478);
    }
}
