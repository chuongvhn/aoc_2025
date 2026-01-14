
def get_number_time_left_in_zero_postion(input):
    cnt = 0
    current_pos = 50
    for item in input:
        num_of_steps_need_to_be_considered = item[1] % 100 # number of step can be greater than 100
        if item[0] == 'R':
            current_pos += num_of_steps_need_to_be_considered
            if current_pos > 99:
                current_pos = current_pos - 100
        else:
            current_pos -= num_of_steps_need_to_be_considered
            if current_pos < 0:
                current_pos = 100 - abs(current_pos)

        if current_pos == 0:
            cnt += 1
    return cnt


if __name__ == "__main__":
    lines = []
    with open("input_day01.txt") as f:
        lines = f.readlines()
        f.close()
    
    instructions = []
    for line in lines:
        direction = line[0]
        number_of_moves = int(line[1:])
        instructions.append((direction, number_of_moves))

    result_part1 = get_number_time_left_in_zero_postion(instructions)    
    print(f"result part 1 {result_part1}")