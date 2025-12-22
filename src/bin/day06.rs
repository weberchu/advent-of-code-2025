fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = read_input();
    let lines = input.lines();
    let mut all_numbers: Vec<Vec<i64>> = vec![];
    let mut grand_total = 0;

    for line in lines {
        let numbers: Vec<&str> = line.split(" ").collect();

        if numbers[0] == "+" || numbers[0] == "*" {
            for (idx, &operator) in numbers.iter().filter(|o| !o.is_empty()).enumerate() {
                let column_total = match operator {
                    "+" => {
                        let mut sum: i64 = 0;
                        for n in &all_numbers {
                            sum += n[idx];
                        }
                        sum
                    },
                    "*" => {
                        let mut product: i64 = 1;
                        for n in &all_numbers {
                            product *= n[idx];
                        }
                        product
                    },
                    _ => {
                        panic!("Unknown operator {}", operator);
                    }
                };

                grand_total += column_total;
            }
        } else {
            let numbers: Vec<i64> = numbers.iter().filter(|n| !n.is_empty()).map(|n| n.parse::<i64>().unwrap()).collect();
            all_numbers.push(numbers);
        }
    }

    println!("part_1 = {}", grand_total);
}

fn part_2() {
    let input = read_input();
    let lines = input.lines();
    let mut lines: Vec<&str> = lines.collect();
    let mut grand_total: i64 = 0;
    let mut last_operator = '+';
    let mut current_operator_total: i64 = 0;

    let operator_line = lines.remove(lines.len() - 1);
    let lines = lines;

    for (i, operator) in operator_line.chars().enumerate() {
        if operator == '+' {
            last_operator = '+';
            current_operator_total = 0;
        } else if operator == '*' {
            last_operator = '*';
            current_operator_total = 1;
        }

        let vertical_number: String = lines.iter().map(|&l| l.chars().nth(i).unwrap()).collect();
        let vertical_number = vertical_number.trim();
        if vertical_number.len() > 0 {
            let n = vertical_number.parse::<i64>().unwrap();
            if last_operator == '+' {
                current_operator_total += n;
            } else if last_operator == '*' {
                current_operator_total *= n;
            } else {
                panic!("Unknown last_operator {}", last_operator);
            }
        } else {
            grand_total += current_operator_total;
            current_operator_total = 0;
        }
    }

    grand_total += current_operator_total;

    println!("part_2 = {:?}", grand_total);
}

fn read_input() -> &'static str {
    include_str!("../../input/day06.txt")
    // include_str!("../../input/test.txt")
}