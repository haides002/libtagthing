#[derive(Debug)]
/// Token enum for evaluating filters
pub enum Token {
    /// Tags and not recognized operators
    Atom(regex::Regex),
    /// Not operation
    Not,
    /// And operation
    And,
    /// Or operation
    Or,
    /// Nand operation
    Nand,
    /// Xor operation
    Xor,
    /// Xnor operation
    Xnor,
    /// (
    GroupOpen,
    /// )
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
            &_ => {
                let criteria_wildcard_support = token.replace("*", ".*");
                let criteria_regex =
                    regex::Regex::new(&format!(r"^((?:{})(?:/.*)?)$", criteria_wildcard_support))
                        .unwrap();

                Self::Atom(criteria_regex)
            }
        }
    }
}

impl crate::Media {
    /// Returns true if the file passes specified conditions
    pub fn matches_filter(&self, fltr: &Vec<crate::filter::Token>) -> bool {
        use crate::filter::*;

        if fltr.is_empty() {
            return true;
        }

        let mut stack: Vec<bool> = Vec::new();
        for element in fltr {
            match element {
                Token::Atom(content) => {
                    let mut matches: bool = false;
                    if self.supports_xmp() {
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
                    let right = stack.pop().expect("faulty filter.");
                    stack.push(!right);
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

/// Parses an input string into a vec of tokens
///
/// Returns None if the filter couldn't be parsed
pub fn parse(filter: String) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> =
        regex::Regex::new(r#"([\(\)!])|(?:(".*")?("[^"]+")|([^()"!\s]+))"#)
            .unwrap()
            .captures_iter(&filter)
            .map(|capture| {
                Token::into(
                    capture
                        .iter()
                        .next()
                        .unwrap()
                        .unwrap()
                        .as_str()
                        .replace("\"", "")
                        .as_str(),
                )
            })
            .collect();

    // insert implicit ANDs
    let mut i: isize = 0;
    while i < tokens.len() as isize - 1 {
        assert!(i >= 0);
        match tokens[i as usize] {
            Token::Atom(_) | Token::GroupClose => match tokens[(i + 1) as usize] {
                Token::Atom(_) | Token::Not | Token::GroupOpen => {
                    tokens.insert((i + 1) as usize, Token::And);
                }
                _ => {}
            },
            _ => {}
        }
        i += 1;
    }

    // validate the filter
    // step 1: check token order
    let mut i: usize = 0;
    while i < tokens.len().saturating_sub(1) {
        use Token::*;
        match (&tokens[i], &tokens[i + 1]) {
            (Not, GroupOpen | Atom(_)) => {}
            (And | Or | Xor | Nand | Xnor, Not | Atom(_) | GroupOpen) => {}
            (GroupOpen, Not | GroupOpen | Atom(_)) => {}
            (GroupClose, GroupClose | And | Or | Xor | Nand | Xnor) => {}
            (Atom(_), And | Or | Xor | Nand | Xnor | GroupClose) => {}
            _ => return None,
        };
        i += 1;
    }

    // step 2: check beninigging and end
    match tokens.first() {
        Some(Token::GroupOpen | Token::Atom(_) | Token::Not) => {}
        None => {}
        _ => return None,
    }
    match tokens.last() {
        Some(Token::GroupClose | Token::Atom(_)) => {}
        None => {}
        _ => return None,
    }

    // step 3: validate the parentheses
    let mut p: isize = 0;
    for token in &tokens {
        match token {
            Token::GroupOpen => p += 1,
            Token::GroupClose => {
                p -= 1;
                if p < 0 {
                    return None;
                }
            }
            _ => {}
        }
    }
    if p != 0 {
        return None;
    }

    // doing this is fine here because the vec is not that large and we only do this once
    tokens.reverse();

    let mut operator_stack: Vec<Token> = Vec::new();
    let mut output_stack: Vec<Token> = Vec::new();
    for token in tokens {
        match token {
            Token::Atom(_) => {
                output_stack.push(token);
            }
            Token::Not => {
                output_stack.push(token);
            }
            Token::GroupOpen => {
                while let Some(operator) = operator_stack.pop() {
                    match operator {
                        Token::GroupClose => break,
                        _ => output_stack.push(operator),
                    }
                }
            }
            _ => {
                operator_stack.push(token);
            }
        }
    }

    while let Some(operator) = operator_stack.pop() {
        output_stack.push(operator);
    }

    Some(output_stack)
}

/// Applies a filter given as a `String` to the given `Vec`. Returns `None` if given an invalid
/// filter.
/// This not clones all matching media instances, but performance should be fine as the Media struct
/// does not contain much data.
pub fn apply_filter(medias: &[crate::Media], query: String) -> Option<Vec<&crate::Media>> {
    let fltr: Vec<Token> = crate::filter::parse(query)?;

    Some(
        medias
            .iter()
            .filter(|media| -> bool { media.matches_filter(&fltr) })
            .collect::<Vec<&crate::Media>>(),
    )
}

pub fn get_matching_indicies(medias: &[crate::Media], query: String) -> Option<Vec<usize>> {
    let fltr: Vec<Token> = crate::filter::parse(query)?;
    Some(
        medias
            .iter()
            .enumerate()
            .filter_map(|(index, media)| -> Option<usize> {
                if media.matches_filter(&fltr) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect(),
    )
}

#[cfg(test)]
mod filter_tests {
    use crate::filter::parse;

    #[test]
    fn test_parse() {
        dbg!(parse("car and (cat or tree)".to_string()));
    }
}
