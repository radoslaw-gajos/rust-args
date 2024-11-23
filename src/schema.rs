use std::collections::HashMap;
use crate::schema::argument::ArgumentType;
use crate::schema::argument::ArgumentTypeFactory;

pub mod argument;

#[derive(Default)]
pub struct Schema {
    map: HashMap<String, String>,
}

impl Schema {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn from(entries: Vec<(String, String)>) -> Self {
        let map = entries
            .into_iter()
            .collect();

        Self {
            map,
        }
    }

    pub fn get(&self, c: char) -> Option<ArgumentType> {
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
