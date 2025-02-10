// LeetCode problem 2349: Design a Number Container System
// https://leetcode.com/problems/design-a-number-container-system/description/

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let mut nc = NumberContainers::new();
    nc.change(1, 10);
    println!("{:?}", nc);
    nc.change(1, 10);
    println!("{:?}", nc);
    println!("{}", nc.find(10));
}

#[derive(Debug)]
struct NumberContainers {
    number_map: HashMap<i32, BinaryHeap<Reverse<i32>>>,
    index_map: HashMap<i32, i32>,
}

impl NumberContainers {
    fn new() -> Self {
        return NumberContainers {
            number_map: HashMap::new(),
            index_map: HashMap::new(),
        };
    }

    fn change(&mut self, index: i32, number: i32) {
        match self.index_map.get(&index) {
            Some(x) => {
                self.number_map
                    .get_mut(&x)
                    .unwrap()
                    .retain(|v| *v != Reverse(index));
            }
            _ => {}
        };

        self.number_map
            .entry(number)
            .and_modify(|v| (*v).push(Reverse(index)))
            .or_insert(BinaryHeap::from(vec![Reverse(index)]));

        self.index_map
            .entry(index)
            .and_modify(|v| *v = number)
            .or_insert(number);
    }

    fn find(&mut self, number: i32) -> i32 {
        return match self.number_map.get(&number) {
            Some(x) => x.peek().unwrap_or(&Reverse(-1)).0,
            None => -1,
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let nc = NumberContainers::new();
        assert_eq!(nc.find(10), -1);
        nc.change(2, 10);
        nc.change(1, 10);
        nc.change(3, 10);
        nc.change(5, 10);
        assert_eq!(nc.find(10), 1);
        nc.change(1, 20);
        assert_eq!(nc.find(10), 2);
    }
}
