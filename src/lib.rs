use std::collections::HashMap;

enum ArgType {
    Boolean,
}

#[derive(Debug, PartialEq)]
enum Value {
    Boolean(bool),
}

struct Arguments;

impl Arguments {
    fn new(config: HashMap<char, ArgType>) -> Self {
        Arguments {
        }
    }

    fn with(self, args: Vec<String>) -> Self {
        self
    }

    fn build(self) -> HashMap<char, Value> {
        HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_boolean_argument() {
        // given
        let args = ["app_name".to_string(), "-b".to_string()];
        let config = HashMap::from([
            ('b', ArgType::Boolean),
        ]);

        // when
        let arguments = Arguments::new(config)
            .with(Vec::from(args))
            .build();

        // then
        assert_eq!(arguments, HashMap::from(
            [('b', Value::Boolean(true))],
        ));
    }
}
