use std::convert::TryInto;
use std::env;
use std::fs;
use std::process;
use std::path::Path;
use std::str::FromStr;

const BOLD: &str = "\x1b[1m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

#[derive(Debug)]
enum TokType {
    INT,
    STR,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    PRINT,
    SWAP,
    ROT,
    END,
}

#[derive(Debug)]
struct Token {
    typ: TokType,
    val: i32,
    data: String,
}

impl Token {
    fn new(typ: TokType, val: i32, data: String) -> Self {
        Self {
            typ,
            val,
            data,
        }
    }
}

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

    for tok in tokens {
        println!("{tok:?}");
    }
}

fn tokenize(source: String) -> Vec<Token> {
    let split_str: Vec<&str> = source.split(&[' ', '\n'][..]).collect();

    let mut tokens: Vec<Token> = Vec::new();

    for string in &split_str {
        match *string {
            "+"     => { tokens.push(Token::new(TokType::ADD, 0, "".to_string())); }
            "-"     => { tokens.push(Token::new(TokType::SUB, 0, "".to_string())); }
            "*"     => { tokens.push(Token::new(TokType::MUL, 0, "".to_string())); }
            "/"     => { tokens.push(Token::new(TokType::DIV, 0, "".to_string())); }
            "%"     => { tokens.push(Token::new(TokType::MOD, 0, "".to_string())); }
            "print" => { tokens.push(Token::new(TokType::PRINT, 0, "".to_string())); }
            "swap"  => { tokens.push(Token::new(TokType::SWAP, 0, "".to_string())); }
            "rot"   => { tokens.push(Token::new(TokType::ROT, 0, "".to_string())); }
            _       => {
                if is_int(*string) {
                    let val: i32 = FromStr::from_str(*string).unwrap();
                    tokens.push(Token::new(TokType::INT, val, "".to_string()));
                }
                else if Some(*string).unwrap().chars().nth(0) == Some('"') {
                    let s = string.to_string();
                    let mut advanced = &s[1..];

                    let loc = advanced.find('"').unwrap();
                    advanced = &advanced[..loc];

                    tokens.push(Token::new(TokType::STR, advanced.len().try_into().unwrap(), advanced.to_string()));
                }
                else {
                    eprintln!("{RED}[ERROR]: Unknown token: {s}{RESET}", s = *string);
                    process::exit(1);
                }
            }
        }
    }
    tokens.push(Token::new(TokType::END, 0, "".to_string()));

    tokens
}

fn is_int(string: &str) -> bool {
    for character in string.chars() {
        if !character.is_digit(10) {
            return false;
        }
    }
    return true;
}

//fn to_str() -> String { return "".to_string(); }
