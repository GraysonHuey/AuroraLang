use std::fs::File;
use std::io::Write;

use crate::tokens::{Token};

#[allow(unused_must_use)]
pub fn generateASM(tokens: Vec<Token>) {
    let mut file = File::create("output.asm").unwrap();

    file.write(b"format ELF64 executable\n\n");
    file.write(b"entry start\n\n");

    file.write(b"segment readable executable\n\n");
    file.write(b"start:\n");

    file.write(b"    ; EXIT\n");
    file.write(b"    mov rax, 60\n");
    file.write(b"    mov rdi, 0\n");
    file.write(b"    syscall\n\n");
}
