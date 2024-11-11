use std::collections::HashMap;

enum ArgType {
    Boolean,
}

enum Value {
    Boolean,
}

struct Arguments;

impl Arguments {
    fn new(config: HashMap<char, ArgType>) -> Self {
        todo!();
    }

    fn with(self, args: Vec<String>) -> Self {
        todo!();
    }

    fn build(self) -> HashMap<char, Value> {
        todo!();
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
    }
}
