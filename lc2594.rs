// LeetCode problem 2594: Minimum Time to Repair Cars
// https://leetcode.com/problems/minimum-time-to-repair-cars/description/

fn main() {
    let result = repair_cars(vec![4, 2, 3, 1], 10);
    println!("Result = {result}");
}

fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
    let mut left = 1;
    let mut right = *ranks.iter().min().unwrap() as i64 * cars as i64 * cars as i64;
    let mut result = 0;

    while left <= right {
        let mid = left + (right - left) / 2;
        if can_repair(&ranks, mid, cars) {
            right = mid - 1;
            result = mid;
        } else {
            left = mid + 1;
        }
    }
    return result;
}

fn can_repair(arr: &Vec<i32>, time: i64, cars: i32) -> bool {
    let mut repairs = 0;
    for &num in arr {
        repairs += ((time / num as i64) as f64).sqrt().floor() as i32;
        if repairs >= cars {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = repair_cars(vec![4, 2, 3, 1], 10);
        assert_eq!(result, 16);
    }

    #[test]
    fn example_two() {
        let result = repair_cars(vec![5, 1, 8], 6);
        assert_eq!(result, 16);
    }
}
