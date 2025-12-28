

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
}