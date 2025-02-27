#![allow(unused, non_snake_case)]

use std::process::Command;

pub const BOLD: &str = "\x1b[1m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const RESET: &str = "\x1b[0m";

pub fn is_int(string: &str) -> bool {
    for character in string.chars() {
        if !character.is_digit(10) {
            return false;
        }
    }
    return true;
}

pub fn command(cmd: &str, args: &[&str]) {
    let proc = Command::new(cmd)
        .args(args)
        .spawn();

    print!("{GREEN}{BOLD}{cmd} ");
    for arg in args {
        print!("{arg} ");
    }
    println!("{RESET}");
}
