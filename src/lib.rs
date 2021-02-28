use regex::Regex;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref LINE_REGEX: Regex =
        Regex::new(r"^\s*([a-zA-Z_]+[a-zA-Z_0-9]*)\s*=\s*(.*)").expect("Invalid regex");
    static ref PART_REGEX: Regex =
        Regex::new(r"^\s*([a-zA-Z_]+[a-zA-Z_0-9]*)\s*->\s*(.*)").expect("Invalid regex");
}

type State = HashMap<String, String>;

const FINAL_RESULT: &str = "FINAL_RESULT";

#[wasm_bindgen]
pub fn convert(txt: String) -> String {
    let lines = txt.split('\n');

    let mut state = HashMap::new();

    for line in lines {
        state = process_line(line, state);
    }

    state.get(FINAL_RESULT).unwrap().to_string()
}

//    let txt = "x = 42 | f | it -> g(3, it)
// y = 6 | g
// x + y";

fn process_line(txt: &str, state: State) -> State {
    let (line_name, piped_code) = line_metadata(txt.trim());
    let parts = piped_code.split('|');
    let mut code = String::from("");

    for (i, part) in parts.enumerate() {
        let (var_name, this_piped_code) = part_metadata(part);
        let mut this_code = String::from("");
        let tokens = tokenize(this_piped_code.trim());
        let mut found_parentheses = false;

        for token in tokens {
            if var_name.is_some() && var_name.as_ref().unwrap() == &token {
                this_code.push_str(&code);
            } else if let Some(v) = state.get(&token) {
                this_code.push_str(v);
            } else {
                this_code.push_str(&token);
                if token.as_str() == "(" {
                    found_parentheses = true;
                    if var_name.is_none() {
                        this_code.push_str(&code);
                        this_code.push_str(", ");
                    }
                }
            }
        }

        if !found_parentheses && i > 0 {
            this_code.push_str(&format!("({})", &code));
        }

        code = this_code;
    }

    let mut new_state = state;
    new_state.insert(line_name.unwrap_or_else(|| FINAL_RESULT.to_string()), code);
    new_state
}

fn line_metadata(txt: &str) -> (Option<String>, String) {
    if let Some(captures) = LINE_REGEX.captures(txt) {
        (Some(captures[1].to_string()), captures[2].to_string())
    } else {
        (None, txt.to_string())
    }
}

fn part_metadata(txt: &str) -> (Option<String>, String) {
    if let Some(captures) = PART_REGEX.captures(txt) {
        (Some(captures[1].to_string()), captures[2].to_string())
    } else {
        (None, txt.to_string())
    }
}

fn tokenize(txt: &str) -> Vec<String> {
    let mut tokens = vec![];
    if txt.is_empty() {
        return tokens;
    }

    let mut chars = txt.chars();
    let mut previous_char = chars.next().unwrap();
    let mut token = previous_char.to_string();

    for c in chars {
        if should_continue(previous_char, c) {
            token.push_str(&c.to_string());
        } else {
            tokens.push(token);
            token = c.to_string();
        }
        previous_char = c;
    }
    if !token.is_empty() {
        tokens.push(token);
    }

    tokens
}

fn is_variable_char(c: char) -> bool {
    c == '_' || c.is_alphanumeric()
}

fn should_continue(previous_char: char, c: char) -> bool {
    c == previous_char || (is_variable_char(c) && is_variable_char(previous_char))
}
