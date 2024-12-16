use crate::token::parser::TokenParser;
use crate::token::tokens::Tokens;
use crate::token::Token;
use crate::token::Token::{AppName, Argument, StrValue, IntValue};
use crate::schema::argument::ArgumentType::{self, Bool, Int, Str};
use crate::schema::Schema;
use std::collections::HashMap;

#[derive(Default)]
pub struct Collection {
    schema: Schema,
    ints: HashMap<String,i64>,
    strings: HashMap<String,String>,
    bools: HashMap<String,bool>,
}

impl Collection {
    fn from(mut tokens: Tokens) -> Collection {
        let mut collection = Collection::default();
        collection.schema = (*tokens.schema().expect("Schema expected")).clone();

        let mut current;
        loop {
            current = tokens.current();

            if current.is_none() {
                break;
            }

            match (*current.unwrap()).clone() {
                AppName => (),
                Argument(arg_type, name) => {
                    match arg_type {
                        Bool => {
                            collection.bools.insert(name.to_string(), true);
                            ()
                        },
                        Int => {
                            tokens.next();
                            current = tokens.current();
                            if current.is_none() {
                                panic!("Unexpected end of tokens. Expected Integer");
                            }
                            let int_val = match current.unwrap() {
                                IntValue(val) => val,
                                AppName => panic!("Unexpected AppName Token! Integer expected."),
                                StrValue(val) => panic!("Unexpected String Token: {val}! Integer expected."),
                                Argument(_,name) => panic!("Unexpected Argument Token: {name}! Integer expected."),
                            };
                            collection.ints.insert(name.to_string(), *int_val);
                            ()
                        },
                        Str => {
                            tokens.next();
                            current = tokens.current();
                            if current.is_none() {
                                panic!("Unexpected end of tokens. Expected String");
                            }
                            let str_val = match current.unwrap() {
                                StrValue(val) => val,
                                AppName => panic!("Unexpected AppName Token! String expected."),
                                IntValue(val) => panic!("Unexpected Int Token: {val}! String expected."),
                                Argument(_,name) => panic!("Unexpected Argument Token: {name}! String expected."),
                            };
                            collection.strings.insert(name.to_string(), str_val.to_string());
                            ()
                        },
                    }
                },
                StrValue(val) => panic!("Unexpected String Token: {val}"),
                IntValue(val) => panic!("Unexpected Int Token: {val}"),
            }

            tokens.next();
        }

        collection
    }

    pub fn from_args(args: Vec<&str>, schema: Schema) -> Self {
        let parser = TokenParser::new()
            .args(args)
            .schema(schema);
        let tokens = parser.collect();

        Self::from(tokens)
    }

    pub fn get_int(&self, key: &str) -> Option<i64> {
        if self.ints.contains_key(key) {
            return self.ints.get(key).copied();
        }
        if argument_type_matches(key, &self.schema, ArgumentType::Int) {
            return None;
        }
        panic!("Key not found in schema!");
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        if self.strings.contains_key(key) {
            return self.strings.get(key).map(|x| x.as_str());
        }
        if argument_type_matches(key, &self.schema, ArgumentType::Str) {
            return None;
        }
        panic!("Key not found in schema!");
    }

    pub fn get_bool(&self, key: &str) -> bool {
        if self.bools.contains_key(key) {
            return *self.bools.get(key).unwrap();
        }
        if argument_type_matches(key, &self.schema, ArgumentType::Bool) {
            return false;
        }
        panic!("Key not found in schema!");
    }
}

