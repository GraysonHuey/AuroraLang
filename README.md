# Aurora Language
## Welcome!
## Aurora is a very simple interpreted stack-based language to start my exploration of how programming languages work. As this is a very simple language, not much can be done in it, but to see some use cases, check out our examples below!
<br></br>
<br></br>
# Examples

## Addition
Code

    PUSH 5
    PUSH 10
    ADD
    DISPLAY 5 + 10 = 
    DISPLAY TOP
   
Output

    5 + 10 = 
    15

## Subtraction
Code

    PUSH 8
    PUSH 16
    SUB
    DISPLAY 8 - 16 = 
    DISPLAY TOP

Output

    8 - 16 =
    -8

Note that subtraction takes the topmost number in the stack and subtracts it from the number underneath it.
## Multiplication
Code

    PUSH 12
    PUSH 5
    MUL
    DISPLAY 12 * 5 = 
    DISPLAY TOP

Output

    12 * 5 =
    60

## Division
Code

    PUSH 18
    PUSH 4
    DIV
    DISPLAY 18 / 4 = 
    DISPLAY TOP

Output

    18 / 4 =
    4.5

Note that division divides the second item down in the stack and divides it by the top item
Also, trying to divide by 0 will cause an error
