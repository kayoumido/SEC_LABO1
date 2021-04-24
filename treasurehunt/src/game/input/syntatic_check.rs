/*!
 * Functions to check if the user input is syntactically valid
 * 
 * # Author
 * Doran Kayoumi <doran.kayoumi@heig-vd.ch>
 */

use regex::Regex;

/// Check if a string is syntactically a correct command
/// i.e. it's either multiple letters or digits
/// 
/// # Examples
/// 
/// * Move    - valid
/// * 1       - valid
/// * M0v3    - invalid
/// 
/// # Arguments
/// 
/// * `s` - string to check the syntax
/// 
pub fn check_cmd_syntax(s: &str) -> bool {
    let re: Regex = Regex::new(r"^([A-Za-z]+)$|^(\d+)$").unwrap();

    re.is_match(&s)
}

/// Check if a string is syntactically a colour
/// i.e. either a colour name or a rgb code
/// 
/// # Examples
/// 
/// * Green             - valid
/// * (123, 456, 789)   - valid (it respects the rgb syntax event tho it isn't valid semanticly)
/// * [123, 456, 789]   - valid (it respects the rgb syntax event tho it isn't valid semanticly)
/// * 123, 456, 789     - valid (it respects the rgb syntax event tho it isn't valid semanticly)
/// * Gr33n             - invalid
/// * (abc, def, ghi)   - invalid
/// 
/// # Arguments
/// 
/// * `s` - string to check the syntax
/// 
pub fn check_colour_syntax(s: &str) -> bool {
    let re: Regex = Regex::new(r"^[A-Za-z]+$|^\((\d{1,3},\s*){2}\d{1,3}\)$|^\[(\d{1,3},\s*){2}\d{1,3}\]$|^(\d{1,3},\s*){2}\d{1,3}$").unwrap();

    re.is_match(&s)
}

/// Check if a string is syntactically a coordinate
/// i.e. numbers bertween parentheses separated by commas
/// 
/// # Examples
/// 
/// * (1, 3)        - valid
/// * [5, 9]        - valid
/// * 7, 2          - invalid
/// * (abc, def)    - invalid
/// * hhh           - invalid
/// 
/// # Arguments
/// 
/// * `s` - string to check the syntax
/// 
pub fn check_coord_syntax(s: &str) -> bool {
    let re: Regex = Regex::new(r"^\((\d+,\s*)+\d+\)$|^\[(\d+,\s*)+\d+\]$").unwrap();

    re.is_match(&s)
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest(
        input, 
        expected, 
        case("Command", true), // valid syntax, but invalid semantic
        case("1", true), 
        case("Move", true), 
        case("Start", true), 
        case("C0mm4nd", false) 
        ::trace
    )]
    fn test_check_cmd_syntax(input: &str, expected: bool) {
        assert_eq!(check_cmd_syntax(input), expected);
    }

    #[rstest(
        input,
        expected,
        case("(1, 2)", true),
        case("[5, 6]", true),
        case("(30, 76)", true),
        case("(1, 2, 3)", true), // valid syntax, but invalid semantic
        case("1, 2", false),
        case("(1. 2)", false),
        case("(1 2)", false),
        case("[abc, fgh]", false),
        case("(abc, fgh)", false),
        case("NoTAcOoRd", false),
        case("()", false),
        ::trace
    )]
    fn test_check_coord_syntax(input: &str, expected: bool) {
        assert_eq!(check_coord_syntax(input), expected);
    }

    #[rstest(
        input, 
        expected, 
        case("Green", true), 
        case("Start", true), // valid syntax, but invalid semantic
        case("(123, 456, 789)", true), 
        case("[987, 543, 321]", true),
        case("147, 258, 369", true),
        case("147 258 369", false),
        case("(147 258 369)", false),
        case("[147. 258. 369]", false),
        case("(abc, def, ghi)", false),
        ::trace
    )]
    fn test_check_colour_syntax(input: &str, expected: bool) {
        assert_eq!(check_colour_syntax(input), expected);
    }
}
