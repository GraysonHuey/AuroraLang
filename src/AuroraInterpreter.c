#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_STACK_SIZE 1000
#define MAX_LINE_LENGTH 1000
#define MAX_PROGRAM_LINES 1000

typedef struct {
    int stack[MAX_STACK_SIZE];
    int top;
} Stack;

typedef struct {
    char* lines[MAX_PROGRAM_LINES];
    int line_count;
} Program;

void stack_init(Stack* s) {
    s->top = -1;
}

void stack_push(Stack* s, int value) {
    if (s->top < MAX_STACK_SIZE - 1) {
        s->stack[++s->top] = value;
    } else {
        fprintf(stderr, "Stack overflow\n");
        exit(1);
    }
}

int stack_pop(Stack* s) {
    if (s->top >= 0) {
        return s->stack[s->top--];
    } else {
        fprintf(stderr, "Stack underflow\n");
        exit(1);
    }
}

void stack_clear(Stack* s) {
    s->top = -1;
}

void stack_swap(Stack* s) {
    if (s->top >= 1) {
        int temp = s->stack[s->top];
        s->stack[s->top] = s->stack[s->top - 1];
        s->stack[s->top - 1] = temp;
    } else {
        fprintf(stderr, "Not enough elements to swap\n");
        exit(1);
    }
}

void stack_display(Stack* s, const char* value) {
    if (strcmp(value, "top") == 0) {
        if (s->top >= 0) {
            printf("%d\n", s->stack[s->top]);
        } else {
            printf("Stack is empty\n");
        }
    } else if (strcmp(value, "stack") == 0) {
        printf("Stack: ");
        for (int i = 0; i <= s->top; i++) {
            printf("%d ", s->stack[i]);
        }
        printf("\n");
    } else {
        printf("%s\n", value);
    }
}

void execute_instruction(Stack* s, char* instruction, int* line_num) {
    char* token = strtok(instruction, " ");
    if (token == NULL) return;

    if (strcmp(token, "PUSH") == 0) {
        token = strtok(NULL, " ");
        if (token) stack_push(s, atoi(token));
    } else if (strcmp(token, "POP") == 0) {
        stack_pop(s);
    } else if (strcmp(token, "SWAP") == 0) {
        stack_swap(s);
    } else if (strcmp(token, "CLEAR") == 0) {
        stack_clear(s);
    } else if (strcmp(token, "ADD") == 0) {
        int b = stack_pop(s);
        int a = stack_pop(s);
        stack_push(s, a + b);
    } else if (strcmp(token, "SUB") == 0) {
        int b = stack_pop(s);
        int a = stack_pop(s);
        stack_push(s, a - b);
    } else if (strcmp(token, "MUL") == 0) {
        int b = stack_pop(s);
        int a = stack_pop(s);
        stack_push(s, a * b);
    } else if (strcmp(token, "DIV") == 0) {
        int b = stack_pop(s);
        int a = stack_pop(s);
        if (b != 0) stack_push(s, a / b);
        else {
            fprintf(stderr, "Division by zero\n");
            exit(1);
        }
    } else if (strcmp(token, "MOD") == 0) {
        int b = stack_pop(s);
        int a = stack_pop(s);
        if (b != 0) stack_push(s, a % b);
        else {
            fprintf(stderr, "Modulo by zero\n");
            exit(1);
        }
    } else if (strcmp(token, "DISPLAY") == 0) {
        token = strtok(NULL, " ");
        if (token) stack_display(s, token);
    } else if (strcmp(token, "READ") == 0) {
        int input;
        scanf("%d", &input);
        stack_push(s, input);
    } else if (strcmp(token, "EQ") == 0) {
        int b = stack_pop(s);
        int a = stack_pop(s);
        stack_push(s, a == b ? 1 : 0);
    } else if (strcmp(token, "NEQ") == 0) {
        int b = stack_pop(s);
        int a = stack_pop(s);
        stack_push(s, a != b ? 1 : 0);
    } else if (strcmp(token, "GT") == 0) {
        int b = stack_pop(s);
        int a = stack_pop(s);
        stack_push(s, a > b ? 1 : 0);
    } else if (strcmp(token, "LT") == 0) {
        int b = stack_pop(s);
        int a = stack_pop(s);
        stack_push(s, a < b ? 1 : 0);
    } else if (strcmp(token, "AND") == 0) {
        int b = stack_pop(s);
        int a = stack_pop(s);
        stack_push(s, a && b ? 1 : 0);
    } else if (strcmp(token, "OR") == 0) {
        int b = stack_pop(s);
        int a = stack_pop(s);
        stack_push(s, a || b ? 1 : 0);
    } else if (strcmp(token, "NOT") == 0) {
        int a = stack_pop(s);
        stack_push(s, !a ? 1 : 0);
    } else if (strcmp(token, "INC") == 0) {
        int a = stack_pop(s);
        stack_push(s, a + 1);
    } else if (strcmp(token, "DEC") == 0) {
        int a = stack_pop(s);
        stack_push(s, a - 1);
    } else if (strcmp(token, "JUMP") == 0) {
        token = strtok(NULL, " ");
        if (token) *line_num = atoi(token) - 2;  // -2 because we increment after
    } else if (strcmp(token, "JZ") == 0) {
        int a = stack_pop(s);
        token = strtok(NULL, " ");
        if (token && a == 0) *line_num = atoi(token) - 2;
    } else if (strcmp(token, "JNZ") == 0) {
        int a = stack_pop(s);
        token = strtok(NULL, " ");
        if (token && a != 0) *line_num = atoi(token) - 2;
    } else {
        fprintf(stderr, "Unknown instruction: %s\n", token);
        exit(1);
    }
}

void load_program(Program* p, const char* filename) {
    FILE* file = fopen(filename, "r");
    if (file == NULL) {
        fprintf(stderr, "Could not open file %s\n", filename);
        exit(1);
    }

    char line[MAX_LINE_LENGTH];
    while (fgets(line, sizeof(line), file)) {
        // Remove newline character
        line[strcspn(line, "\n")] = 0;
        
        // Allocate memory for the line and copy it
        p->lines[p->line_count] = malloc(strlen(line) + 1);
        strcpy(p->lines[p->line_count], line);
        p->line_count++;

        if (p->line_count >= MAX_PROGRAM_LINES) {
            fprintf(stderr, "Program too long\n");
            exit(1);
        }
    }

    fclose(file);
}

void run_program(Program* p) {
    Stack stack;
    stack_init(&stack);

    if (strcmp(p->lines[0], "START") != 0 || strcmp(p->lines[p->line_count - 1], "END") != 0) {
        fprintf(stderr, "Program must start with START and end with END\n");
        exit(1);
    }

    for (int i = 1; i < p->line_count - 1; i++) {
        execute_instruction(&stack, p->lines[i], &i);
    }
}

int main(int argc, char* argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Usage: %s <filename>\n", argv[0]);
        return 1;
    }

    Program program = {0};
    load_program(&program, argv[1]);
    run_program(&program);

    // Free allocated memory
    for (int i = 0; i < program.line_count; i++) {
        free(program.lines[i]);
    }

    return 0;
}
