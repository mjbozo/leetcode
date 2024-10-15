// LeetCode problem 1381: Design a Stack With Increment Operation
// https://leetcode.com/problems/design-a-stack-with-increment-operation/description/

#[derive(Debug)]
struct CustomStack {
    values: Vec<i32>,
}

impl CustomStack {
    fn new(max_size: i32) -> Self {
        return CustomStack {
            values: Vec::with_capacity(max_size as usize),
        };
    }

    fn push(&mut self, x: i32) {
        if self.values.len() == self.values.capacity() {
            return;
        }

        self.values.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.values.is_empty() {
            return -1;
        }

        let popped = self.values.remove(self.values.len() - 1);
        return popped;
    }

    fn increment(&mut self, k: i32, val: i32) {
        for i in 0..k {
            let i = i as usize;
            if i >= self.values.len() {
                break;
            }

            self.values[i] += val;
        }
    }
}

fn main() {
    let mut stack = CustomStack::new(3);
    stack.push(1);
    stack.push(2);
    println!("{:?}", stack);

    println!("{}", stack.pop());
    println!("{:?}", stack);

    stack.push(2);
    stack.push(3);
    stack.push(4);
    println!("{:?}", stack);

    stack.increment(5, 100);
    println!("{:?}", stack);

    stack.increment(2, 100);
    println!("{:?}", stack);

    println!("{}", stack.pop());
    println!("{}", stack.pop());
    println!("{}", stack.pop());
    println!("{}", stack.pop());
}
