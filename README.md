# Welcome to the Aurora Programming Language!

## Overview

This project is a compiler that translates programs written in Aurora (a simple stack-based language) into x86-64 assembly code that can be assembled and linked into executable binaries. The compiler is written in Rust and is designed to be a learning tool for understanding how programming languages and compilers work.

## Features

- Translates Aurora code to x86-64 assembly
- Stack-based operations
- Basic arithmetic operations (addition, subtraction, multiplication, division, modulus)
- Integer and string input/output
- Stack manipulation (dup, swap, rot)
- Simple compilation workflow

## Requirements

- Rust (edition 2018 or newer)
- FASM (Flat Assembler)
- LD (GNU Linker)
- x86-64 Linux environment (for running the compiled executables)

## Installation

1. Clone the repository
2. Make sure you have Rust, FASM, and LD installed
3. Build the project with `cargo build --release`
4. The executable will be available in the `target/release` directory

## Usage

```
./aurora_compiler <file.aur> [output name] [-d | -debug]
```

### Arguments

- `<file.aur>`: Required. The Aurora source file to compile (must have `.aur` extension)
- `[output name]`: Optional. The name of the output executable (defaults to "output")
- `[-d | -debug]`: Optional. Enable debug output during compilation

## Aurora Language Reference

Aurora is a simple stack-based language with the following operations:

### Stack Manipulation

- `+`: Addition - Pops the top two values from the stack, adds them, and pushes the result
- `-`: Subtraction - Pops the top two values from the stack, subtracts the top value from the second value, and pushes the result
- `*`: Multiplication - Pops the top two values from the stack, multiplies them, and pushes the result
- `/`: Division - Pops the top two values from the stack, divides the second value by the top value, and pushes the result
- `%`: Modulus - Pops the top two values from the stack, divides the second value by the top value, and pushes the remainder
- `dup`: Duplicates the top value on the stack
- `swap`: Swaps the top two values on the stack
- `rot`: Rotates the top three values on the stack

### Input/Output

- `iprint`: Prints the top value from the stack as an integer
- `iprintln`: Prints the top value from the stack as an integer followed by a newline
- `sprint`: Prints a string value
- `sprintln`: Prints a string value followed by a newline
- `iread`: Reads an UNSIGNED integer from standard input and pushes it onto the stack (currently working on implementing signedness on iread)
- `sread`: Reads a string from standard input and pushes it onto the stack

### Constants

- Numbers are pushed directly onto the stack (e.g., `42`)
- Strings are enclosed in double quotes (e.g., `"Hello, world!"`)

## Examples

### Basic Arithmetic

```
// addition.aur
+
iprint
```

This program adds 5 and 10, then prints the result (15).

### String Output

```
// hello.aur
"Hello, world!"
sprintln
```

This program prints "Hello, world!" followed by a newline.

## Compilation Process

1. Aurora code is tokenized
2. Tokens are converted to assembly instructions
3. FASM assembles the instructions into an object file
4. LD links the object file into an executable

## Project Structure

- `main.rs`: Entry point and command-line interface
- `tokens.rs`: Tokenization and lexical analysis
- `generateASM.rs`: Code generation for x86-64 assembly
- `utils.rs`: Utility functions and constants

## Acknowledgments

The integer printing functionality in the assembly output is based on code by Tsoding, used under the MIT license.

## License

This project is open-source software.
