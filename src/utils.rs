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
