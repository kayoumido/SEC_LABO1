use regex::Regex;

pub fn check_cmd_syntax(s: &str) -> bool {
    let re: Regex = Regex::new(r"([A-Za-z]+)|(\d+)").unwrap();

    re.is_match(&s)
}

pub fn check_colour_syntax(s: &str) -> bool {
    let re: Regex = Regex::new(r"^[A-Za-z]+$|^\((\d{1,3},\s*){2}\d{1,3}\)$|^\[(\d{1,3},\s*){2}\d{1,3}\]$|^(\d{1,3},\s*){2}\d{1,3}$").unwrap();

    re.is_match(&s)
}

pub fn check_coord_syntax(s: &str) -> bool {
    let re: Regex = Regex::new(r"^\((\d+,\s*)+\d+\)$|^\[(\d+,\s*)+\d+\]$").unwrap();

    re.is_match(&s)
}
