use regex::Regex;

lazy_static! {
    static ref LINE_REGEX: Regex =
        Regex::new(r"^\s*([a-zA-Z_]+[a-zA-Z_0-9]*)\s*=\s*(.*)").expect("Invalid regex");
    static ref PART_REGEX: Regex =
        Regex::new(r"^\s*([a-zA-Z_]+[a-zA-Z_0-9]*)\s*->\s*(.*)").expect("Invalid regex");
}

pub(super) fn line_metadata(txt: &str) -> (Option<String>, String) {
    if let Some(captures) = LINE_REGEX.captures(txt) {
        (Some(captures[1].to_string()), captures[2].to_string())
    } else {
        (None, txt.to_string())
    }
}

pub(super) fn part_metadata(txt: &str) -> (Option<String>, String) {
    if let Some(captures) = PART_REGEX.captures(txt) {
        (Some(captures[1].to_string()), captures[2].to_string())
    } else {
        (None, txt.to_string())
    }
}
