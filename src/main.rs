#![allow(unused, non_snake_case)]

use std::env;
use std::fs;
use std::process;
use std::path::Path;

mod utils;
mod tokens;
mod generateASM;
use utils::{BOLD, RED, GREEN, RESET, command, DEBUG};
use tokens::{Token, TokType, tokenize};
use generateASM::generateASM;

fn usage(program: String) {
    println!("{RED}[ERROR]: Incorrect command usage!");
    println!("Usage: {program} <file.aur> [output name] [-d || -debug]");
    println!("    -d and -debug: Turn on optional debug info as the source file is compiled{RESET}");
    process::exit(1);
}

fn main() {
    let program: String = env::args().nth(0).unwrap();
    if env::args().len() < 2 || env::args().len() > 4 {
        usage(program);
    }

    let file_name: String = env::args().nth(1).expect("No filename provided");
    if !(Path::new(&file_name).extension().and_then(|ext| ext.to_str()) == Some("aur")) {
        eprintln!("{RED}[ERROR]: Expected '.aur' extension!{RESET}");
        process::exit(1);
    }

    let aurora_source: String = fs::read_to_string(&file_name)
        .map_err(|err| {
            eprintln!("{RED}[ERROR]: {err}{RESET}");
            process::exit(1);
        })
        .unwrap_or_else(|_| "NULL".to_string());

    let debugFlags: Vec<String> = vec!["-d".to_string(), "-debug".to_string()];

    if (env::args().len() == 4 && debugFlags.contains(&env::args().nth(3).unwrap())) || (env::args().len() == 3 && debugFlags.contains(&env::args().nth(2).unwrap())) {
        unsafe { DEBUG = true; }
    }

    let tokens: Vec<Token> = tokenize(&aurora_source);

    unsafe {
        if DEBUG {
            for tok in &tokens {
                println!("[LOG]: {tok:?}");
            }
        }
    }

    generateASM(tokens);

    if !command("fasm", &["output.asm"]) {
        eprintln!("{RED}[ERROR]: Assembly failed!{RESET}");
        process::exit(1);
    }

    let mut output_name: String = env::args().nth(2).unwrap_or_else(|| {"output".to_string()});
    if debugFlags.contains(&output_name) { output_name = "output".to_string(); }

    if !command("ld", &["output.o", "-o", output_name.as_str(), "-lc"]) {
        eprintln!("{RED}[ERROR]: Linking failed!{RESET}");
        process::exit(1);
    }

    unsafe {
        if !DEBUG {
            command("rm", &["output.asm", "output.o"]);
        }
    }

    println!("{GREEN}{BOLD}{file_name} successfully compiled!{RESET}");
}
