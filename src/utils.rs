#![allow(unused, non_snake_case)]

use std::process::{Command, exit};

pub const BOLD: &str = "\x1b[1m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "1b[33m";
pub const RESET: &str = "\x1b[0m";

pub static mut DEBUG: bool = false;

pub fn is_int(string: &str) -> bool {
    for character in string.chars() {
        if !character.is_digit(10) {
            return false;
        }
    }
    return true;
}

pub fn command(cmd: &str, args: &[&str]) -> bool {
    print!("{GREEN}{BOLD}[CMD]: {cmd} ");
    for arg in args {
        print!("{arg} ");
    }
    println!("{RESET}");

    let output = Command::new(cmd)
        .args(args)
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("{RED}[ERROR] Command failed with exit code: {}{RESET}", 
                          output.status.code().unwrap());
                
                if !output.stderr.is_empty() {
                    eprintln!("{RED}Error output:{RESET}");
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }
                
                return false;
            }
            
            return true;
        },
        Err(e) => {
            eprintln!("{RED}[ERROR] Failed to execute command: {e}{RESET}");
            return false;
        }
    }
}


pub fn log(string: &str) {
    println!("{YELLOW}{string}{RESET}");
}
