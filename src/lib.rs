#[derive(Debug, PartialEq)]
pub enum ArgumentType {
    Bool,
}

mod schema {
    use crate::ArgumentType;
    use std::collections::HashMap;

    pub struct Schema {
        map: HashMap<String, String>,
    }

    impl Schema {
        fn new() -> Self {
            Self {
                map: HashMap::new(),
            }
        }

        fn from(entries: Vec<(String, String)>) -> Self {
            let map = entries
                .into_iter()
                .collect();

            Self {
                map,
            }
        }

        fn get(&self, c: char) -> Option<ArgumentType> {
            let key = c.to_string();

            if let Some(t) = self.map.get(&key) {
                match t.as_str() {
                    "bool" => Some(ArgumentType::Bool),
                    &_ => panic!("invalid argument!"),
                }
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_return_none_when_not_defined() {
            // given
            let schema = Schema::new();

            // when
            let arg = schema.get('b');

            // then
            assert_eq!(arg, None);
        }

        #[test]
        fn should_return_bool_argument() {
            // given
            let schema = Schema::from(vec![
                ("b".to_string(), "bool".to_string()),
            ]);

            // when
            let arg = schema.get('b');

            // then
            assert!(arg.is_some());
            assert_eq!(arg, Some(ArgumentType::Bool));
        }
    }
}

mod token {
    use crate::ArgumentType;

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
}
