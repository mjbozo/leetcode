// LeetCode problem 2161: Partition Array According to Given Pivot
// https://leetcode.com/problems/partition-array-according-to-given-pivot/description/

fn main() {
    let result = pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10);
    println!("Result = {result:?}");
}

fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    // bit gross but ends up being faster than clean solution
    let mut less_count = 0;
    let mut pivot_count = 0;

    for num in nums.iter() {
        match num {
            x if *x < pivot => less_count += 1,
            x if *x == pivot => pivot_count += 1,
            _ => {}
        }
    }

    let n = nums.len();
    let mut res = vec![0; n];
    let mut less_offset = 0;
    let mut pivot_offset = 0;
    let mut more_offset = 0;

    for num in nums {
        match num {
            x if x < pivot => {
                res[less_offset] = num;
                less_offset += 1;
            }
            x if x == pivot => {
                res[less_count + pivot_offset] = num;
                pivot_offset += 1;
            }
            _ => {
                res[less_count + pivot_count + more_offset] = num;
                more_offset += 1;
            }
        }
    }

    return res;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10);
        assert_eq!(result, vec![9, 5, 3, 10, 10, 12, 14]);
    }

    #[test]
    fn example_two() {
        let result = pivot_array(vec![-3, 4, 3, 2], 2);
        assert_eq!(result, vec![-3, 2, 4, 3]);
    }
}
