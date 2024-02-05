// LeetCode problem 0011: Containe With Most Water
// https://leetcode.com/problems/continer-with-most-water/description/

use std::cmp;

fn main() {
    let result = max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
    println!("Result = {result}");
}

fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut maximum = 0;

    while left < right {
        let area = (right - left) as i32 * cmp::min(height[left], height[right]);
        if area > maximum {
            maximum = area;
        }

        if height[left] < height[right] {
            let mut offset = 1;
            while height[left + offset] <= height[left] {
                offset += 1;
            }
            left += offset;
        } else {
            let mut offset = 1;
            while height[right - offset] <= height[right] && right > left && right > offset {
                offset += 1;
            }
            right -= offset;
        }
    }

    return maximum;
}

// fn max_area(height: Vec<i32>) -> i32 {
//     let mut maximum = 0;
//     let mut prev_left_height = 0;
//     for left in 0..(height.len() - 1) {
//         if height[left] <= prev_left_height {
//             continue;
//         }

//         let mut prev_right_height = 0;
//         for right in ((left + 1)..height.len()).rev() {
//             if height[right] <= prev_right_height {
//                 continue;
//             }

//             let area = (right - left) as i32 * cmp::min(height[left], height[right]);
//             if area > maximum {
//                 maximum = area;
//             }
//             prev_right_height = height[right];
//         }

//         prev_left_height = height[left];
//     }
    
//     return maximum as i32;
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        assert_eq!(result, 49);
    }

    #[test]
    fn example_two() {
        let result = max_area(vec![1, 1]);
        assert_eq!(result, 1);
    }
}