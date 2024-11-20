use std::collections::HashMap;
use crate::schema::argument::ArgumentType;
use crate::schema::argument::ArgumentTypeFactory;

pub mod argument {
    #[derive(Debug, PartialEq)]
    pub enum ArgumentType {
        Bool,
        Str,
        Int,
    }

    impl ArgumentType {
        fn from(string: &str) -> Option<Self> {
            match string {
                "bool" => Some(ArgumentType::Bool),
                "string" => Some(ArgumentType::Str),
                "int" => Some(ArgumentType::Int),
                &_ => panic!("invalid argument!"),
            }
        }
    }

    pub trait ArgumentTypeFactory {
        fn arg_type(self: Self) -> Option<ArgumentType>;
    }

    impl ArgumentTypeFactory for &str {
        fn arg_type(self: Self) -> Option<ArgumentType> {
            ArgumentType::from(self)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_get_type_from_string() {
            assert_eq!(ArgumentType::from("bool"), Some(ArgumentType::Bool));
            assert_eq!(ArgumentType::from("string"), Some(ArgumentType::Str));
            assert_eq!(ArgumentType::from("int"), Some(ArgumentType::Int));
        }
    }
}

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

        self.map.get(&key)?.arg_type()
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

    #[test]
    fn should_return_string_argument() {
        // given
        let schema = Schema::from(vec![
            ("s".to_string(), "string".to_string()),
        ]);

        // when
        let arg = schema.get('s');

        // then
        assert!(arg.is_some());
        assert_eq!(arg, Some(ArgumentType::Str));
    }

    #[test]
    fn should_return_integer_argument() {
        // given
        let schema = Schema::from(vec![
            ("i".to_string(), "int".to_string()),
        ]);

        // when
        let arg = schema.get('i');

        // then
        assert!(arg.is_some());
        assert_eq!(arg, Some(ArgumentType::Int));
    }

    #[test]
    fn should_return_arguments() {
        // given
        let schema = Schema::from(vec![
            ("i".to_string(), "int".to_string()),
            ("s".to_string(), "string".to_string()),
            ("b".to_string(), "bool".to_string()),
        ]);

        // then
        assert_eq!(schema.get('i'), Some(ArgumentType::Int));
        assert_eq!(schema.get('s'), Some(ArgumentType::Str));
        assert_eq!(schema.get('b'), Some(ArgumentType::Bool));
    }
}
