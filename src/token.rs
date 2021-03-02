pub(super) fn tokenize(txt: &str) -> Vec<String> {
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
