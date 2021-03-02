use super::metadata::{line_metadata, part_metadata};
use super::sanitize::{sanitize_input, sanitize_output};
use super::token::tokenize;
use std::collections::HashMap;

type State = HashMap<String, String>;

const FINAL_RESULT: &str = "FINAL_RESULT";

pub fn convert(txt: &str) -> String {
    let txt = sanitize_input(txt);
    let lines = txt.split('\n');

    let mut state = HashMap::new();

    for line in lines {
        let l = line.trim();
        if !l.is_empty() {
            state = process_line(l, state);
        }
    }

    let output = state.get(FINAL_RESULT).unwrap();

    sanitize_output(output)
}

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

#[cfg(test)]
mod tests {
    use super::convert;

    #[test]
    fn test_convert() {
        let cases = vec![
            ("42 | f", "f(42)"),
            ("42 | g(3)", "g(42, 3)"),
            ("42 | f | g(3)", "g(f(42), 3)"),
            ("42 | f | x -> g(3, x)", "g(3, f(42))"),
            ("42 | f | foo -> g(3, foo, x)", "g(3, f(42), x)"),
            (
                r#"42 | f | x -> g(x \|\| z(6 == 3))"#,
                "g(f(42) || z(6 == 3))",
            ),
            (
                r#"x = 42 | f
y = 6 | g
x \= y"#,
                "f(42) = g(6)",
            ),
            (r#"42 | g(f->5)"#, "g(42, f->5)"),
            (r#"42 | x \-> f(3)"#, "x -> f(42, 3)"),
            (
                "42 
            | f
            | g(3)",
                "g(f(42), 3)",
            ),
            (
                "x = 42 
            | f
            | g(3)

y = 53 | h


x + y",
                "g(f(42), 3) + h(53)",
            ),
        ];

        for (input, output) in cases {
            assert_eq!(convert(input), output)
        }
    }
}
