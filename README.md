# Aurora Language
## Welcome!
Aurora is a very simple interpreted stack-based language to start my exploration of how programming languages work. As this is a very simple language, not much can be done in it, but to see some use cases, check out our examples below!
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

## Comments
A comment is a line of text solely for the programmer to read. It is completely ignored by the interpreter.

    // This is a comment

Comments are denoted by two forward slashes
