/*
 * Title: Overlapping Intervals
 * Author: WKS
 * Description: https://www.shiftedup.com/2015/05/17/programming-challenge-merging-overlapping-intervals
 */
use std::ops::Range;
use std::cmp;

fn main() {
    let mut ranges = vec![2..4, 1..3, 8..16, 9..11, 21..25];

    println!("In: {:?}", ranges);

    ranges.sort_by(|r1, r2| r1.start.cmp(&r2.start));

    let mut merged_ranges: Vec<Range<i32>> = vec![];
    let mut current_range = ranges.first().unwrap().clone();

    for i in 0..ranges.len() {
        if let Some(next_range) = ranges.get(i + 1) {
            if next_range.start <= current_range.end {
                let end = cmp::max(current_range.end, next_range.end);
                current_range = current_range.start..end
            } else {
                merged_ranges.push(current_range);
                current_range = next_range.clone();
            }
        } else {
            merged_ranges.push(current_range);
            break;
        }
    }

    println!("Out: {:?}", merged_ranges);
}