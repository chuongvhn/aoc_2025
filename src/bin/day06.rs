
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
    let mut work_sheet :Vec<Vec<char>> = Vec::new();
    for (row_idx, line) in input.iter().enumerate() {
        work_sheet.push(Vec::new());
        for c in line.chars() {
            work_sheet[row_idx].push(c);
        }
    }

    let mut parsed_numbers_from_worksheet : Vec<String> = Vec::new();
    for idx in (0..work_sheet[0].len()).rev() {
        if work_sheet[0][idx] != ' ' || work_sheet[1][idx] != ' ' || work_sheet[2][idx] != ' ' || work_sheet[3][idx] != ' ' {
            let mut num : String = String::new();
            num.push(work_sheet[0][idx]);
            num.push(work_sheet[1][idx]);
            num.push(work_sheet[2][idx]);
            num.push(work_sheet[3][idx]);
            let num_without_space = num.replace(" ","");
            parsed_numbers_from_worksheet.push(num_without_space);
        }
        else {
            parsed_numbers_from_worksheet.push(work_sheet[4][idx +1].to_string());
        }
        if idx == 0 {
            parsed_numbers_from_worksheet.push(work_sheet[4][idx].to_string());
        }
    }

    //debug
    println!("{:?}", parsed_numbers_from_worksheet);
    let mut result = 0u64;
    let mut stack_to_store_all_nums_in_each_operation: Vec<u64> = Vec::new();
    for number_in_string in parsed_numbers_from_worksheet {
        let maybe_number = number_in_string.parse::<u64>();
        if maybe_number.is_ok() {
            stack_to_store_all_nums_in_each_operation.push(maybe_number.unwrap());
        }
        else {
            let mut tmp_multiple = 1u64;
            let mut tmp_sum = 0u64;
            for num in &stack_to_store_all_nums_in_each_operation {
                if number_in_string == "*" {
                    tmp_multiple *= num;
                }
                else if number_in_string == "+" {
                    tmp_sum += num;
                }
                else {
                    panic!("weird thing is happening");
                }
            }
            stack_to_store_all_nums_in_each_operation.clear();
            if tmp_multiple != 1 {
                result += tmp_multiple;
            }
            if tmp_sum != 0 {
                result += tmp_sum;
            }
        }
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