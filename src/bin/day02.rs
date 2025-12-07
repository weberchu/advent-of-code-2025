fn main() {
    let input = read_input();
    let mut part_1_sum: i64 = 0;
    let mut part_2_sum: i64 = 0;

    let ranges = input.split(",");
    for range in ranges {
        let mut range = range.split("-");
        let start_str = range.next().unwrap();
        let start = start_str.parse::<i64>().unwrap();
        let end_str = range.next().unwrap();
        let end = end_str.parse::<i64>().unwrap();

        part_1_sum += repeat_twice(start, end);
        part_2_sum += repeat_any(start, end);
    }

    println!("part_1_sum = {}", part_1_sum);
    println!("part_2_sum = {}", part_2_sum);
}

fn repeat_twice(start: i64, end: i64) -> i64 {
    if start.to_string().len() == end.to_string().len() && start.to_string().len() % 2 == 1 {
        // println!("range is single-digit so cannot be repeated");
        return 0;
    }

    let mut duplicated_sum: i64 = 0;

    for i in start..end+1 {
        let str = i.to_string();
        let (first_half, second_half) = str.split_at(str.len() / 2);
        if first_half == second_half {
            duplicated_sum += str.parse::<i64>().unwrap();
        }
    }

    duplicated_sum
}

fn repeat_any(start: i64, end: i64) -> i64 {
    let mut duplicated_sum: i64 = 0;

    for i in start..end+1 {
        let str = i.to_string();

        for base_len in 1..(str.len() / 2 + 1) {
            if str.len() % base_len == 0 {
                let base_str: String = str.chars().take(base_len).collect();
                let repeating_str = base_str.repeat(str.len() / base_len);
                if repeating_str == str {
                    // println!("repeating_str = {repeating_str}");
                    duplicated_sum += str.parse::<i64>().unwrap();
                    break;
                }
            }
        }
    }

    duplicated_sum
}

fn read_input() -> &'static str {
    include_str!("../../input/day02.txt")
    // include_str!("../../input/test.txt")
}