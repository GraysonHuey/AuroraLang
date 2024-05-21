#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <sstream>
#include <variant> // C++ 17 or up needed (replit uses C++ 16)

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


int main([[maybe_unused]] int argc, char** argv) {
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
            for (size_t i = 1; i < command_parts.size(); ++i) {
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
            auto num1_a = stack.back();
            stack.pop_back();
            auto num2_a = stack.back();
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
            auto num2_a = stack.back();
            stack.pop_back();
            auto num1_a = stack.back();
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
            auto num1_a = stack.back();
            stack.pop_back();
            auto num2_a = stack.back();
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
            auto num2_a = stack.back();
            stack.pop_back();
            auto num1_a = stack.back();

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

        else if (command == "DIV") {
            if (stack.size() < 2) {
                throw std::runtime_error("Not enough values in the stack to perform DIV");
            }
            auto num2_a = stack.back();
            stack.pop_back();
            auto num1_a = stack.back();

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

            stack.emplace_back(num1_i % num2_i);
        }

        else if (command == "DISPLAY") {
            if (stack.empty()) {
                throw std::runtime_error("Stack is empty, cannot DISPLAY");
            }
            if (command_parts.size() == 1 || command_parts[1].find_first_not_of(" \t\r\n") == std::string::npos) {
                std::cout << std::endl;
            }
            std::string argument = command_parts[1];
            if (argument == "TOP") {
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
            bool equal = false;
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
            bool equal = true;
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
    }

    return EXIT_SUCCESS;
}
