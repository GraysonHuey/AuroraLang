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

        for line in lines[1:]:
            stripped_line = line.strip()

            if not stripped_line:
                continue  # Skip empty lines

            if stripped_line.startswith("//"):
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
                argument = line.strip()[len("DISPLAY "):]  # Preserve whitespace
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

            else:
                raise Exception(f"Unknown command: {command_parts[0]}")
            # print(f"Stack: {stack}")


if __name__ == "__main__":
    main()
