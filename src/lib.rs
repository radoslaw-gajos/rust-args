#[derive(Debug, PartialEq)]
struct Token {
}

struct TokenParser {
}

impl TokenParser {
    fn new(args: Vec<&str>) -> Self {
        Self {
        }
    }

    fn collect(mut self) -> Vec<Token> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_empty_list_when_no_args() {
        // given
        let mut parser = TokenParser::new(Vec::new());

        // when
        let tokens = parser.collect();
        
        // then
        assert_eq!(tokens, Vec::new());
    }

    #[test]
    fn should_ignore_first_argument() {
        // given
        let mut parser = TokenParser::new(vec!["app_name"]);

        // when
        let tokens = parser.collect();
        
        // then
        assert_eq!(tokens, Vec::new());
    }
}
