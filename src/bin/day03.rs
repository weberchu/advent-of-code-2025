use std::str::Lines;
use std::cmp::max;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = read_input();
    let lines = input.lines();

    let joltage = find_joltage(lines, 2);

    println!("part_1 = {joltage}");
}

fn part_2() {
    let input = read_input();
    let lines = input.lines();

    let joltage = find_joltage(lines, 12);

    println!("part_1 = {joltage}");
}

fn find_joltage(lines: Lines, num_of_battery: usize) -> i64 {
    let mut joltage: i64 = 0;

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let mut chosen_digits = vec!['0'; num_of_battery];

        for i in 0 .. chars.len() {
            let d = chars[i];

            let start_j = max(0, num_of_battery as i32 - chars.len() as i32 + i as i32);
            for j in start_j as usize .. num_of_battery {
                if d > chosen_digits[j] {
                    chosen_digits[j] = d;
                    for k in j + 1 .. num_of_battery {
                        chosen_digits[k] = '0';
                    }
                    break;
                }
            }
        }

        let value: String = chosen_digits.into_iter().collect();

        joltage += value.parse::<i64>().unwrap();
    }

    joltage
}

fn read_input() -> &'static str {
    include_str!("../../input/day03.txt")
    // include_str!("../../input/test.txt")
}