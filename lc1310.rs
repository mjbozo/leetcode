// LeetCode problem 1310: XOR Queries of a Subarray
// https://leetcode.com/problems/xor-queries-of-a-subarray/description/

fn main() {
    let result = xor_queries(
        vec![1, 3, 4, 8],
        vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]],
    );

    println!("Result = {result:?}");
}

fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let arr_len = arr.len();

    // generate prefix xor array
    let mut prefix_xor: Vec<i32> = Vec::with_capacity(arr_len);
    prefix_xor.push(arr[0]);
    for i in 1..arr_len {
        prefix_xor.push(prefix_xor[i - 1] ^ arr[i]);
    }

    let mut results: Vec<i32> = Vec::with_capacity(queries.len());
    for (i, query) in queries.iter().enumerate() {
        // index into prefix array
        results.push(prefix_xor[query[1] as usize]);

        // xor with preceeding result to isolate range
        if query[0] != 0 {
            results[i] ^= prefix_xor[query[0] as usize - 1];
        }
    }

    return results;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = xor_queries(
            vec![1, 3, 4, 8],
            vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]],
        );
        assert_eq!(result, vec![2, 7, 14, 8]);
    }

    #[test]
    fn example_two() {
        let result = xor_queries(
            vec![4, 8, 2, 10],
            vec![vec![2, 3], vec![1, 3], vec![0, 0], vec![0, 3]],
        );
        assert_eq!(result, vec![8, 0, 4, 4]);
    }
}
