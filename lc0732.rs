// LeetCode problem 0732: My Calendar III
// https://leetcode.com/problems/my-calendar-iii/description/

use std::collections::HashMap;

struct MyCalendarThree {
    line: HashMap<i32, i32>,
}

impl MyCalendarThree {
    fn new() -> Self {
        return MyCalendarThree {
            line: HashMap::<i32, i32>::new(),
        };
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        self.line
            .entry(start_time)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        self.line
            .entry(end_time)
            .and_modify(|e| *e -= 1)
            .or_insert(-1);

        let mut k = 0;
        let mut max = 0;

        let mut vec_rep: Vec<(&i32, &i32)> = self.line.iter().collect();
        vec_rep.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        for (_, value) in vec_rep {
            k += value;
            max = max.max(k);
        }

        return max;
    }
}

fn main() {
    let mut calendar = MyCalendarThree::new();
    let mut results = vec![];
    results.push(calendar.book(10, 20));
    results.push(calendar.book(50, 60));
    results.push(calendar.book(10, 40));
    results.push(calendar.book(5, 15));
    results.push(calendar.book(5, 10));
    results.push(calendar.book(25, 55));
    println!("Result = {results:?}");
}
