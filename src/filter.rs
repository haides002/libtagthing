use std::collections::VecDeque;

#[derive(Debug)]
enum Token {
    Atom(String),
    Not,
    And,
    Or,
    Nand,
    Xor,
    Xnor,
    GroupOpen,
    GroupClose,
}

impl Token {
    /// Return Some(Self) if the token is a valid operation, otherwise None
    pub fn into(token: &str) -> Self {
        match token.to_uppercase().as_str() {
            "AND" | "&" => Self::And,
            "OR" | "|" => Self::Or,
            "NOT" | "!" => Self::Not,
            "XOR" => Self::Xor,
            "NAND" => Self::Nand,
            "XNOR" => Self::Xnor,
            "(" => Self::GroupOpen,
            ")" => Self::GroupClose,
            &_ => Self::Atom(token.to_string()),
        }
    }
}

fn parse(filter: String) -> Option<Vec<Token>> {
    let mut tokens: VecDeque<Token> =
        regex::Regex::new(r#"([\(\)!])|(?:(".*")?("[\w ]+")|([^()"!\s]+))"#)
            .unwrap()
            .captures_iter(&filter)
            .map(|capture| Token::into(capture.iter().next().unwrap().unwrap().as_str()))
            .collect();

    let mut stack: Vec<Token> = Vec::new();
    let mut out: Vec<Token> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop_front().unwrap();
        match token {
            Token::Atom(_) => {
                out.push(token);
            }
            Token::Not => {
                out.push(tokens.pop_front()?);
                out.push(token);
            }
            Token::GroupClose => {
                while let Some(operator) = stack.pop() {
                    match operator {
                        Token::GroupOpen => break,
                        _ => out.push(operator),
                    }
                }
            }
            _ => stack.push(token),
        }
    }

    while let Some(operator) = stack.pop() {
        out.push(operator);
    }

    Some(out)
}

#[cfg(test)]
mod filter_tests {
    use crate::filter::parse;

    #[test]
    fn test_parse() {
        dbg!(parse("car and (cat or tree)".to_string()));
    }
}
