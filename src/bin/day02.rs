
fn get_number_of_digits(input: u64)->Vec<u64> {
    let mut rets : Vec<u64> = Vec::new();
    let mut tmp = input;
    let mut calculate = tmp / 10;
    let mut digit = tmp % 10;
    rets.push(digit);
    
    while calculate != 0 {
        tmp = calculate;
        calculate = tmp / 10;
        digit = tmp % 10;
        rets.push(digit);
    }

    return rets;
}

fn get_invalid_id(start: u64, end: u64)-> Vec<u64> {
    let mut result :Vec<u64> = Vec::new();
    for i in start..=end {
        let digits = get_number_of_digits(i);
        let mut is_ok = true;
        if digits.len() % 2 == 0 {
            for j in 0..digits.len() / 2 {
                if digits[j] != digits[j + digits.len()/2] {
                    is_ok = false;
                    break;
                } 
            }
            if is_ok {
                result.push(i);
            }
        }
    }
    return result;
}

struct CombinedDigit {
    num: u64,
    len: u64
}

fn combine_digit(start: u64, end: u64, digits : &Vec<u64>)->CombinedDigit{
    let mut ret : u64 = 0;
    for idx in start..=end {
        ret += digits[idx as usize] * (10_u64.pow((end - idx) as u32));
    }
    return CombinedDigit { num: ret, len: (end - start+1) };
}

fn is_invalid_id_part2(num: u64)->bool {
    let digits = get_number_of_digits(num);
    let mut tmp_len: u64 = 0;
    if digits.len() % 2 == 0 {
        tmp_len = digits.len() as u64 / 2 - 1;
    }
    else {
        tmp_len = digits.len() as u64 / 2;
    }
    for idx in 0..=tmp_len {
        let mut continue_to_check = true;
        let combined_digit = combine_digit(0,idx as u64, &digits);
        if digits.len() % combined_digit.len as usize != 0 { continue;}
        if digits.len() == 1 { continue;}
        let mut i_named_another_idx = combined_digit.len as usize;
        while i_named_another_idx <= digits.len() - combined_digit.len as usize {
            let candiate_to_compare = combine_digit(i_named_another_idx as u64, (i_named_another_idx + combined_digit.len as usize - 1) as u64, &digits);
            if combined_digit.num !=  candiate_to_compare.num {
                continue_to_check = false;
                break;
            }
            i_named_another_idx += combined_digit.len as usize;
        }
        if continue_to_check == true {
            return true;
        }
    }
    return false;
} 
        

fn get_invalid_id_part2(start: u64, end: u64)-> Vec<u64> {
    let mut result : Vec<u64> = vec![];
    for num in start..=end {
        if is_invalid_id_part2(num) {
            result.push(num);
        }
    }
    return result;
}

#[test]
fn run_tests() {
    let expected_vec = vec![3,2,1];
    assert_eq!(expected_vec,get_number_of_digits(123));
    let expected_vec = vec![0];
    assert_eq!(expected_vec,get_number_of_digits(0));
    let expected_vec = vec![0,0,0,3,2,1,1];
    assert_eq!(expected_vec, get_number_of_digits(1123000));
    
    let expected_vec = vec![11,22];
    assert_eq!(expected_vec,get_invalid_id(11, 22));
    
    let expected_vec = vec![99];
    assert_eq!(expected_vec,get_invalid_id(95, 115));
    
    let expected_vec = vec![1010];
    assert_eq!(expected_vec,get_invalid_id(998, 1012));
    
    let expected_vec = vec![1188511885];
    assert_eq!(expected_vec,get_invalid_id(1188511880, 1188511890));

    let expected_vec = vec![222222];
    assert_eq!(expected_vec,get_invalid_id(222220, 222224));

    let expected_vec: Vec<u64> = vec![];
    assert_eq!(expected_vec,get_invalid_id(1698522, 1698528));

    let expected_vec: Vec<u64> = vec![446446];
    assert_eq!(expected_vec,get_invalid_id(446443, 446449));
    
    let expected_vec: Vec<u64> = vec![38593859];
    assert_eq!(expected_vec,get_invalid_id(38593856, 38593862));
    

}

#[test]
fn test_part2() {
    let numbers = get_invalid_id_part2(38593856, 38593862);
    for n in numbers {
        println!("{n},");
    }
}

#[test]
fn test_is_invalid_id_part2() {
   assert_eq!(false,is_invalid_id_part2(3737333733));
   assert_eq!(true,is_invalid_id_part2(3737337373));
   assert_eq!(false, is_invalid_id_part2(1));
}

#[test]
fn test_combine_digits() {
    let input: Vec<u64> = vec![1];
    let ret = combine_digit(0, 0, &input);
    assert_eq!(1, ret.num);
    let input: Vec<u64> = vec![1,2,3];
    let ret = combine_digit(0, 2, &input);
    assert_eq!(123, ret.num);
}

fn split_into_start_end(input: &str)->Vec<u64> {
    let nums: Vec<u64> = input.split("-").map(|x| x.parse().unwrap()).collect();
    nums
}

fn split(input: &String)->Vec<&str> {
    let nums: Vec<&str> = input.split(",").collect();
    nums
}

fn main() {
    // run_tests();
    let input = aoc_2025::read_file("input_day02.txt");
    let ids = split(&input[0]);
    let mut ret: u64 = 0;
    let mut ret2: u64 = 0;
    for id in ids {
        let ranges = split_into_start_end(&id);
        let numbers = get_invalid_id(ranges[0],ranges[1]);
        for num in numbers {
            ret += num as u64;
        }
        let numbers = get_invalid_id_part2(ranges[0], ranges[1]);
        for num in numbers {
            ret2 += num as u64;
        }
    }

    println!("ret {ret}, ret2 {ret2}");
}