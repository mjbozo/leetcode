// LeetCode problem 1574: Shortest Subarray to be Removed to Make Array Sorted
// https://leetcode.com/problems/shortest-subarray-to-be-removed-to-make-array-sorted/description/

fn main() {
    let result = find_length_of_shortest_subarray(vec![1, 2, 3, 10, 4, 2, 3, 5]);
    println!("Result = {result}");
}

fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
    let mut right = arr.len() - 1;
    while right > 0 && arr[right] >= arr[right - 1] {
        right -= 1;
    }

    let mut shortest = right;
    let mut left = 0;

    while left < right && (left == 0 || arr[left - 1] <= arr[left]) {
        while right < arr.len() && arr[left] > arr[right] {
            right += 1;
        }

        shortest = std::cmp::min(shortest, right - left - 1);
        left += 1;
    }

    return shortest as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_length_of_shortest_subarray(vec![1, 2, 3, 10, 4, 2, 3, 5]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_two() {
        let result = find_length_of_shortest_subarray(vec![5, 4, 3, 2, 1]);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_three() {
        let result = find_length_of_shortest_subarray(vec![1, 2, 3]);
        assert_eq!(result, 0);
    }
}
