// LeetCode problem 1072: Flip Columns For Maximum Number of Equal Rows
// https://leetcode.com/problems/flip-columns-for-maximum-number-of-equal-rows/description/

fn main() {
    let result = max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 1]]);
    println!("Result = {result}");
}

fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
    // try set each row to all zeroes, test for max
    let mut maximum = 0;

    for row in matrix.iter() {
        let mut change_rows = vec![false; row.len()];
        for (idx, element) in row.iter().enumerate() {
            if *element == 1 {
                change_rows[idx] = true;
            }
        }

        let consistent_rows = test_change(matrix.clone(), &change_rows);
        maximum = std::cmp::max(maximum, consistent_rows);
    }

    return maximum;
}

fn test_change(mut matrix: Vec<Vec<i32>>, idxs: &Vec<bool>) -> i32 {
    let mut rows_matching = 0;

    for row in matrix.iter_mut() {
        let mut all_match = true;
        if idxs[0] {
            row[0] ^= 1;
        }

        for i in 1..row.len() {
            if idxs[i] {
                row[i] ^= 1;
            }

            if row[i] != row[i - 1] {
                all_match = false;
                break;
            }
        }

        if all_match {
            rows_matching += 1;
        }
    }

    return rows_matching;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 1]]);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_two() {
        let result = max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 0]]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_three() {
        let result = max_equal_rows_after_flips(vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]);
        assert_eq!(result, 2);
    }
}