fn argument_type_matches(key: &str, schema: &Schema, arg_type: ArgumentType) -> bool {
    if let Some(valid_key) = schema.get(key.chars().nth(0).expect("Valid string expected")) {
        if valid_key == arg_type {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_values_from_collection() {
        // given
        let schema = Schema::from(vec![
            ("i".to_string(), "int".to_string()),
            ("s".to_string(), "string".to_string()),
            ("b".to_string(), "bool".to_string()),
        ]);
        let parser = TokenParser::new()
            .args(vec!["app_name", "-i", "42", "-b", "-s", "string"])
            .schema(schema);
        let tokens = parser.collect();

        // when
        let collection = Collection::from(tokens);

        // then
        assert_eq!(collection.get_int("i"), Some(42));
        assert_eq!(collection.get_str("s"), Some("string"));
        assert!(collection.get_bool("b"));
    }

    #[test]
    fn should_get_false_when_flag_not_set() {
        // given
        let schema = Schema::from(vec![
            ("b".to_string(), "bool".to_string()),
        ]);
        let parser = TokenParser::new()
            .args(vec!["app_name"])
            .schema(schema);
        let tokens = parser.collect();

        // when
        let collection = Collection::from(tokens);

        // then
        assert!(!collection.get_bool("b"));
    }

    #[test]
    fn should_get_true_when_flag() {
        // given
        let schema = Schema::from(vec![
            ("b".to_string(), "bool".to_string()),
        ]);
        let parser = TokenParser::new()
            .args(vec!["app_name", "-b"])
            .schema(schema);
        let tokens = parser.collect();

        // when
        let collection = Collection::from(tokens);

        // then
        assert!(collection.get_bool("b"));
    }

    #[test]
    fn should_return_none_when_string_not_set() {
        // given
        let schema = Schema::from(vec![
            ("s".to_string(), "string".to_string()),
        ]);
        let parser = TokenParser::new()
            .args(vec!["app_name"])
            .schema(schema);
        let tokens = parser.collect();

        // when
        let collection = Collection::from(tokens);

        // then
        assert_eq!(collection.get_str("s"), None);
    }

    #[test]
    fn should_return_string_value() {
        // given
        let schema = Schema::from(vec![
            ("s".to_string(), "string".to_string()),
        ]);
        let parser = TokenParser::new()
            .args(vec!["app_name", "-s", "val"])
            .schema(schema);
        let tokens = parser.collect();

        // when
        let collection = Collection::from(tokens);

        // then
        assert_eq!(collection.get_str("s"), Some("val"));
    }

    #[test]
    fn should_return_none_when_int_not_set() {
        // given
        let schema = Schema::from(vec![
            ("i".to_string(), "int".to_string()),
        ]);
        let parser = TokenParser::new()
            .args(vec!["app_name"])
            .schema(schema);
        let tokens = parser.collect();

        // when
        let collection = Collection::from(tokens);

        // then
        assert_eq!(collection.get_int("i"), None);
    }

    #[test]
    fn should_return_int() {
        // given
        let schema = Schema::from(vec![
            ("i".to_string(), "int".to_string()),
        ]);
        let parser = TokenParser::new()
            .args(vec!["app_name", "-i", "-42"])
            .schema(schema);
        let tokens = parser.collect();

        // when
        let collection = Collection::from(tokens);

        // then
        assert_eq!(collection.get_int("i"), Some(-42));
    }

    #[test]
    #[should_panic]
    fn should_panic_if_get_str_instead_of_get_int() {
        // given
        let schema = Schema::from(vec![
            ("i".to_string(), "int".to_string()),
        ]);
        let parser = TokenParser::new()
            .args(vec!["app_name"])
            .schema(schema);
        let tokens = parser.collect();

        // when
        let collection = Collection::from(tokens);

        // then
        collection.get_str("i");
    }

    #[test]
    #[should_panic]
    fn should_panic_if_get_int_instead_of_get_str() {
        // given
        let schema = Schema::from(vec![
            ("s".to_string(), "string".to_string()),
        ]);
        let parser = TokenParser::new()
            .args(vec!["app_name"])
            .schema(schema);
        let tokens = parser.collect();

        // when
        let collection = Collection::from(tokens);

        // then
        collection.get_int("s");
    }

    #[test]
    #[should_panic]
    fn should_panic_if_get_bool_instead_of_get_str() {
        // given
        let schema = Schema::from(vec![
            ("s".to_string(), "string".to_string()),
        ]);
        let parser = TokenParser::new()
            .args(vec!["app_name"])
            .schema(schema);
        let tokens = parser.collect();

        // when
        let collection = Collection::from(tokens);

        // then
        collection.get_bool("s");
    }
}
