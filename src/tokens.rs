#![allow(unused, non_snake_case)]

use std::process;
use std::str::FromStr;

use crate::utils;
use utils::{BOLD, RED, GREEN, RESET, is_int, DEBUG};

#[derive(Debug)]
pub enum TokType {
    INT,
    STR,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    IPRINT,
    SPRINT,
    IREAD,
    SREAD,
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

fn splitSource(source: &String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut current_token = String::new();
    let mut in_string = false;

    for c in source.chars() {
        if c == '"' {
            in_string = !in_string;
            current_token.push(c);
        } else if (c == ' ' || c == '\n' || c == '\r') && !in_string {
            if !current_token.is_empty() {
                result.push(current_token);
                current_token = String::new();
            }
        } else {
            current_token.push(c);
        }
    }

    if !current_token.is_empty() {
        result.push(current_token);
    }

    result
}

fn process_escape_sequences(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' && chars.peek().is_some() {
            match chars.next().unwrap() {
                'n' => result.push('\n'),
                't' => result.push('\t'),
                'r' => result.push('\r'),
                '\\' => result.push('\\'),
                '"' => result.push('"'),
                '0' => result.push('\0'),
                c => {
                    result.push('\\');
                    result.push(c);
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

pub fn tokenize(source: &String) -> Vec<Token> {
    let split_str: Vec<String> = splitSource(&source);

    let mut tokens: Vec<Token> = Vec::new();

    let mut unfinishedStr: String = String::new();
    let mut isUnfinished: bool = false;

    for string in &split_str {
        match string.as_str() {
            "+"      => { tokens.push(Token::new(TokType::ADD,    0, "".to_string())); }
            "-"      => { tokens.push(Token::new(TokType::SUB,    0, "".to_string())); }
            "*"      => { tokens.push(Token::new(TokType::MUL,    0, "".to_string())); }
            "/"      => { tokens.push(Token::new(TokType::DIV,    0, "".to_string())); }
            "%"      => { tokens.push(Token::new(TokType::MOD,    0, "".to_string())); }
            "iprint" => { tokens.push(Token::new(TokType::IPRINT, 0, "".to_string())); }
            "sprint" => { tokens.push(Token::new(TokType::SPRINT, 0, "".to_string())); }
            "swap"   => { tokens.push(Token::new(TokType::SWAP,   0, "".to_string())); }
            "rot"    => { tokens.push(Token::new(TokType::ROT,    0, "".to_string())); }
            "iread"  => { tokens.push(Token::new(TokType::IREAD,  0, "".to_string())); }
            "sread"  => { tokens.push(Token::new(TokType::SREAD,  0, "".to_string())); }
            _        => {
                if utils::is_int(string) {
                    let val: i32 = FromStr::from_str(string).unwrap();
                    tokens.push(Token::new(TokType::INT, val, "".to_string()));
                }
                else if Some(string).unwrap().chars().nth(0) == Some('"') || isUnfinished {
                    let s = string.to_string();

                    let mut advanced: &str;

                    if !isUnfinished {
                        advanced = &s[1..];
                    } else {
                        advanced = &s;
                    }

                    let loc = advanced.find('"');

                    match loc {
                        Some(location) => {
                            isUnfinished = false;
                            unfinishedStr.push_str(&advanced[..location]);

                            let processed_str = process_escape_sequences(&unfinishedStr);
                            tokens.push(Token::new(TokType::STR, processed_str.len().try_into().unwrap(), processed_str));

                            unfinishedStr.clear();
                        }
                        None => {
                            isUnfinished = true;
                            unfinishedStr.push_str(&advanced);
                            unfinishedStr.push_str(" ");
                            continue;
                        }
                    }
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
