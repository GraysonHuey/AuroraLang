#![allow(unused, non_snake_case)]

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
    IPRINT,
    SPRINT,
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

    let mut characters: Vec<char> = source.chars().collect();

    for character in &characters.clone() {
        if *character == ' ' {
            result.push(" ".to_string());
        } else {
            let string: String = characters.iter().collect();
            let idx: usize = string.find(' ').unwrap_or_else(|| usize::MAX );
            if idx == usize::MAX {
                let pushStr = characters.iter().collect();
                result.push(pushStr);
                break;
            }

            let pushStr = characters[..idx].iter().collect();
            characters = characters.split_off(idx+1);
            result.push(pushStr);
        }
    }

    println!("{source}");
    println!("{result:?}");
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
                        }
                        None => {
                            isUnfinished = true;
                            unfinishedStr.push_str(&advanced);
                            println!("UNFINISHEDSTR: {0}", unfinishedStr);
                            continue;
                        }
                    }

                    advanced = unfinishedStr.as_str();

                    //advanced = &advanced[..loc];

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
