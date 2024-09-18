
# Welcome to the Aurora programming language!
Aurora is a very simple interpreted stack-based language to start my exploration of how programming languages work. When writing this language, I was inspired by the syntax of BASIC as well as some assembly instructions. As this is a very simple language, not much can be done in it, but to see some use cases, check out our examples below!
<br></br>
## What am I doing next?
My next step is to add a couple more commands to this language, and then I'll start working on converting the interpreter into a compiler.
<br></br>
# Quick Reference

    START & END → Required at the beginning and end of the Aurora file
    
    PUSH {value} → Pushes a number to the top of the stack
    
    POP → Discards the top element of the stack

    SWAP → Swap the top two elements of the stack
    
    CLEAR → Remove all elements from the stack

    ADD → Removes the top two elements of the stack, adds them, and inserts the result onto the stack
    
    SUB → Removes the top two elements of the stack, subtracts the top from the number underneath it, and inserts the result onto the stack

	MUL → Removes the top two elements of the stack, multiplies them, and inserts the result onto the stack

	DIV → Removes the top two elements of the stack, divides the one underneath by the top, and inserts the result onto the stack

	MOD → Removes the top two elements of the stack, divides the one underneath by the top, and inserts the remainer onto the stack

	DISPLAY {value} → Prints the specified value. Can print the top of the stack, entire stack, or a string

	READ → Gets an integer input from the user and inserts it onto the top of the stack

	EQ → Inserts a 1 to the top of the stack if the current top two values are equal (otherwise, 0 is inserted)

	NEQ → Inserts a 0 to the top of the stack if the current top two values are not equal (otherwise, 1 is inserted)

	GT → Inserts a 1 to the top of the stack if the current top value is greater than the value of the item below it (otherwise, 0 is inserted)

	LT → Inserts a 1 to the top of the stack if the current top value is less than the value of the item below it (otherwise, 0 is inserted)

	AND → Performs an AND operation on the top two elements of the stack and inserts the result
	
	OR → Performs an OR operation on the top two elements of the stack and inserts the result

	NOT → Performs a NOT operation on the top element of the stack and inserts the result

	INC → Increments the top element of the stack

	DEC → Decrements the top element of the stack

	JUMP {line num} → Jumps to the specified line number

	JZ {line num} → Jumps to the specified line number if the top element of the stack is 0

	JNZ {line num} → Jumps to the specified line number if the top element of the stack is not 0

<br></br>
<br></br>
# Examples/Documentation

## Addition
Code

    START
    
    PUSH 5
    PUSH 10
    ADD
    DISPLAY 5 + 10 = 
    DISPLAY TOP
    
    END
   
Output

    5 + 10 = 
    15

## Subtraction
Code

    START
    
    PUSH 8
    PUSH 16
    SUB
    DISPLAY 8 - 16 = 
    DISPLAY TOP
	
	END

Output

    8 - 16 =
    -8

Note that subtraction takes the topmost number in the stack and subtracts it from the number underneath it.
## Multiplication
Code

    START
    
    PUSH 12
    PUSH 5
    MUL
    DISPLAY 12 * 5 = 
    DISPLAY TOP

	END

Output

    12 * 5 =
    60

## Division
Code

    START
    
    PUSH 18
    PUSH 4
    DIV
    DISPLAY 18 / 4 = 
    DISPLAY TOP
    
    END

Output

    18 / 4 =
    4.5

Note that division takes the second item down in the stack and divides it by the top item
Also, trying to divide by 0 will cause an error
## Modulus
Code

    START  
  
    PUSH 11  
    PUSH 4  
    MOD  
      
    DISPLAY 11 % 4 =  
    DISPLAY TOP  
      
    END
Output

    11 % 4 = 
    3

Note that modulus takes the second item down in the stack, divides it by the top item, and returns the remainder
Also, trying to mod by 0 will cause an error

## Removing items from the stack
Code

    START
    
	PUSH 5
	DISPLAY STACK
	POP
	DISPLAY STACK
	
	END

Output

    [5]
    []

The pop command simply takes the top item of the stack and discards it

<br></br>

Code

    START
    PUSH 5
    PUSH 7
    DISPLAY STACK
    CLEAR
    DISPLAY STACK
Output

    [5, 7]
    []
The clear command removes every single element of the stack and resets it to how it is at the beginning of runtime

## Comments
A comment is a line of text solely for the programmer to read. It is completely ignored by the interpreter.

    // This is a comment

Comments are denoted by two forward slashes
