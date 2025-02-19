/*
    Merge Intervals
    Given an array of intervals where each interval is represented by a pair of integers [start, end], 
    merge all overlapping intervals and return a list of non-overlapping intervals.
    
    The intervals are inclusive, meaning the interval [start, end] includes both start and end points.
    
    You need to implement the function `merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
    The function should return a vector containing all the merged intervals.

    Hint: You can start by sorting the intervals by their starting point and then merge them one by one.
*/

use std::fmt::{self, Display, Formatter};

pub fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
        return Vec::new();
    }

    let mut sorted_intervals = intervals.clone();
    sorted_intervals.sort_by_key(|interval| interval[0]); // Sort by start time

    let mut merged_intervals = Vec::new();
    merged_intervals.push(sorted_intervals[0].clone());

    for interval in sorted_intervals.into_iter().skip(1) {
        let last_merged = merged_intervals.last_mut().unwrap();

        // Check for overlap:  If the current interval's start is less than or equal to the last merged interval's end
        if interval[0] <= last_merged[1] {
            // Merge: Update the end of the last merged interval if the current interval extends further
            last_merged[1] = last_merged[1].max(interval[1]);
        } else {
            // No overlap: Add the current interval as a new merged interval
            merged_intervals.push(interval.clone());
        }
    }

    merged_intervals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals_1() {
        let intervals = vec![
            vec![1, 3],
            vec![2, 6],
            vec![8, 10],
            vec![15, 18]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 6],
            vec![8, 10],
            vec![15, 18]
        ]);
    }

    #[test]
    fn test_merge_intervals_2() {
        let intervals = vec![
            vec![1, 4],
            vec![4, 5]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 5]
        ]);
    }

    #[test]
    fn test_merge_intervals_3() {
        let intervals = vec![
            vec![1, 4],
            vec![0, 4]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![0, 4]
        ]);
    }

    #[test]
    fn test_merge_intervals_4() {
        let intervals = vec![
            vec![1, 10],
            vec![2, 6],
            vec![8, 10]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 10]
        ]);
    }

    #[test]
    fn test_merge_intervals_5() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![4, 7],
            vec![8, 10]
        ];
        let result = merge_intervals(intervals);
        println!("Merged intervals: {:?}", result);
        assert_eq!(result, vec![
            vec![1, 2],
            vec![3, 7],
            vec![8, 10]
        ]);
    }
}
