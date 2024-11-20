use crate::schema::ArgumentType;

#[derive(Debug, PartialEq)]
pub struct Token {
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
}

impl TokenParser {
    fn new(args: Vec<&str>) -> Self {
        Self {
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
