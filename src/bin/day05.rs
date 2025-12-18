use std::cmp::{max, min};
use std::collections::HashSet;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = read_input();
    let lines = input.lines();

    let mut ranges = HashSet::<(i64, i64)>::new();
    let mut is_processing_ranges = true;
    let mut fresh_count = 0;

    for line in lines {
        if line.len() == 0 {
            is_processing_ranges = false;
            continue;
        }

        if is_processing_ranges {
            let mut range = line.split("-");
            let start = range.next().unwrap().parse::<i64>().unwrap();
            let end = range.next().unwrap().parse::<i64>().unwrap();
            ranges.insert((start, end));
        } else {
            let id = line.parse::<i64>().unwrap();

            if is_fresh(id, &ranges) {
                fresh_count += 1;
            }
        }
    }

    println!("part_1 = {}", fresh_count);
}

fn is_fresh(id: i64, ranges: &HashSet<(i64, i64)>) -> bool {
    ranges.into_iter().any(|(start, end)| {
        &id >= start && &id <= end
    })
}



fn part_2() {
    let input = read_input();
    let lines = input.lines();
    let mut ranges = HashSet::<(i64, i64)>::new();

    for line in lines {
        if line.len() == 0 {
            break;
        }

        let mut range = line.split("-");
        let start = range.next().unwrap().parse::<i64>().unwrap();
        let end = range.next().unwrap().parse::<i64>().unwrap();

        add_ranges(&mut ranges, (start, end));
    }

    let total: i64 = ranges.iter().map(|(start, end)| {
        end - start + 1
    }).sum();

    println!("part_2 = {}", total);
}

fn add_ranges(ranges: &mut HashSet<(i64, i64)>, new_range: (i64, i64)) {
    let mut range_to_replace: Option<((i64, i64), (i64, i64))> = None;

    for range in ranges.iter() {
        if new_range.1 < range.0 || new_range.0 > range.1 {
            continue;
        }

        let combined_range = (min(range.0, new_range.0), max(range.1, new_range.1));
        range_to_replace = Some((range.clone(), combined_range));
        break;
    }

    if let Some((to_remove, combined_range)) = range_to_replace {
        // some overlap found
        ranges.remove(&to_remove);
        add_ranges(ranges, combined_range)
    } else {
        // no overlap found
        ranges.insert(new_range);
    }
}

fn read_input() -> &'static str {
    include_str!("../../input/day05.txt")
    // include_str!("../../input/test.txt")
}