use std::env;
use std::fs;
use std::process;
use std::path::Path;

mod utils;
mod tokens;
mod generateASM;
use utils::{BOLD, RED, GREEN, RESET, command};
use tokens::{Token, TokType, tokenize};
use generateASM::generateASM;

fn usage(program: String) {
    println!("{RED}[ERROR]: Incorrect command usage!");
    println!("Usage: {program} <file.aur> [output name]{RESET}");
    process::exit(1);
}

fn main() {
    let program: String = env::args().nth(0).unwrap();
    if env::args().len() < 2 || env::args().len() > 3 {
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

    let tokens: Vec<Token> = tokenize(aurora_source);

    for tok in &tokens {
        println!("{tok:?}");
    }

    generateASM(tokens);

    command("fasm", &["output.asm"]);

    let output_name: String = env::args().nth(2).unwrap_or_else(|| {"output".to_string()});

    command("ld", &["output.o", "-o", output_name.as_str(), "-lc"]);
    //command("chmod", &["+x", "output"]);

    /*if output_name != "output".to_string() {
        command("mv", &["output", output_name.as_str()]);
    }*/

    println!("{GREEN}{BOLD}{file_name} successfully compiled!{RESET}");
}
