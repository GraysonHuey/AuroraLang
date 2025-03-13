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

    // Syscalls used are all for x86_64 (64-bit) Linux as defined by: https://chromium.googlesource.com/chromiumos/docs/+/master/constants/syscalls.md#x86_64-64_bit

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
                file.write(b"    push rax\n");
                strings.push(tok.data.clone());
                let length = format!("    mov rax, str{0}_len\n", curStr);
                file.write(length.as_bytes());
                file.write(b"    push rax\n\n");
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
                file.write(b"    pop rbx\n");
                file.write(b"    pop rax\n");
                file.write(b"    xor rdx, rdx\n");
                file.write(b"    idiv rbx\n");
                file.write(b"    push rax\n\n");
            }
            TokType::MOD => {
                file.write(b"    ; -- MOD --\n");
                file.write(b"    pop rbx\n");
                file.write(b"    pop rax\n");
                file.write(b"    xor rdx, rdx\n");
                file.write(b"    idiv rbx\n");
                file.write(b"    push rdx\n\n");
            }
            TokType::IPRINT => {
                file.write(b"    ; -- PRINT INT --\n");
                file.write(b"    pop rdi\n");
                file.write(b"    call iprint\n\n");
            }
            TokType::IPRINTLN => {
                file.write(b"    ; -- PRINT INT NEWLN -- \n");
                file.write(b"    pop rdi\n");
                file.write(b"    call iprintln\n\n");
            }
            TokType::SPRINT => {
                file.write(b"    ; -- PRINT STR --\n");
                file.write(b"    mov rax, 1   ; write\n");
                file.write(b"    mov rdi, 1   ; stdout\n");
                file.write(b"    pop rdx      ; count\n");
                file.write(b"    pop rsi      ; buf\n");
                file.write(b"    syscall\n\n");
            }
            TokType::SPRINTLN => {
                file.write(b"    ; -- PRINT STR NEWLN --\n");
                file.write(b"    mov rax, 1              ; write\n");
                file.write(b"    mov rdi, 1              ; stdout\n");
                file.write(b"    pop rdx                 ; count\n");
                file.write(b"    pop rsi                 ; buf\n");
                file.write(b"    mov BYTE [rsi+rdx], 10  ; Add newline to the end of the string\n");
                file.write(b"    inc rdx                 ; Add one to length (because of the added newline)\n");
                file.write(b"    syscall\n\n");
            }
            TokType::IREAD => {
                file.write(b"    ; -- READ INT -- \n");
                file.write(b"    mov rax, 0      ; read\n");
                file.write(b"    mov rdi, 0      ; stdin\n");
                file.write(b"    mov rsi, intBuf ; buf\n");
                file.write(b"    mov rdx, 20     ; count\n");
                file.write(b"    syscall\n");

                file.write(b"    mov rcx, rax      ; Save length of input\n");
                file.write(b"    mov rbx, 0\n");
                file.write(b"    mov bl, byte [intBuf+rcx-1]\n");
                file.write(b"    cmp bl, 10        ; Check for newline\n");
                file.write(b"    jne .skip_newline\n");
                file.write(b"    mov byte [intBuf+rcx-1], 0  ; Replace newline with null\n");
                file.write(b"    dec rcx           ; Adjust length\n");
                file.write(b".skip_newline:\n");
                file.write(b"    mov rsi, intBuf   ; Source string\n");
                file.write(b"    call atoi         ; Convert to integer\n");
                file.write(b"    push rax          ; Push the integer value\n\n");
            }
            TokType::SREAD => {
                file.write(b"    ; -- READ STR --\n");
                file.write(b"    mov rax, 0      ; read\n");
                file.write(b"    mov rdi, 0      ; stdin\n");
                file.write(b"    mov rsi, strBuf ; buf\n");
                file.write(b"    mov rdx, 1024   ; count\n");
                file.write(b"    syscall\n");

                file.write(b"    mov rcx, rax      ; Save length of input\n");
                file.write(b"    mov rbx, 0\n");
                file.write(b"    mov bl, byte [strBuf+rcx-1]\n");
                file.write(b"    cmp bl, 10        ; Check for newline\n");
                file.write(b"    jne .continue_sread\n");
                file.write(b"    dec rcx           ; Don't include newline in length\n");
                file.write(b".continue_sread:\n");
                file.write(b"    mov rax, strBuf   ; Get buffer address\n");
                file.write(b"    push rax          ; Push string address\n");
                file.write(b"    mov rax, rcx      ; Get string length\n");
                file.write(b"    push rax          ; Push string length\n\n");
            }
            TokType::DUP => {
                file.write(b"    ; -- DUP -- \n");
                file.write(b"    pop rax\n");
                file.write(b"    push rax\n");
                file.write(b"    push rax\n\n");
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
    file.write(b"iprintln:\n");
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

    file.write(b"iprint:\n");
    file.write(b"    mov     r9, -3689348814741910323\n");
    file.write(b"    sub     rsp, 40\n");
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

    file.write(b"atoi:\n");
    file.write(b"    xor rax, rax         ; Initialize result to 0\n");
    file.write(b"    xor rcx, rcx         ; Initialize index to 0\n");
    file.write(b"    xor rbx, rbx         ; Initialize temporary\n");
    file.write(b"    mov rdx, 10          ; Base 10\n");
    file.write(b"    \n");
    file.write(b"    ; Check for negative sign\n");
    file.write(b"    mov r8, 0            ; Initialize sign flag to 0 (positive)\n");
    file.write(b"    mov bl, byte [rsi]   ; Get first character\n");
    file.write(b"    cmp bl, '-'          ; Is it negative?\n");
    file.write(b"    jne .atoi_loop       ; If not, start conversion\n");
    file.write(b"    inc rsi              ; Skip the negative sign\n");
    file.write(b"    mov r8, 1            ; Set sign flag to 1 (negative)\n");
    file.write(b"    \n");
    file.write(b".atoi_loop:\n");
    file.write(b"    mov bl, byte [rsi+rcx] ; Get next character\n");
    file.write(b"    cmp bl, 0            ; Check for null terminator\n");
    file.write(b"    je .atoi_done        ; If null, we're done\n");
    file.write(b"    cmp bl, '0'          ; Check if below '0'\n");
    file.write(b"    jl .atoi_done        ; If below '0', we're done\n");
    file.write(b"    cmp bl, '9'          ; Check if above '9'\n");
    file.write(b"    jg .atoi_done        ; If above '9', we're done\n");
    file.write(b"    \n");
    file.write(b"    sub bl, '0'          ; Convert from ASCII to number\n");
    file.write(b"    imul rax, rdx        ; Multiply result by 10\n");
    file.write(b"    add rax, rbx         ; Add current digit\n");
    file.write(b"    \n");
    file.write(b"    inc rcx              ; Move to next character\n");
    file.write(b"    jmp .atoi_loop\n");
    file.write(b"    \n");
    file.write(b".atoi_done:\n");
    file.write(b"    cmp r8, 1            ; Was it negative?\n");
    file.write(b"    jne .atoi_return     ; If not, return\n");
    file.write(b"    neg rax              ; If negative, negate result\n");
    file.write(b".atoi_return:\n");
    file.write(b"    ret\n");


    file.write(b"\nsection '.data' writeable\n");
    file.write(b"    formatStr: db \"%s\", 10\n");
    file.write(b"    intBuf: times 21 db 0\n");
    file.write(b"    strBuf: times 1024 db 0\n\n");

    curStr = 0;
    for string in strings {
        let name = format!("    str{}: db ", curStr);
        file.write(name.as_bytes());

        for (i, c) in string.bytes().enumerate() {
            if i > 0 {
                file.write(b", ");
            }
            let byte = format!("{}", c);
            file.write(byte.as_bytes());
        }

        file.write(b", 0\n");

        let length = format!("    str{}_len = $ - str{}\n", curStr, curStr);
        file.write(length.as_bytes());
        curStr += 1;
    }

    file.write(b"\nsection '.note.GNU-stack'");
}
