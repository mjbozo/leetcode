// LeetCode problem 2022: Convert 1D Array Into 2D Array
// https://leetcode.com/problems/convert-1d-array-into-2d-array/description/

fn main() {
    let result = construct2_d_array(vec![1, 2, 3, 4], 2, 2);
    println!("Result = {result:?}");
}

fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    let m = m as usize;
    let n = n as usize;

    if original.len() != m * n {
        return vec![];
    }

    return original.chunks(n).map(|c| c.to_vec()).collect();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = construct2_d_array(vec![1, 2, 3, 4], 2, 2);
        assert_eq!(result, vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn example_two() {
        let result = construct2_d_array(vec![1, 2, 3], 1, 3);
        assert_eq!(result, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn example_three() {
        let result = construct2_d_array(vec![1, 2], 1, 1);
        assert_eq!(result, vec![] as Vec<Vec<i32>>);
    }
}
