use crate::schema::argument::ArgumentType;

#[derive(Debug, PartialEq)]
pub enum Token {
    Argument(ArgumentType),
    StrValue(String),
    IntValue(i64),
    BoolValue(bool),
}

pub struct Tokens {
}

impl Tokens {
    fn new() -> Self {
        Self {
        }
    }

    fn size(&self) -> usize {
        0usize
    }
}

pub struct TokenParser {
    args: Vec<String>,
}

impl TokenParser {
    fn new(args: Vec<&str>) -> Self {
        Self {
            args: args.into_iter().map(String::from).collect(),
        }
    }

    fn collect(mut self) -> Tokens {
        Tokens::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_empty_token_collection_when_no_args() {
        // given
        let mut parser = TokenParser::new(Vec::new());

        // when
        let tokens = parser.collect();
        
        // then
        assert_eq!(tokens.size(), 0);
    }

    #[test]
    fn should_ignore_first_argument() {
        // given
        let mut parser = TokenParser::new(vec!["app_name"]);

        // when
        let tokens = parser.collect();
        
        // then
        assert_eq!(tokens.size(), 0);
    }
}
