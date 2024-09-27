// LeetCode problem 0731: My Calendar II
// https://leetcode.com/problems/my-calendar-ii/description/

use std::ops::Range;

trait Overlap {
    fn overlaps(&self, other: &Self) -> bool;
    fn intersection(&self, other: &Self) -> Option<Range<i32>>;
}

struct MyCalendarTwo {
    bookings: Vec<Range<i32>>,
    doubles: Vec<Range<i32>>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        return MyCalendarTwo {
            bookings: vec![],
            doubles: vec![],
        };
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for double in &self.doubles {
            if double.overlaps(&(start..end)) {
                return false;
            }
        }

        for booking in &self.bookings {
            if let Some(overlap) = booking.intersection(&(start..end)) {
                self.doubles.push(overlap);
            }
        }

        self.bookings.push(start..end);
        return true;
    }
}

impl Overlap for Range<i32> {
    fn overlaps(&self, other: &Self) -> bool {
        return self.start < other.end && other.start < self.end;
    }

    fn intersection(&self, other: &Self) -> Option<Range<i32>> {
        if !self.overlaps(other) {
            return None;
        }

        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        return Some(start..end);
    }
}
