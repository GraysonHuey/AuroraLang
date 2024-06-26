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
            command = command_parts[0]

            match command:
                case "PUSHINT":
                    try:
                        num = int(command_parts[1])
                        stack.append(num)
                    except (ValueError, IndexError):
                        raise Exception("Invalid PUSH command. Usage: PUSH <number>")

                case "PUSHSTR":
                    string = ' '.join(command_parts[1:])
                    stack.append(string)

                case "POP":
                    if not stack:
                        raise Exception("Stack is empty, cannot POP.")
                    stack.pop()

                case "CLEAR":
                    if not stack:
                        raise Exception("Stack is already empty, cannot CLEAR")
                    stack.clear()

                case "ADD":
                    if len(stack) < 2:
                        raise Exception("Not enough values on the stack to perform ADD.")
                    num1 = stack.pop()
                    num2 = stack.pop()
                    if isinstance(num1, str) or isinstance(num1, str):
                        raise Exception("Cannot add an INT and a STR")
                    stack.append(num1 + num2)

                case "SUB":
                    if len(stack) < 2:
                        raise Exception("Not enough values on the stack to perform SUB.")
                    num2 = stack.pop()
                    num1 = stack.pop()
                    if isinstance(num1, str) or isinstance(num1, str):
                        raise Exception("Cannot subtract an INT and a STR")
                    stack.append(num1 - num2)

                case "MUL":
                    if len(stack) < 2:
                        raise Exception("Not enough values on the stack to perform MUL.")
                    num1 = stack.pop()
                    num2 = stack.pop()
                    if isinstance(num1, str) or isinstance(num1, str):
                        raise Exception("Cannot multiply an INT and a STR")
                    stack.append(num1 * num2)

                case "DIV":
                    if len(stack) < 2:
                        raise Exception("Not enough values on the stack to perform DIV.")
                    num2 = stack.pop()
                    num1 = stack.pop()
                    if isinstance(num1, str) or isinstance(num1, str):
                        raise Exception("Cannot divide an INT and a STR")
                    if num2 == 0:
                        raise Exception("Cannot divide by zero.")
                    stack.append(num1 / num2)

                case "MOD":
                    if len(stack) < 2:
                        raise Exception("Not enough values on the stack to perform MOD.")
                    num2 = stack.pop()
                    num1 = stack.pop()
                    if isinstance(num1, str) or isinstance(num1, str):
                        raise Exception("Cannot divide an INT and a STR")
                    if num2 == 0:
                        raise Exception("Cannot divide by zero.")
                    stack.append(num1 % num2)

                case "DISPLAY":
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
                    elif argument.strip() == "NEWLN":
                        print("\n")
                    elif argument.strip() == "":
                        raise Exception("Provide an argument to output. Options: (Any STR, TOP, STACK, NEWLN)")
                    else:
                        print(argument)

                case "EQ":
                    if len(stack) < 2:
                        raise Exception("Not enough values on the stack to perform EQ.")
                    num1 = stack[-1]
                    num2 = stack[-2]
                    stack.append(1 if num1 == num2 else 0)

                case "NEQ":
                    if len(stack) < 2:
                        raise Exception("Not enough values on the stack to perform NEQ.")
                    num1 = stack[-1]
                    num2 = stack[-2]
                    stack.append(1 if num1 != num2 else 0)

                case "GT":
                    if len(stack) < 2:
                        raise Exception("Not enough values on the stack to perform GT.")
                    num1 = stack[-1]
                    num2 = stack[-2]
                    stack.append(1 if num1 > num2 else 0)

                case "LT":
                    if len(stack) < 2:
                        raise Exception("Not enough values on the stack to perform LT.")
                    num1 = stack[-1]
                    num2 = stack[-2]
                    stack.append(1 if num1 < num2 else 0)

                case "AND":
                    if len(stack) < 2:
                        raise Exception("Not enough values on the stack to perform AND.")
                    num1 = stack[-1]
                    num2 = stack[-2]
                    stack.append(num1 and num2)

                case "OR":
                    if len(stack) < 2:
                        raise Exception("Not enough values on the stack to perform OR.")
                    num1 = stack[-1]
                    num2 = stack[-2]
                    stack.append(num1 or num2)

                case "NOT":
                    if not stack:
                        raise Exception("Stack is empty, cannot perform NOT")
                    num = stack[-1]
                    stack.append(0 if num else 1)

                case "INC":
                    if not stack:
                        raise Exception("Stack is empty, cannot perform INC")
                    if isinstance(stack[-1], str):
                        raise Exception("Cannot increment a STR")
                    stack[-1] += 1

                case "DEC":
                    if not stack:
                        raise Exception("Stack is empty, cannot perform DEC")
                    if isinstance(stack[-1], str):
                        raise Exception("Cannot decrement a STR")
                    stack[-1] -= 1

                case "READINT":
                    try:
                        num = int(input())
                        stack.append(num)
                    except ValueError:
                        raise Exception("Invalid input. Expected an integer")

                case "READSTR":
                    stack.append(input())

                case "JUMP":
                    try:
                        line_num = int(command_parts[1])
                        if line_num < 0 or line_num >= len(lines):
                            raise Exception("Invalid line number for JUMP.")
                        current_line = line_num - 1  # Adjust for zero indexing
                        continue
                    except (ValueError, IndexError):
                        raise Exception("Invalid JUMP command. Usage: JUMP <line_number>")

                case "JZ":
                    if not stack:
                        raise Exception("Stack is empty, cannot perform JZ.")
                    num = stack[-1]
                    if num == 0:
                        try:
                            line_num = int(command_parts[1])
                            if line_num < 0 or line_num >= len(lines):
                                raise Exception("Invalid line number for JZ.")
                            current_line = line_num - 1  # Adjust for zero indexing
                            continue
                        except (ValueError, IndexError):
                            raise Exception("Invalid JZ command. Usage: JZ <line_number>")

                case "JNZ":
                    if not stack:
                        raise Exception("Stack is empty, cannot perform JNZ")
                    num = stack[-1]
                    if num != 0:
                        try:
                            line_num = int(command_parts[1])
                            if line_num < 0 or line_num >= len(lines):
                                raise Exception("Invalid line number for JNZ.")
                            current_line = line_num - 1  # Adjust for zero indexing
                            continue
                        except (ValueError, IndexError):
                            raise Exception("Invalid JNZ command. Usage: JNZ <line_number>")

                case "SWAP":
                    if not stack:
                        raise Exception("Stack is empty, cannot perform SWAP")
                    if len(stack) < 2:
                        raise Exception("Not enough values on the stack to perform SWAP")
                    stack[-1], stack[-2] = stack[-2], stack[-1]

                case _:
                    raise Exception(f"Unknown command: {command_parts[0]}")

            current_line += 1

        # Uncomment the next line to print the stack after each command for debugging
        # print(f"Stack: {stack}")

    input("Press enter to exit...")


if __name__ == "__main__":
    main()
