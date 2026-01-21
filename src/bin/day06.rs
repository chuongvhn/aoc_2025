
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

fn part2(input: &[String]) -> u64 {
    let mut result = 0u64;
    let mut work_sheet :Vec<Vec<char>> = Vec::new();
    for (row_idx, line) in input.iter().enumerate() {
        work_sheet.push(Vec::new());
        for c in line.chars() {
            work_sheet[row_idx].push(c);
        }
        result += 1;
    }

    let mut parsed_number_from_worksheet : Vec<String> = Vec::new();
    for idx in (0..work_sheet[0].len() -1).rev() {
        if work_sheet[0][idx] != ' ' && work_sheet[1][idx] != ' ' && work_sheet[2][idx] != ' ' && work_sheet[3][idx] != ' ' {
            let mut num : String = String::new();
            num.push(work_sheet[0][idx]);
            num.push(work_sheet[1][idx]);
            num.push(work_sheet[2][idx]);
            num.push(work_sheet[3][idx]);
            parsed_number_from_worksheet.push(num);
        }
        else {
            parsed_number_from_worksheet.push(work_sheet[4][idx -1].to_string());
        }
    }

    for item in parsed_number_from_worksheet {
        println!("{item}");
    }

    result
}

fn main() {
    let input = aoc_2025::read_file("input_day06.txt");
    let result_part1 = parse_input_to_work_sheet(&input);
    println!("result part 1 {}", result_part1);
    let result_part2 = part2(&input);
    println!("result part 2 {}", result_part2);   
}