// LeetCode problem 0763: Partition Labels
// https://leetcode.com/problems/partition-labels/description/

use std::collections::HashMap;

fn main() {
    let result = partition_labels(String::from("ababcbacadefegdehijhklij"));
    println!("Result = {result:?}");
}

fn partition_labels(s: String) -> Vec<i32> {
    let mut highest = HashMap::new();
    let sb = s.as_bytes();
    let n = sb.len();

    for i in 0..n {
        let b = sb[i];
        highest.entry(b).and_modify(|v| *v = i).or_insert(i);
    }

    let mut partitions = vec![];
    let mut previous_partition_sum = 0;
    let mut current_partition = -1;

    for i in 0..n {
        let b = sb[i];
        let x = *highest.get(&b).unwrap() as i32;
        if x > current_partition {
            current_partition = x;
        }
        if current_partition == i as i32 {
            let size = i as i32 - previous_partition_sum + 1;
            partitions.push(size);
            previous_partition_sum += size;
        }
    }

    return partitions;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let result = partition_labels(String::from("ababcbacadefegdehijhklij"));
        assert_eq!(result, vec![9, 7, 8]);
    }

    #[test]
    fn example_two() {
        let result = partition_labels(String::from("eccbbbbdec"));
        assert_eq!(result, vec![10]);
    }
}
