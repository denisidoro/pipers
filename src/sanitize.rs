use regex::Regex;

lazy_static! {
    static ref NEWLINE_REGEX: Regex = Regex::new(r"[ \t]*\n+[ \t]+").expect("Invalid regex");
}

const PIPE_ESCAPE_CHAR: char = '\u{02A3}';
const EQUAL_ESCAPE_CHAR: char = '\u{02A4}';
const ARROW_ESCAPE_CHAR: char = '\u{02A5}';

pub(super) fn sanitize_input(txt: &str) -> String {
    let replaced = txt
        .replace("\\|", &PIPE_ESCAPE_CHAR.to_string())
        .replace("\\=", &EQUAL_ESCAPE_CHAR.to_string())
        .replace("\\->", &ARROW_ESCAPE_CHAR.to_string());

    NEWLINE_REGEX.replace_all(&replaced, " ").into()
}

pub(super) fn sanitize_output(txt: &str) -> String {
    txt.replace(&PIPE_ESCAPE_CHAR.to_string(), "|")
        .replace(&EQUAL_ESCAPE_CHAR.to_string(), "=")
        .replace(&ARROW_ESCAPE_CHAR.to_string(), "->")
}
