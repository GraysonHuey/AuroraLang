#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <sstream>
#include <variant>
#include <limits>

// Function to split a string by space
std::vector<std::string> split(const std::string& s) {
    std::vector<std::string> tokens;
    std::istringstream iss(s);
    std::string token;
    while (iss >> token) {
        tokens.push_back(token);
    }
    return tokens;
}


int main(int argc, char** argv) {

    if (argc != 2) {
        std::stringstream ss;
        ss << "Incorrect arguments! Usage: " << argv[0] << " <filename>\n";
        throw std::runtime_error(ss.str());
    }

    std::string file_name = argv[1];

    if (file_name.size() < 4 || file_name.substr(file_name.size() - 4) != ".aur") {
        throw std::runtime_error("Not an Aurora (.aur) file.");
    }

    std::ifstream file(file_name);
    if (!file.is_open()) {
        throw std::runtime_error("Error opening file: " + file_name);
    }

    std::vector<std::variant<int, std::string>> stack;
    std::string line;
    int current_line = 0;

    bool startFound = false;
    bool endFound = false;
    while (std::getline(file, line)) {
        line.erase(0, line.find_first_not_of(" \t\r\n")); // Trim leading whitespace
        if (line == "START") {
            startFound = true;
        }
        if (line == "END") {
            endFound = true;
        }
    }

    if (!startFound) {
        throw std::runtime_error("START not found");
    }

    if (!endFound) {
        throw std::runtime_error("END not found");
    }

    file.clear();
    file.seekg(0);

    while (std::getline(file, line)) {
        current_line++;
        line.erase(0, line.find_first_not_of(" \t\r\n")); // Trim leading whitespace
        line = line.substr(0, line.find_last_not_of(" \t\r\n")+1);
        //std::cout << line << std::endl;

        if (line.empty() || line.substr(0, 2) == "//") {
            continue; // Skip empty lines or comments
        }

        if (line.substr(0, 3) == "END") {
            break;
        }

        std::vector<std::string> command_parts = split(line);
        std::string command = command_parts[0];

        if (command == "PUSHINT") {
            int num;
            try {
                num = stoi(command_parts[1]);
            } catch (std::exception &e) {
                throw std::runtime_error("Invalid PUSH command. Usage: PUSHINT <number>");
            }
            stack.emplace_back(num);
        }

        else if (command == "PUSHSTR") {
            // Create an output stringstream
            std::ostringstream oss;

            // Join the parts starting from index 1 with space separator
            for (size_t i = 1; i < command_parts.size(); i++) {
                if (i > 1) {
                    oss << " "; // Add space separator
                }
                oss << command_parts[i]; // Add the part to stringstream
            }

            // Get the joined string
            std::string result = oss.str();
            stack.emplace_back(result);
        }

        else if (command == "POP") {
            if (stack.empty()) {
                throw std::runtime_error("Stack is empty, cannot POP");
            }
            stack.pop_back();
        }

        else if (command == "CLEAR") {
            if (stack.empty()) {
                throw std::runtime_error("Stack is already empty, cannot CLEAR");
            }
            stack.clear();
        }

        else if (command == "ADD") {
            if (stack.size() < 2) {
                throw std::runtime_error("Not enough values in the stack to perform ADD");
            }
            auto num1_a = stack[stack.size() - 1];
            auto num2_a = stack[stack.size() - 2];
            if (std::holds_alternative<std::string>(num1_a) || std::holds_alternative<std::string>(num2_a)) {
                throw std::runtime_error("Cannot add an INT and a STR");
            }
            int num1_i = std::get<int>(num1_a);
            int num2_i = std::get<int>(num2_a);

            stack.emplace_back(num1_i + num2_i);
        }

        else if (command == "SUB") {
            if (stack.size() < 2) {
                throw std::runtime_error("Not enough values in the stack to perform SUB");
            }
            auto num2_a = stack[stack.size() - 1];
            auto num1_a = stack[stack.size() - 2];
            if (std::holds_alternative<std::string>(num1_a) || std::holds_alternative<std::string>(num2_a)) {
                throw std::runtime_error("Cannot subtract an INT and a STR");
            }
            int num1_i = std::get<int>(num1_a);
            int num2_i = std::get<int>(num2_a);

            stack.emplace_back(num1_i - num2_i);
        }

        else if (command == "MUL") {
            if (stack.size() < 2) {
                throw std::runtime_error("Not enough values in the stack to perform MUL");
            }
            auto num1_a = stack[stack.size() - 1];
            auto num2_a = stack[stack.size() - 2];
            if (std::holds_alternative<std::string>(num1_a) || std::holds_alternative<std::string>(num2_a)) {
                throw std::runtime_error("Cannot multiply an INT and a STR");
            }
            int num1_i = std::get<int>(num1_a);
            int num2_i = std::get<int>(num2_a);

            stack.emplace_back(num1_i * num2_i);
        }

        else if (command == "DIV") {
            if (stack.size() < 2) {
                throw std::runtime_error("Not enough values in the stack to perform DIV");
            }
            auto num2_a = stack[stack.size() - 1];
            auto num1_a = stack[stack.size() - 2];

            int num1_i;
            int num2_i;
            try {
                num1_i = std::get<int>(num1_a);
                num2_i = std::get<int>(num2_a);
            } catch (std::exception &e) {
                //throw std::runtime_error("Cannot divide an INT and a STR");
            }

            if (num2_i == 0) {
                throw std::runtime_error("Cannot divide by 0.");
            }

            stack.emplace_back(num1_i / num2_i);
        }

        else if (command == "MOD") {
            if (stack.size() < 2) {
                throw std::runtime_error("Not enough values in the stack to perform MOD");
            }
            auto num2_a = stack[stack.size() - 1];
            auto num1_a = stack[stack.size() - 2];

            int num1_i;
            int num2_i;
            try {
                num1_i = std::get<int>(num1_a);
                num2_i = std::get<int>(num2_a);
            } catch (std::exception &e) {
                throw std::runtime_error("Cannot divide an INT and a STR");
            }

            if (num2_i == 0) {
                throw std::runtime_error("Cannot divide by 0.");
            }

            stack.emplace_back(num1_i % num2_i);
        }

        else if (command == "DISPLAY") {
            if (command_parts.size() == 1 || command_parts[1].find_first_not_of(" \t\r\n") == std::string::npos || command_parts.size() < 2) {
                std::cout << std::endl;
            }
            std::string argument = command_parts[1];
            if (argument == "TOP") {
                if (stack.empty()) {
                    throw std::runtime_error("Stack is empty, cannot DISPLAY");
                }
                try {
                    std::cout << std::get<int>(stack.back()) << std::endl;
                } catch (std::exception &e) {
                    std::cout << std::get<std::string>(stack.back()) << std::endl;
                }
            } else if (argument == "STACK") {
                if (stack.empty()) {
                    std::cout << "[]" << std::endl;
                } else {
                    std::cout << "[";
                    for (size_t i = 0; i < stack.size(); ++i) {
                        try {
                            std::cout << std::get<int>(stack[i]);
                        } catch (std::exception &e) {
                            std::cout << std::get<std::string>(stack[i]);
                        }
                        if (i != stack.size() - 1) {
                            std::cout << ", ";
                        }
                    }
                    std::cout << "]" << std::endl;
                }
            } else if (argument == "NEWLN") {
                std::cout << std::endl;
            } else {
                for (size_t i = 1; i < command_parts.size(); ++i) {
                    std::cout << command_parts[i];
                    if (i != command_parts.size() - 1) {
                        std::cout << " ";
                    }
                }
                std::cout << std::endl;
            }
        }

        else if (command == "EQ") {
            if (stack.size() < 2) {
                throw std::runtime_error("Not enough values on the stack to perform EQ");
            }
            auto item1 = stack[stack.size() - 1];
            auto item2 = stack[stack.size() - 2];
            bool equal;
            try {
                if (std::holds_alternative<int>(item1) && std::holds_alternative<int>(item2)) {
                    equal = std::get<int>(item1) == std::get<int>(item2);
                } else if (std::holds_alternative<std::string>(item1) && std::holds_alternative<std::string>(item2)) {
                    equal = std::get<std::string>(item1) == std::get<std::string>(item2);
                } else {
                    equal = false;
                }
            } catch (std::exception &e) {
                throw std::runtime_error("Bad variant access");
            }
            stack.emplace_back(equal ? 1 : 0);
        }

        else if (command == "NEQ") {
            if (stack.size() < 2) {
                throw std::runtime_error("Not enough values on the stack to perform EQ");
            }
            auto item1 = stack[stack.size() - 1];
            auto item2 = stack[stack.size() - 2];
            bool equal;
            try {
                if (std::holds_alternative<int>(item1) && std::holds_alternative<int>(item2)) {
                    equal = std::get<int>(item1) == std::get<int>(item2);
                } else if (std::holds_alternative<std::string>(item1) && std::holds_alternative<std::string>(item2)) {
                    equal = std::get<std::string>(item1) == std::get<std::string>(item2);
                } else {
                    equal = false;
                }
            } catch (std::exception &e) {
                throw std::runtime_error("Bad variant access");
            }
            stack.emplace_back(equal ? 0 : 1);
        }

        else if (command == "GT") {
            if (stack.size() < 2) {
                throw std::runtime_error("Not enough values on the stack to perform GT");
            }
            auto item1 = stack[stack.size() - 1];
            auto item2 = stack[stack.size() - 2];
            bool greater;
            try {
                if (std::holds_alternative<int>(item1) && std::holds_alternative<int>(item2)) {
                    greater = std::get<int>(item1) > std::get<int>(item2);
                } else if (std::holds_alternative<std::string>(item1) && std::holds_alternative<std::string>(item2)) {
                    greater = std::get<std::string>(item1) > std::get<std::string>(item2);
                } else {
                    greater = false;
                }
            } catch (std::exception &e) {
                throw std::runtime_error("Bad variant access");
            }
            stack.emplace_back(greater ? 1 : 0);
        }

        else if (command == "LT") {
            if (stack.size() < 2) {
                throw std::runtime_error("Not enough values on the stack to perform LT");
            }
            auto item1 = stack[stack.size() - 1];
            auto item2 = stack[stack.size() - 2];
            bool less;
            try {
                if (std::holds_alternative<int>(item1) && std::holds_alternative<int>(item2)) {
                    less = std::get<int>(item1) < std::get<int>(item2);
                } else if (std::holds_alternative<std::string>(item1) && std::holds_alternative<std::string>(item2)) {
                    less = std::get<std::string>(item1) < std::get<std::string>(item2);
                } else {
                    less = false;
                }
            } catch (std::exception &e) {
                throw std::runtime_error("Bad variant access");
            }
            stack.emplace_back(less ? 1 : 0);
        }

        else if (command == "AND") {
            if (stack.size() < 2) {
                throw std::runtime_error("Not enough values on the stack to perform AND");
            }
            auto item1 = stack[stack.size() - 1];
            auto item2 = stack[stack.size() - 2];
            try {
                if (std::holds_alternative<int>(item1) && std::holds_alternative<int>(item2)) {
                    stack.emplace_back(std::get<int>(item1) == 1 && std::get<int>(item2) == 1 ? 1 : 0);
                } else {
                    throw std::runtime_error("Cannot perform AND on an INT and a STR");
                }
            } catch (std::bad_variant_access &e) {
                throw std::runtime_error("Bad variant access");
            }
        }

        else if (command == "OR") {
            if (stack.size() < 2) {
                throw std::runtime_error("Not enough values on the stack to perform OR");
            }
            auto item1 = stack[stack.size() - 1];
            auto item2 = stack[stack.size() - 1];
            try {
                if (std::holds_alternative<int>(item1) && std::holds_alternative<int>(item2)) {
                    stack.emplace_back(std::get<int>(item1) != 0 || std::get<int>(item2) != 0 ? 1 : 0);
                } else {
                    throw std::runtime_error("Cannot perform OR on an INT and a STR");
                }
            } catch (std::bad_variant_access &e) {
                throw std::runtime_error("Bad variant access");
            }
        }

        else if (command == "NOT") {
            if (stack.empty()) {
                throw std::runtime_error("Stack is empty, cannot perform NOT");
            }
            auto item = stack[stack.size() - 1];
            if (std::holds_alternative<int>(item)) {
                int item_i = std::get<int>(item) * -1;
                stack.emplace_back(item_i);
            } else {
                throw std::runtime_error("Cannot perform NOT on a STR");
            }
        }

        else if (command == "INC") {
            if (stack.empty()) {
                throw std::runtime_error("Stack is empty, cannot increment");
            }
            try {
                auto item = stack[stack.size() - 1];
                if (std::holds_alternative<int>(item)) {
                    int num = std::get<int>(item);
                    num++;
                    stack.pop_back();
                    stack.emplace_back(num);
                } else {
                    throw std::runtime_error("Cannot increment a STR");
                }
            } catch (std::bad_variant_access &e) {
                throw std::runtime_error("Bad variant access");
            }
        }

        else if (command == "DEC") {
            if (stack.empty()) {
                throw std::runtime_error("Stack is empty, cannot decrement");
            }
            try {
                auto item = stack[stack.size() - 1];
                if (std::holds_alternative<int>(item)) {
                    int num = std::get<int>(item);
                    num--;
                    stack.pop_back();
                    stack.emplace_back(num);
                } else {
                    throw std::runtime_error("Cannot decrement a STR");
                }
            } catch (std::bad_variant_access &e) {
                throw std::runtime_error("Bad variant access");
            }
        }

        else if (command == "READINT") {
            std::string input;
            std::cin >> input;
            try {
                int input_i = stoi(input);
                stack.emplace_back(input_i);
                // Clear the newline character left in the input buffer
                std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
            } catch (std::exception &e) {
                throw std::runtime_error("READINT only accepts integers. Did you mean to use READSTR?");
            }
        }

        else if (command == "READSTR") {
            std::string input;
            std::getline(std::cin, input);
            stack.emplace_back(input);
        }

        else if (command == "SWAP") {
            auto temp = stack[stack.size() - 1];
            stack[stack.size() - 1] = stack[stack.size() - 2];
            stack[stack.size() - 2] = temp;
        }

        else if (command == "JUMP") {
            if (command_parts.size() < 2) {
                throw std::runtime_error("Incorrect JUMP command. Usage: JUMP <line num>");
            }
            int jump_line = stoi(command_parts[1]);
            file.clear();
            file.seekg(0);
            for (int i = 0; i < jump_line; i++) {
                getline(file, line);
            }
            current_line = jump_line - 1;
        }

        else if (command == "JZ") {
            if (command_parts.size() < 2) {
                throw std::runtime_error("Incorrect JZ command. Usage: JZ <line num>");
            }
            int jump_line = stoi(command_parts[1]);
            if (std::get<int>(stack.back()) == 0) {
                file.clear();
                file.seekg(0);
                for (int i = 0; i < jump_line; i++) {
                    getline(file, line);
                }
                current_line = jump_line - 1;
            }
        }

        else if (command == "JNZ") {
            if (command_parts.size() < 2) {
                throw std::runtime_error("Incorrect JNZ command. Usage: JNZ <line num>");
            }
            int jump_line = stoi(command_parts[1]);
            if (std::get<int>(stack.back()) != 0) {
                file.clear();
                file.seekg(0);
                for (int i = 0; i < jump_line; i++) {
                    getline(file, line);
                }
                current_line = jump_line - 1;
            }
        }

//        Uncomment the following lines to print the stack for every line iterated over in the file
//        if (stack.empty()) {
//            std::clog << "[]" << std::endl;
//        } else {
//            std::clog << "[";
//            for (size_t i = 0; i < stack.size(); ++i) {
//                try {
//                    std::clog << std::get<int>(stack[i]);
//                } catch (std::exception &e) {
//                    std::clog << std::get<std::string>(stack[i]);
//                }
//                if (i != stack.size() - 1) {
//                    std::clog << ", ";
//                }
//            }
//            std::clog << "]" << std::endl;
//        }
    }

    std::cout << "Press enter to exit..." << std::endl;
    getchar();

    return EXIT_SUCCESS;
}
