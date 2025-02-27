#![allow(unused, non_snake_case)]

use std::fs::File;
use std::io::Write;

use crate::tokens::{Token, TokType};

#[allow(unused_must_use)]
pub fn generateASM(tokens: Vec<Token>) {
    let mut file = File::create("output.asm").unwrap();

    let mut strings: Vec<String> = Vec::new();
    let mut curStr: u32 = 0;

    file.write(b"format ELF64\n\n");
    file.write(b"section '.text' executable\n");
    file.write(b"public _start\n\n");
    file.write(b"extrn printf\n\n");
    file.write(b"_start:\n");

    for tok in &tokens {
        match tok.typ {
            TokType::INT => {
                file.write(b"    ; -- INT --\n");
                let value = format!("    mov rax, {0}\n", tok.val);
                file.write(value.as_bytes());
                file.write(b"    push rax\n\n");
            }
            TokType::STR => {
                file.write(b"    ; -- STR --\n");
                let value = format!("    mov rax, str{0}\n", curStr);
                file.write(value.as_bytes());
                file.write(b"    push rax\n\n");
                strings.push(tok.data.clone());
                curStr += 1;
            }
            TokType::ADD => {
                file.write(b"    ; -- ADD --\n");
                file.write(b"    pop rax\n");
                file.write(b"    pop rbx\n");
                file.write(b"    add rax, rbx\n");
                file.write(b"    push rax\n\n");
            }
            TokType::SUB => {
                file.write(b"    ; -- SUB --\n");
                file.write(b"    pop rbx\n");
                file.write(b"    pop rax\n");
                file.write(b"    sub rax, rbx\n");
                file.write(b"    push rax\n\n");
            }
            TokType::MUL => {
                file.write(b"    ; -- MUL --\n");
                file.write(b"    pop rax\n");
                file.write(b"    pop rbx\n");
                file.write(b"    imul rax, rbx\n");
                file.write(b"    push rax\n\n");
            }
            TokType::DIV => {
                file.write(b"    ; -- DIV --\n");
                file.write(b"    pop rax\n");
                file.write(b"    pop rbx\n");
                file.write(b"    xor rdx, rdx\n");
                file.write(b"    idiv rbx\n");
                file.write(b"    push rax\n\n");
            }
            TokType::MOD => {
                file.write(b"    ; -- MOD --\n");
                file.write(b"    pop rax\n");
                file.write(b"    pop rbx\n");
                file.write(b"    xor rdx, rdx\n");
                file.write(b"    idiv rbx\n");
                file.write(b"    push rdx\n\n");
            }
            TokType::IPRINT => {
                file.write(b"    ; -- PRINT --\n");
                file.write(b"    pop rdi\n");
                file.write(b"    call iprint\n\n");
            }
            TokType::SPRINT => {

            }
            TokType::SWAP => {
                file.write(b"    ; -- SWAP --\n");
                file.write(b"    pop rax\n");
                file.write(b"    pop rbx\n");
                file.write(b"    push rax\n");
                file.write(b"    push rbx\n\n");
            }
            TokType::ROT => {
                file.write(b"    ; -- ROT --\n");
                file.write(b"    pop rax\n");
                file.write(b"    pop rbx\n");
                file.write(b"    pop rcx\n");
                file.write(b"    push rbx\n");
                file.write(b"    push rax\n");
                file.write(b"    push rcx\n\n");
            }
            TokType::END => {
                file.write(b"    ; EXIT\n");
                file.write(b"    mov rax, 60\n");
                file.write(b"    mov rdi, 0\n");
                file.write(b"    syscall\n\n");
            }
        }
    }
    file.write(b"; CREDITS TO TSODING FOR THE PRINT FUNCTION (htps://gitlab.com/tsoding/porth)\n; This should fall under fair use as defined by the MIT licence used by Tsoding\n");
    file.write(b"iprint:\n");
    file.write(b"    mov     r9, -3689348814741910323\n");
    file.write(b"    sub     rsp, 40\n");
    file.write(b"    mov     BYTE [rsp+31], 10\n");
    file.write(b"    lea     rcx, [rsp+30]\n");
    file.write(b".L2:\n");
    file.write(b"    mov     rax, rdi\n");
    file.write(b"    lea     r8, [rsp+32]\n");
    file.write(b"    mul     r9\n");
    file.write(b"    mov     rax, rdi\n");
    file.write(b"    sub     r8, rcx\n");
    file.write(b"    shr     rdx, 3\n");
    file.write(b"    lea     rsi, [rdx+rdx*4]\n");
    file.write(b"    add     rsi, rsi\n");
    file.write(b"    sub     rax, rsi\n");
    file.write(b"    add     eax, 48\n");
    file.write(b"    mov     BYTE [rcx], al\n");
    file.write(b"    mov     rax, rdi\n");
    file.write(b"    mov     rdi, rdx\n");
    file.write(b"    mov     rdx, rcx\n");
    file.write(b"    sub     rcx, 1\n");
    file.write(b"    cmp     rax, 9\n");
    file.write(b"    ja      .L2\n");
    file.write(b"    lea     rax, [rsp+32]\n");
    file.write(b"    mov     edi, 1\n");
    file.write(b"    sub     rdx, rax\n");
    file.write(b"    xor     eax, eax\n");
    file.write(b"    lea     rsi, [rsp+32+rdx]\n");
    file.write(b"    mov     rdx, r8\n");
    file.write(b"    mov     rax, 1\n");
    file.write(b"    syscall\n");
    file.write(b"    add     rsp, 40\n");
    file.write(b"    ret\n\n");

    file.write(b"\nsection '.data' writeable\n");
    file.write(b"    formatStr: dq \"%d\", 10\n\n");

    curStr = 0;
    for string in strings {
        let name = format!("    str{0}: dq \"{1}\", 0\n", curStr, string.clone());
        file.write(name.as_bytes());
        curStr += 1;
    }

    file.write(b"\nsection '.note.GNU-stack'");
}
