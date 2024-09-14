// LeetCode problem 1894: Find the Student that Will Replace the Chalk
// https://leetcode.com/problems/find-the-student-that-will-replace-the-chalk/description/

fn main() {
    let result = chalk_replacer(vec![5, 1, 5], 22);
    println!("Result = {result}");
}

fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let chalk: Vec<usize> = chalk.iter().map(|&x| x as usize).collect();
    let chalk_sum: usize = chalk.iter().sum();
    let mut remaining = k % chalk_sum;

    for (idx, &student) in chalk.iter().enumerate() {
        if remaining < student {
            return idx as i32;
        }
        remaining -= student;
    }

    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = chalk_replacer(vec![5, 1, 5], 22);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_two() {
        let result = chalk_replacer(vec![3, 4, 1, 2], 25);
        assert_eq!(result, 1);
    }
}
