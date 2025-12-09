const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = read_input();
    let lines = input.lines();

    let mut map: Vec<Vec<char>> = vec![];

    for line in lines {
        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }

    let mut accessible_count = 0;

    for c in 0 .. map.len() {
        for r in 0 .. map[c].len() {
            if map[c][r] != '@' {
                continue;
            }

            let neighbor_row_count = NEIGHBORS.iter().filter(|neighbor| {
                let neighbor_c = c as i32 + neighbor.0;
                let neighbor_r = r as i32 + neighbor.1;
                neighbor_c >= 0 && neighbor_c < map.len() as i32
                    && neighbor_r >= 0 && neighbor_r < map[c].len() as i32
                    && map[neighbor_c as usize][neighbor_r as usize] == '@'
            }).count();

            if neighbor_row_count < 4 {
                accessible_count += 1;
            }
        }
    }

    println!("part_1 = {:?}", accessible_count);
}

fn part_2() {
    let input = read_input();
    let lines = input.lines();

    let mut map: Vec<Vec<char>> = vec![];

    for line in lines {
        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }

    let mut removed_count = 0;
    let mut prev_removed_count = -1;

    while prev_removed_count != removed_count {
        prev_removed_count = removed_count;

        for c in 0 .. map.len() {
            for r in 0 .. map[c].len() {
                if map[c][r] != '@' {
                    continue;
                }

                let neighbor_row_count = NEIGHBORS.iter().filter(|neighbor| {
                    let neighbor_c = c as i32 + neighbor.0;
                    let neighbor_r = r as i32 + neighbor.1;
                    neighbor_c >= 0 && neighbor_c < map.len() as i32
                        && neighbor_r >= 0 && neighbor_r < map[c].len() as i32
                        && map[neighbor_c as usize][neighbor_r as usize] == '@'
                }).count();

                if neighbor_row_count < 4 {
                    map[c][r] = '.';
                    removed_count += 1;
                }
            }
        }
    }

    println!("part_2 = {}", removed_count);
}


fn read_input() -> &'static str {
    include_str!("../../input/day04.txt")
    // include_str!("../../input/test.txt")
}