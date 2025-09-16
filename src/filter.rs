use std::collections::VecDeque;

#[derive(Debug)]
pub enum Token {
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

impl crate::Media {
    /// Returns true if the file passes specified conditions
    pub fn matches_filter(&self, fltr: &Vec<crate::filter::Token>) -> bool {
        use crate::filter::*;

        let mut stack: Vec<bool> = Vec::new();
        for element in fltr {
            match element {
                Token::Atom(content) => {
                    let mut matches: bool = false;
                    if self.supports_tags() {
                        for tag in self.tags().expect("has_tags() seems to have returned BS") {
                            if tag.matches(content) {
                                matches = true;
                                break;
                            }
                        }
                    }

                    stack.push(matches);
                }
                Token::Or => {
                    let right = stack.pop().expect("faulty filter.");
                    let left = stack.pop().expect("faulty filter.");
                    stack.push(left || right);
                }
                Token::Xor => {
                    let right = stack.pop().expect("faulty filter.");
                    let left = stack.pop().expect("faulty filter.");
                    stack.push(left ^ right);
                }
                Token::And => {
                    let right = stack.pop().expect("faulty filter.");
                    let left = stack.pop().expect("faulty filter.");
                    stack.push(left && right);
                }
                Token::Xnor => {
                    let right = stack.pop().expect("faulty filter.");
                    let left = stack.pop().expect("faulty filter.");
                    stack.push(!(left ^ right));
                }
                Token::Nand => {
                    let right = stack.pop().expect("faulty filter.");
                    let left = stack.pop().expect("faulty filter.");
                    stack.push(!(left && right));
                }
                Token::Not => {
                    let left = stack.pop().expect("faulty filter.");
                    stack.push(!left);
                }
                Token::GroupOpen => {}
                Token::GroupClose => {}
            }
        }

        // check if the evaluation went cleanly
        assert!(stack.len() == 1);

        stack.pop().unwrap()
    }
}

pub fn parse(filter: String) -> Option<Vec<Token>> {
    let mut tokens: VecDeque<Token> =
        regex::Regex::new(r#"([\(\)!])|(?:(".*")?("[^"]+")|([^()"!\s]+))"#)
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
