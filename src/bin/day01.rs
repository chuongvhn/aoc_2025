
// left 'L', right 'R'
fn get_direction(input: &str) -> char {
    input.chars().next().unwrap_or('x')
}

// get number of steps RX, LY -> move right X steps or left Y steps
fn get_steps(input: &str) -> i32 {
    let steps = input.chars()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    steps % 100
}

fn number_of_moves(input: &Vec<String>) -> u32 {
    let mut current_pos: i32 = 50;
    let mut cnt = 0;
    for line in input {
        let direction = get_direction(&line);
        let steps  = get_steps(&line);
        // println!("{line} {direction} {steps}");
        if direction == 'R' {
            current_pos = current_pos + steps;
            if current_pos == 0 {
                cnt += 1;
            }
            else if current_pos == 100 {
                cnt += 1;
                current_pos = 0;
            }
            else if current_pos > 100 {
                current_pos = current_pos - 99 -1;
                if current_pos == 0 {
                    cnt +=1;
                }
            }
        }
        else if direction == 'L' {
            current_pos = current_pos - steps;
            if current_pos == 0 {
                cnt += 1;
            }
            else if current_pos < 0 {
                current_pos = 99 + current_pos +1;
                if current_pos == 0 {
                    cnt +=1;
                }
            }
        }
        else {
            eprintln!("Unknown direction {direction}");
        }
    }
    return cnt;
}


fn number_of_moves_passing_zero(input: &Vec<String>) -> u32 {
    let mut current_pos: i32 = 50;
    let mut temp_pos = 0;
    let mut cnt = 0;
    for line in input {
        temp_pos = current_pos;
        let direction = get_direction(&line);
        let steps  = get_steps(&line);
        // println!("{line} {direction} {steps}");
        if direction == 'R' {
            current_pos = current_pos + steps;
            if current_pos == 0 {
                cnt += 1;
            }
            else if current_pos == 100 {
                cnt += 1;
                current_pos = 0;
            }
            else if current_pos > 100 {
                cnt += 1;
                current_pos = current_pos - 99 -1;
                if current_pos == 0 {
                    cnt -=1;
                }
            }
        }
        else if direction == 'L' {
            current_pos = current_pos - steps;
            if current_pos == 0 {
                cnt += 1;
            }
            else if current_pos < 0 {
                if temp_pos != 0 {
                    cnt += 1;
                }

                current_pos = 99 + current_pos +1;
                if current_pos == 0 {
                    cnt -=1;
                }
            }
        }
        else {
            eprintln!("Unknown direction {direction}");
        }
    }
    return cnt;
}

fn get_nums_cross_zero(input: &str) -> u32 {
    let steps = input.chars()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap_or(0);
    steps / 100
}

fn number_of_moves_part2(input: &Vec<String>)-> u32 {
    let mut ret = number_of_moves_passing_zero(input);
    for mo in input {
        ret = ret + get_nums_cross_zero(mo);
    }
    return ret;
}

fn test_get_direction() {
    assert_eq!(get_direction("R23"), 'R');
    assert_eq!(get_direction("L5"), 'L');
    assert_eq!(get_direction("X10"), 'X');
}

fn test_get_steps() {
    assert_eq!(get_steps("R23"), 23);
    assert_eq!(get_steps("L5"), 5);
    assert_eq!(get_steps("X10"), 10);
}

fn test_get_number_of_moves() {
    let input = vec![
        String::from("L68"),
        String::from("L30"),
        String::from("R48"),
        String::from("L5"),
        String::from("R60"),
        String::from("L55"),
        String::from("L1"),
        String::from("L99"),
        String::from("R14"),
        String::from("L82"),
    ];
    assert_eq!(number_of_moves(&input), 3);
    assert_eq!(number_of_moves_part2(&input),6);
}

fn run_tests() {
    test_get_direction();
    test_get_steps();
    test_get_number_of_moves();
}
fn main() {
    run_tests();
    let content = aoc_2025::read_file("input_day01.txt");
    let cnt = number_of_moves(&content);
    let cnt2= number_of_moves_part2(&content);
    println!("Result part 1: {}, result part 2: {}",cnt, cnt2);    
}
