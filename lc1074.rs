// LeetCode problem 1074: Numer of Submatrices That Sum to Target
// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/description/

fn main() {
    let input = vec![vec![0, 1, 1, 1, 0, 1], vec![0, 0, 0, 0, 0, 1], vec![0, 0, 1, 0, 0, 1], vec![1, 1, 0, 1, 1, 0], vec![1, 0, 0, 1, 0, 0]];
    let result = num_submatrix_sum_target(input, 0);
    println!("Result = {result}");
}

fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
    let width = matrix[0].len();
    let height = matrix.len();

    let mut count = 0;

    for x0 in 0..width {
        for x1 in x0..width {
            for y0 in 0..height {
                for y1 in y0..height {
                    let m: i32 = matrix[y0..=y1].iter().map(|r| {
                        let s: i32 = r[x0..=x1].iter().sum();
                        s   
                    }).sum();
                    if m == target {
                        count += 1;
                    }
                }
            }
        }
    }
    
    return count;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]];
        let result = num_submatrix_sum_target(input, 0);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_two() {
        let input = vec![vec![1, -1], vec![-1, 1]];
        let result = num_submatrix_sum_target(input, 0);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_three() {
        let input = vec![vec![904]];
        let result = num_submatrix_sum_target(input, 0);
        assert_eq!(result, 0);
    }
}
