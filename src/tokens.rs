use std::process;
use std::str::FromStr;

use crate::utils;
use utils::{BOLD, RED, GREEN, RESET, is_int};

#[derive(Debug)]
pub enum TokType {
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
pub struct Token {
    pub typ: TokType,
    pub val: i32,
    pub data: String,
}

impl Token {
    pub fn new(typ: TokType, val: i32, data: String) -> Self {
        Self {
            typ,
            val,
            data,
        }
    }
}

pub fn tokenize(source: String) -> Vec<Token> {
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
                if utils::is_int(*string) {
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
