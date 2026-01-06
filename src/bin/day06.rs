
fn parse_input_to_work_sheet(input: &[String]) -> u64 {
    let mut result = 0u64;
    let mut work_sheet :Vec<Vec<String>> = Vec::new();
    for line in input {
        work_sheet.push(line.split_whitespace().map(|s| s.to_string()).collect());
    }
    for col_idx in 0..work_sheet[0].len() {
        let mut result_for_a_column = 0u64;
        if work_sheet[work_sheet.len() - 1][col_idx] == "*" {
            result_for_a_column = work_sheet[0][col_idx].parse::<u64>().unwrap() 
                                * work_sheet[1][col_idx].parse::<u64>().unwrap()
                                * work_sheet[2][col_idx].parse::<u64>().unwrap()
                                * work_sheet[3][col_idx].parse::<u64>().unwrap()
        }
        else if work_sheet[work_sheet.len() -1][col_idx] == "+" {
            result_for_a_column = work_sheet[0][col_idx].parse::<u64>().unwrap() 
                                + work_sheet[1][col_idx].parse::<u64>().unwrap()
                                + work_sheet[2][col_idx].parse::<u64>().unwrap()
                                + work_sheet[3][col_idx].parse::<u64>().unwrap()
        }
        result += result_for_a_column;

    }
    result
}

fn main() {
    let input = aoc_2025::read_file("input_day06.txt");
    let result_part1 = parse_input_to_work_sheet(&input);
    println!("result part 1 {}", result_part1);
}