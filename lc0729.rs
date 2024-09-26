// LeetCode problem 0729: My Calendar I
// https://leetcode.com/problems/my-calendar-i/description/

use std::ops::Range;

struct MyCalendar {
    bookings: Vec<Range<i32>>,
}

trait Overlap {
    fn overlaps(&self, other: Self) -> bool;
}

impl Overlap for MyCalendar {
    fn overlaps(&self, other: Self) -> bool {
        return self.start < other.end && other.start < self.end;
    }
}

impl MyCalendar {
    fn new() -> Self {
        return MyCalendar { bookings: vec![] };
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for booking in &self.bookings {
            if booking.overlaps(start..end) {
                return false;
            }
        }

        self.bookings.push(start..end);
        return true;
    }
}
