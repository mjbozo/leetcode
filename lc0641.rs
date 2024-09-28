// LeetCode problem 0641: Design Circular Deque
// https://leetcode.com/problems/design-circular-deque/description/

struct MyCircularDeque {
    values: Vec<i32>,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        return MyCircularDeque {
            values: Vec::with_capacity(k as usize),
        };
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.values.len() >= self.values.capacity() {
            return false;
        }

        self.values.insert(0, value);
        return true;
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.values.len() >= self.values.capacity() {
            return false;
        }

        self.values.push(value);
        return true;
    }

    fn delete_front(&mut self) -> bool {
        if self.values.is_empty() {
            return false;
        }

        self.values.remove(0);
        return true;
    }

    fn delete_last(&mut self) -> bool {
        if self.values.is_empty() {
            return false;
        }

        self.values.remove(self.values.len() - 1);
        return true;
    }

    fn get_front(&self) -> i32 {
        if self.values.is_empty() {
            return -1;
        }

        return self.values[0];
    }

    fn get_rear(&self) -> i32 {
        if self.values.is_empty() {
            return -1;
        }

        return self.values[self.values.len() - 1];
    }

    fn is_empty(&self) -> bool {
        return self.values.is_empty();
    }

    fn is_full(&self) -> bool {
        return self.values.len() == self.values.capacity();
    }
}

fn main() {
    let mut custom_deque = MyCircularDeque::new(3);
    println!("{}", custom_deque.insert_last(1));
    println!("{}", custom_deque.insert_last(2));
    println!("{}", custom_deque.insert_front(3));
    println!("{}", custom_deque.insert_front(4));
    println!("{}", custom_deque.get_rear());
    println!("{}", custom_deque.is_full());
    println!("{}", custom_deque.delete_last());
    println!("{}", custom_deque.insert_front(4));
    println!("{}", custom_deque.get_front());
    println!("{}", custom_deque.is_empty());
    println!("{}", custom_deque.delete_front());
    println!("{}", custom_deque.get_front());
}
