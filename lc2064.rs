// LeetCode problem 2064: Minimized Maximum of Products Distributed to Any Store
// https://leetcode.com/problems/minimized-maximum-of-products-distributed-to-any-store/description/

fn main() {
    let result = minimized_maximum(22, vec![25, 11, 29, 6, 24, 4, 29, 18, 6, 13, 25, 30]);
    println!("Result = {result}");
}

fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = *quantities.iter().max().unwrap_or(&0);

    while left < right {
        let middle = (left + right) / 2;
        if can_distribute(middle, &quantities, n) {
            right = middle;
        } else {
            left = middle + 1;
        }
    }

    return left;
}

fn can_distribute(x: i32, quantities: &Vec<i32>, n: i32) -> bool {
    let q = quantities.len();
    let mut j = 0;
    let mut remaining = quantities[j];

    for i in 0..n {
        if remaining <= x {
            j += 1;
            if j == q {
                return true;
            } else {
                remaining = quantities[j];
            }
        } else {
            remaining -= x;
        }
    }

    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = minimized_maximum(6, vec![11, 6]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = minimized_maximum(7, vec![15, 10, 10]);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_three() {
        let result = minimized_maximum(1, vec![100000]);
        assert_eq!(result, 100000);
    }
}
