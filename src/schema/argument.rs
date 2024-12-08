#[derive(Debug, PartialEq, Clone)]
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

