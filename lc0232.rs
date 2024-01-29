// LeetCode problem 0232: Implement Queue using Stacks
// https://leetcode.com/problems/implement-queue-using-stacks/description/

use std::collections::VecDeque;

fn main () {
    let mut my_queue = MyQueue::new();
    my_queue.push(1);
    my_queue.push(2);
    let peek = my_queue.peek();
    println!("Peek = {peek}");

    let pop = my_queue.pop();
    println!("Pop = {pop}");

    let empty = my_queue.empty();
    println!("Empty = {empty}");
}

struct MyQueue {
    in_stack: VecDeque<i32>,
    out_stack: VecDeque<i32>
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            in_stack: VecDeque::new(),
            out_stack: VecDeque::new()
        }
    }

    fn push(&mut self, x: i32) {
        if self.in_stack.is_empty() {
            self.swap();
        }

        self.in_stack.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        if self.out_stack.is_empty() {
            self.swap();
        }

        let x = self.out_stack.pop_back().expect("Out stack is empty when attempting pop");
        return x;
    }

    fn peek(&mut self) -> i32 {
        if self.out_stack.is_empty() {
            self.swap();
        }

        let x = self.out_stack.back().expect("Out stack is empty when attempting pop");
        return *x;
    }

    fn empty(&self) -> bool {
        return self.in_stack.is_empty() && self.out_stack.is_empty();
    }

    fn swap(&mut self) {
        if self.in_stack.is_empty() {
            for _ in 0..self.out_stack.len() {
                let x = self.out_stack.pop_back().expect("In stack is empty when attempting swap");
                self.in_stack.push_back(x);
            }
        } else if self.out_stack.is_empty() {
            for _ in 0..self.in_stack.len() {
                let x = self.in_stack.pop_back().expect("Out stack is empty when attempting swap");
                self.out_stack.push_back(x);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let mut my_queue = MyQueue::new();
        my_queue.push(1);
        assert_eq!(my_queue.in_stack.len(), 1);
        assert_eq!(my_queue.in_stack[0], 1);
        
        my_queue.push(2);
        assert_eq!(my_queue.in_stack.len(), 2);
        assert_eq!(my_queue.in_stack[1], 2);
        
        let peek = my_queue.peek();
        assert_eq!(peek, 1);

        let pop = my_queue.pop();
        assert_eq!(pop, 1);
        
        let empty = my_queue.empty();
        assert!(!empty);
    }
}
