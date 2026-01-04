use std::u64;

fn part1(ingredient_id_ranges: &Vec<(u64,u64)>, available_ingredient_ids: &Vec<u64>) -> u64 {
    let mut cnt =0;

    for available_ingredient_id in available_ingredient_ids {
        for range in ingredient_id_ranges {
            if range.0 <= *available_ingredient_id && *available_ingredient_id <= range.1 {
                cnt += 1;
                break;
            }
        }
    }
    cnt
}

fn main() {
    let input = aoc_2025::read_file("input_day05.txt");
    let mut ingredient_id_ranges :Vec<(u64,u64)> = Vec::new();
    let mut available_ingredient_ids :Vec<u64> = Vec::new();
    let mut end_of_id_range = false;
    for line in input {
        if line.is_empty() {
            end_of_id_range = true;
        }

        if !end_of_id_range {
            let parts: Vec<u64> = line.split("-").map(|s| s.parse::<u64>().unwrap()).collect();
            if parts.len() == 2 {
                ingredient_id_ranges.push((parts[0], parts[1]));
            }
        }
        else {
            if line.is_empty() {continue;}
            let id_in_integer = line.parse::<u64>().unwrap();
            available_ingredient_ids.push(id_in_integer);
        }
    }

    let resutl = part1(&ingredient_id_ranges, &available_ingredient_ids);
    println!("result part1 {}", resutl);
    
}