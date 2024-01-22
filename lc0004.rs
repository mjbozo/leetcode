// LeetCode problem 0004: Median of Two Sorted Arrays
// https://leetcode.com/problems/median-of-two-sorted-arrays/description/

fn main() {
    let result = find_median_sorted_arrays(vec![1, 3], vec![2]);
    println!("Result = {result}");
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let total_size = nums1.len() + nums2.len();
    let final_index = if total_size % 2 == 0 { total_size / 2 } else { (total_size + 1) / 2 };
    let mut a_iter = nums1.iter();
    let mut b_iter = nums2.iter();

    let mut prev = None;
    let mut current = None;
    let mut a_value = a_iter.next();
    let mut b_value = b_iter.next();
    let mut iteration = 0;
    loop {
        match (a_value, b_value) {
            (Some(a), Some(b)) => {
                if a < b {
                    if current.is_some() {
                        prev = current;
                    }
                    current = Some(a);
                    a_value = a_iter.next();
                } else {
                    if current.is_some() {
                        prev = current;
                    }
                    current = Some(b);
                    b_value = b_iter.next();
                }
            },
            
            (Some(a), None) => {
                if current.is_some() {
                    prev = current;
                }
                current = Some(a);
                a_value = a_iter.next();
            },

            (None, Some(b)) => {
                if current.is_some() {
                    prev = current;
                }
                current = Some(b);
                b_value = b_iter.next();
            }

            (None, None) => break
        }


        if iteration == final_index {
            break;
        }

        iteration += 1;
    }
    
    let result = if total_size % 2 == 0 {
        (*prev.unwrap_or(&0) as f64 + *current.unwrap_or(&0) as f64) / 2.0
    } else {
        *prev.unwrap_or(current.unwrap_or(&0)) as f64
    };

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn example_two() {
        let result = find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        assert_eq!(result, 2.5);
    }

    #[test]
    fn example_three() {
        let result = find_median_sorted_arrays(vec![], vec![1]);
        assert_eq!(result, 1.0);
    }
}
