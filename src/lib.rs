use std::collections::HashMap;

struct Arguments {
    bool_map: HashMap<char, bool>,
}

impl Arguments {
    fn from(_args: Vec<&str>) -> Self {
        Self {
            bool_map: HashMap::new(),
        }
    }

    fn get_bool(&self, _c: char) -> Option<bool> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_bool_from_args() {
        // given
        let args = vec!["app_name", "-b"];

        // when
        let arguments = Arguments::from(args);

        // then
        assert!(arguments.get_bool('b').unwrap());
    }
}
