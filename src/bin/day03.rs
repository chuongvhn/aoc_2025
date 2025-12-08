use std::collections::VecDeque;


fn get_battery_ids(line: &String) -> u32 {
    let mut numbers: VecDeque<u32> = VecDeque::new();
    for s in line.chars() {
        numbers.push_back(s.to_string().parse().unwrap());
    }
    let mut max_0: u32 = 0;
    let mut max_0_idx = 0;
    let mut max_1 = 0;
    let mut max_1_idx = 0;
    for (index,num) in numbers.iter().enumerate() {
        if *num > max_0 {
            max_0 = *num;
            max_0_idx = index;
        }
    }
    if max_0_idx == numbers.len() - 1 {
        for (index,num) in numbers.iter().enumerate() {
            if *num > max_1 && index < numbers.len() - 1 {
                max_1 = *num;
                max_1_idx = index;
            }
        }
    }
    else {
        for idx in max_0_idx +1..numbers.len() {
            if numbers[idx] > max_1 {
                max_1 = numbers[idx];
                max_1_idx = idx;
            }
        }
    }

    let mut ret = 0;
    if max_0_idx < max_1_idx {
        ret = max_0 * 10 + max_1;
    }
    else {
        ret = max_1 * 10 + max_0;
    }
    return ret;
}

#[test]
fn test_get_battery_ids() {
    let line = "987654321111111".to_string();
    assert_eq!(98,get_battery_ids(&line));
    let line = "811111111111119".to_string();
    assert_eq!(89,get_battery_ids(&line));
    let line = "234234234234278".to_string();
    assert_eq!(78,get_battery_ids(&line));
    let line = "818181911112111".to_string();
    assert_eq!(92,get_battery_ids(&line));
    let line = "2222422126525122433332324122232442621332112124353325142213221221321242522245242222212213223253222222".to_string();
    assert_eq!(66,get_battery_ids(&line));
    let line = "6986637616744837475696535678356366536287839555455376737334329457284565566954578742689469447785687752".to_string();
    assert_eq!(99, get_battery_ids(&line));
    let line = "4444355567375444144441434742443725566463147643443444453733475443534453656464433364465467345427344384".to_string();
    assert_eq!(84, get_battery_ids(&line));
}

fn main() {
    let input = aoc_2025::read_file("input_day03.txt");
    let mut result : u32 = 0;
    for line in input {
        let ret = get_battery_ids(&line);
        println!("{line} : {ret}");
        result += ret;

    }
    println!("{result}");
}