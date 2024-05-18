import sys


def main():
    stack = []

    file = sys.argv[1]
    if file[-4:] != ".aur":
        raise Exception("Not an Aurora (.aur) file.")

    with open(file, 'r') as filp:
        lines = filp.readlines()
        if lines[0].strip() != "START":
            raise Exception("Start command not found.")

        if not any(line.strip() == "END" for line in lines):
            raise Exception("End command not found.")

        current_line = 1
        while current_line < len(lines):
            stripped_line = lines[current_line].strip()

            if not stripped_line:
                current_line += 1
                continue  # Skip empty lines

            if stripped_line.startswith("//"):
                current_line += 1
                continue

            if stripped_line.startswith("END"):
                break

            command_parts = stripped_line.split()

            if command_parts[0] == "PUSH":
                try:
                    num = int(command_parts[1])
                    stack.append(num)
                except (ValueError, IndexError):
                    raise Exception("Invalid PUSH command. Usage: PUSH <number>")

            elif command_parts[0] == "POP":
                if not stack:
                    raise Exception("Stack is empty, cannot POP.")
                stack.pop()

            elif command_parts[0] == "ADD":
                if len(stack) < 2:
                    raise Exception("Not enough values on the stack to perform ADD.")
                num1 = stack.pop()
                num2 = stack.pop()
                stack.append(num1 + num2)

            elif command_parts[0] == "SUB":
                if len(stack) < 2:
                    raise Exception("Not enough values on the stack to perform SUB.")
                num2 = stack.pop()
                num1 = stack.pop()
                stack.append(num1 - num2)

            elif command_parts[0] == "MUL":
                if len(stack) < 2:
                    raise Exception("Not enough values on the stack to perform MUL.")
                num1 = stack.pop()
                num2 = stack.pop()
                stack.append(num1 * num2)

            elif command_parts[0] == "DIV":
                if len(stack) < 2:
                    raise Exception("Not enough values on the stack to perform DIV.")
                num2 = stack.pop()
                num1 = stack.pop()
                if num2 == 0:
                    raise Exception("Cannot divide by zero.")
                stack.append(num1 / num2)

            elif command_parts[0] == "MOD":
                if len(stack) < 2:
                    raise Exception("Not enough values on the stack to perform MOD.")
                num2 = stack.pop()
                num1 = stack.pop()
                if num2 == 0:
                    raise Exception("Cannot divide by zero.")
                stack.append(num1 % num2)

            elif command_parts[0] == "DISPLAY":
                argument = lines[current_line].strip()[len("DISPLAY "):]  # Preserve whitespace
                if argument.strip() == "TOP":
                    if stack:
                        print(stack[-1])
                    else:
                        raise Exception("Stack is empty, cannot DISPLAY.")
                elif argument.strip() == "STACK":
                    if stack:
                        print(stack)
                    else:
                        print("[]")
                else:
                    print(argument)

            elif command_parts[0] == "EQ":
                if len(stack) < 2:
                    raise Exception("Not enough values on the stack to perform EQ.")
                num1 = stack.pop()
                num2 = stack.pop()
                stack.append(1 if num1 == num2 else 0)

            elif command_parts[0] == "NEQ":
                if len(stack) < 2:
                    raise Exception("Not enough values on the stack to perform NEQ.")
                num1 = stack.pop()
                num2 = stack.pop()
                stack.append(1 if num1 != num2 else 0)

            elif command_parts[0] == "GT":
                if len(stack) < 2:
                    raise Exception("Not enough values on the stack to perform GT.")
                num1 = stack.pop()
                num2 = stack.pop()
                stack.append(1 if num1 > num2 else 0)

            elif command_parts[0] == "LT":
                if len(stack) < 2:
                    raise Exception("Not enough values on the stack to perform LT.")
                num1 = stack.pop()
                num2 = stack.pop()
                stack.append(1 if num1 < num2 else 0)

            elif command_parts[0] == "AND":
                if len(stack) < 2:
                    raise Exception("Not enough values on the stack to perform AND.")
                num1 = stack.pop()
                num2 = stack.pop()
                stack.append(num1 and num2)

            elif command_parts[0] == "OR":
                if len(stack) < 2:
                    raise Exception("Not enough values on the stack to perform OR.")
                num1 = stack.pop()
                num2 = stack.pop()
                stack.append(num1 or num2)

            elif command_parts[0] == "NOT":
                if not stack:
                    raise Exception("Stack is empty, cannot perform NOT")
                num = stack.pop()
                stack.append(0 if num else 1)

            elif command_parts[0] == "INC":
                if not stack:
                    raise Exception("Stack is empty, cannot perform INC")
                stack[-1] += 1

            elif command_parts[0] == "DEC":
                if not stack:
                    raise Exception("Stack is empty, cannot perform DEC")
                stack[-1] -= 1

            elif command_parts[0] == "READ":
                try:
                    num = int(input("Enter a number: "))
                    stack.append(num)
                except ValueError:
                    raise Exception("Invalid input. Expected an integer")

            elif command_parts[0] == "JUMP":
                try:
                    line_num = int(command_parts[1])
                    if line_num < 0 or line_num >= len(lines):
                        raise Exception("Invalid line number for JUMP.")
                    current_line = line_num - 1  # Adjust for zero indexing
                    continue
                except (ValueError, IndexError):
                    raise Exception("Invalid JUMP command. Usage: JUMP <line_number>")

            elif command_parts[0] == "JZ":
                if not stack:
                    raise Exception("Stack is empty, cannot perform JZ.")
                num = stack.pop()
                if num == 0:
                    try:
                        line_num = int(command_parts[1])
                        if line_num < 0 or line_num >= len(lines):
                            raise Exception("Invalid line number for JZ.")
                        current_line = line_num - 1  # Adjust for zero indexing
                        continue
                    except (ValueError, IndexError):
                        raise Exception("Invalid JZ command. Usage: JZ <line_number>")

            elif command_parts[0] == "JNZ":
                if not stack:
                    raise Exception("Stack is empty, cannot perform JNZ")
                num = stack.pop()
                if num != 0:
                    try:
                        line_num = int(command_parts[1])
                        if line_num < 0 or line_num >= len(lines):
                            raise Exception("Invalid line number for JNZ.")
                        current_line = line_num - 1  # Adjust for zero indexing
                        continue
                    except (ValueError, IndexError):
                        raise Exception("Invalid JNZ command. Usage: JNZ <line_number>")

            else:
                raise Exception(f"Unknown command: {command_parts[0]}")

            current_line += 1

            # Uncomment the next line to print the stack after each command for debugging
            # print(f"Stack: {stack}")

    input("Press enter to exit...")


if __name__ == "__main__":
    main()
