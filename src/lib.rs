use std::collections::HashMap;

enum ArgType {
    Boolean,
}

#[derive(Debug, PartialEq)]
enum Value {
    Boolean(bool),
}

struct Argument {
}

impl Argument {
    fn from(string: &str) {

    }
}

struct Arguments {
    config: HashMap<char, ArgType>,
    args: Vec<String>,
}

impl Arguments {
    fn new(config: HashMap<char, ArgType>) -> Self {
        Arguments {
            config,
            args: Vec::new(),
        }
    }

    fn with(self, args: Vec<String>) -> Self {
        Self {
            args,
            ..self
        }
    }

    fn build(&self) -> HashMap<char, Value> {
        let map = HashMap::new();

        for a in &self.args {
            let arg = 
        }

        map
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
