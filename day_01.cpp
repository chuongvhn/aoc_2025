#include <iostream>
#include <fstream>
#include <vector>


std::uint32_t get_number_time_left_in_zero_postion(std::vector<std::string>& input)
{
    std::uint32_t ret{};
    std::int32_t current_pos{50};
    for (auto const& line : input) {
        auto direction = line.substr(0,1);
        auto number_of_steps = std::stoi(line.substr(1, line.length() -1 )) % 100; //number of steps can be greater than 100
        if (direction == "R" )
        {
            current_pos += number_of_steps;
            if (current_pos > 99 )
            {
                current_pos = current_pos - 100;
            }
        }
        else 
        {
            current_pos -= number_of_steps;
            if (current_pos < 0)
            {
                current_pos = 100 - std::abs(current_pos);
            }
        }

        if (current_pos == 0)
        {
            ret += 1;
        }
    }

    return ret;
}

int main()
{
    std::ifstream input("input_day01.txt");
    if (!input.is_open()) {
        std::cerr << "file not exist\n";
        return -1;
    }

    std::string lines;
    std::vector<std::string> buffer;
    while (std::getline(input, lines)) 
    {
        buffer.push_back(std::move(lines));
    }
    input.close();

    auto ret = get_number_time_left_in_zero_postion(buffer);
    std::cout << "result part 1: " << ret << std::endl;
}