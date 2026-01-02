

// get number of rolls in 8 adjacent positions

fn part1(buffer: &[Vec<u32>; 140]) -> u32 {
    let mut cnt = 0u32;
    for (row_index, row) in buffer.iter().enumerate() {
        for (col_index, &value) in row.iter().enumerate() {
            if value == 1 {
                // check 8 adjacent positions
                let adjacent_rolls = [
                    (-1, -1), (-1, 0), (-1, 1),
                    (0, -1),           (0, 1),
                    (1, -1),  (1, 0),  (1, 1),
                ]
                .iter()
                .filter(|(dr, dc)| {
                    let new_row = (row_index as i32 + dr) as usize;
                    let new_col = (col_index as i32 + dc) as usize;
                    new_row < buffer.len() && new_col < buffer[new_row].len() && buffer[new_row][new_col] == 1
                })
                .count() as u32;
                if adjacent_rolls < 4 {
                    cnt += 1;
                }
            }
        }
    }
    cnt
}

struct NumberOfRollsAndPosition {
    cnt: u32,
    position: Vec<(usize, usize)>
}

fn check_adjacent_pos_for_part2(buffer: &mut [Vec<u32>; 140]) -> NumberOfRollsAndPosition {
    let mut cnt = 0u32;
    let mut pos_can_be_removed: Vec<(usize, usize)> = Vec::new();
    for row_idx in 0..buffer.len() {
        for col_idx in 0..buffer[row_idx].len() {
            let eight_adjacent_pos = [
                (-1isize, -1isize), (-1, 0), (-1, 1),
                (0, -1),                    (0, 1),
                (1, -1),  (1, 0),  (1, 1),
            ];
            if buffer[row_idx][col_idx] == 1 {
                let mut numer_of_pos_has_value = 0;
                for &pos in &eight_adjacent_pos {
                    let new_row = row_idx as isize + pos.0;
                    let new_col = col_idx as isize + pos.1;
                    if new_row >= 0 && (new_row as usize) < buffer.len() &&
                        new_col >= 0 && (new_col as usize) < buffer[row_idx].len() &&
                        buffer[new_row as usize][new_col as usize] == 1 {
                        numer_of_pos_has_value += 1;
                    }
                }
                if numer_of_pos_has_value < 4 {
                    pos_can_be_removed.push((row_idx as usize,col_idx as usize));
                    cnt += 1;
                }
            }
            
        }
    }
    NumberOfRollsAndPosition{cnt: cnt, position: pos_can_be_removed }
}

fn part2(buffer: &mut [Vec<u32>;140]) -> u32 {
    let mut result = 0;
    loop {
        let ret = check_adjacent_pos_for_part2(buffer);
        if ret.cnt == 0 {
            break;
        }
        else {
            for pos in ret.position {
                buffer[pos.0][pos.1] = 0;
            }
        }
        result += ret.cnt;
    }
    result
}

fn main() {
    let input = aoc_2025::read_file("input_day04.txt");
    let mut buffer: [Vec<u32>;140] = std::array::from_fn(|_| Vec::new());
    for (row_index, line) in input.iter().enumerate() {
        for symbol in line.chars() {
            match symbol {
                '@' => {
                    buffer[row_index].push(1);
                },
                '.' => {
                    buffer[row_index].push(0);
                },
                _ => {}
            }
        }
    }
    let result = part1(&buffer);
    println!("Part 1: {}", result);
    let result =part2(&mut buffer);
    println!("Part 2: {}", result);
}