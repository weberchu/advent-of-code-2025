fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = read_input();

    let lines = input.lines();

    let mut dial: i32 = 50;
    let mut zero_count = 0;

    for (_i, line) in lines.enumerate() {
        let (direction, distance) = line.split_at(1);
        let distance = distance.parse::<i32>().unwrap();

        match direction {
            "L" => {
                dial -= distance
            },
            "R" => {
                dial += distance
            },
            _ => {
                panic!("unexpected direction {direction}");
            }
        }

        dial = dial % 100;

        if dial < 0 {
            dial += 100;
        }

        if dial == 0 {
            zero_count += 1;
        }
    }

    println!("zero count = {zero_count}");
}

fn part_2() {
    let input = read_input();

    let lines = input.lines();

    let mut dial: i32 = 50;
    let mut zero_count = 0;

    for (_i, line) in lines.enumerate() {
        let (direction, distance) = line.split_at(1);
        let distance = distance.parse::<i32>().unwrap();
        let original_dial = dial;

        match direction {
            "L" => {
                dial -= distance
            },
            "R" => {
                dial += distance
            },
            _ => {
                panic!("unexpected direction {direction}");
            }
        }

        let pass_throug_zero_count = if dial <= 0 && original_dial > 0 {
            dial.abs() / 100 + 1
        } else {
            dial.abs() / 100
        };

        zero_count += pass_throug_zero_count;

        dial = dial % 100;

        if dial < 0 {
            dial += 100;
        }

        // println!("{} {}: dial={}, pass_throug_zero_count={}", i, line, dial, pass_throug_zero_count);
        // println!("{}: {}", i, dial);
    }

    println!("zero count = {zero_count}");
}


fn read_input() -> &'static str {
    include_str!("../../input/day01.txt")
    // include_str!("../../input/test.txt")
}