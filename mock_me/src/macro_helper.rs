pub const HEADER: &'static str = r#"
    extern crate mock_me_test_context;
    let _mock_me_test_context_instance = mock_me_test_context::get_test_context();
"#;

#[derive(Debug, PartialEq)]
pub struct InjectMatch {
    pub identifier: String,
    pub function_to_mock: String,
}

#[derive(Debug, PartialEq)]
pub struct MockMatch {
    pub identifier: String,
    pub function_to_mock: String,
    pub function_signature: String,
}

pub fn get_inject_matches(attr_str: &str) -> Vec<InjectMatch> {
    let p = Parser::new(attr_str);
    p.get_inject_matches()
}

pub fn get_mock_matches(attr_str: &str) -> Vec<MockMatch> {
    let p = Parser::new(attr_str);
    p.get_mock_matches()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn inject_macro_values_should_parse_correctly() {
        let token_string = r#"id_1 = "db_fake" , id_2 = "other_fake""#;

        let inject_matches = get_inject_matches(token_string);
        assert_eq!(
            inject_matches,
            vec![
                InjectMatch {
                    identifier: "id_1".to_string(),
                    function_to_mock: "db_fake".to_string()
                },
                InjectMatch {
                    identifier: "id_2".to_string(),
                    function_to_mock: "other_fake".to_string()
                },
            ]
        );
    }

    #[test]
    fn mock_macro_values_should_parse_correctly() {
        let token_string = r#"
            id_1 = "external_db_call: fn(u32) -> String" , id_2 =
            "other_call: fn() -> String" "#;

        let mock_matches = get_mock_matches(token_string);
        assert_eq!(
            mock_matches,
            vec![
                MockMatch {
                    identifier: "id_1".to_string(),
                    function_to_mock: "external_db_call".to_string(),
                    function_signature: "fn(u32) -> String".to_string()
                },
                MockMatch {
                    identifier: "id_2".to_string(),
                    function_to_mock: "other_call".to_string(),
                    function_signature: "fn() -> String".to_string()
                }
            ]
        );
    }
}

// inspired in https://limpet.net/mbrubeck/2014/08/11/toy-layout-engine-2.html
struct Parser<'a> {
    pos: usize,
    input: &'a str,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser {
            pos: 0,
            input: input,
        }
    }

    fn get_inject_matches(mut self) -> Vec<InjectMatch> {
        self.execute(Parser::consume_inject_match)
    }

    fn get_mock_matches(mut self) -> Vec<MockMatch> {
        self.execute(Parser::consume_mock_match)
    }

    fn execute<F, R>(&mut self, mut consume_function: F) -> Vec<R>
    where
        F: FnMut(&mut Self) -> R,
    {
        let mut result = Vec::new();

        loop {
            self.consume_whitespace();

            if self.eof() {
                break;
            }

            let item = consume_function(self);
            result.push(item);

            self.consume_whitespace();

            if self.eof() || !self.consume_has_separator() {
                break;
            }
        }

        self.consume_whitespace();
        result
    }

    fn consume_has_separator(&mut self) -> bool {
        let mut has_next = false;
        if self.next_char() == ',' {
            has_next = true;
            self.consume_char();
        }

        has_next
    }

    fn consume_inject_match(&mut self) -> InjectMatch {
        let identifier = self.parse_text();

        self.consume_whitespace();
        assert_eq!(self.consume_char(), '=');
        self.consume_whitespace();

        assert_eq!(self.consume_char(), '"');
        let function_to_mock = self.parse_text();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), '"');

        InjectMatch {
            identifier: identifier,
            function_to_mock: function_to_mock,
        }
    }

    fn consume_mock_match(&mut self) -> MockMatch {
        let identifier = self.parse_text();

        self.consume_whitespace();
        assert_eq!(self.consume_char(), '=');
        self.consume_whitespace();

        assert_eq!(self.consume_char(), '"');
        let function_to_mock = self.parse_text();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), ':');
        self.consume_whitespace();
        let function_signature = self.consume_while(|c| c != '"');
        assert_eq!(self.consume_char(), '"');

        MockMatch {
            identifier: identifier,
            function_to_mock: function_to_mock,
            function_signature: function_signature,
        }
    }

    fn next_char(&self) -> char {
        self.input.as_bytes()[self.pos] as char
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let cur_char = self.input.as_bytes()[self.pos];
        self.pos += 1;
        cur_char as char
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();

        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }

        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn parse_text(&mut self) -> String {
        self.consume_while(|c| c.is_alphanumeric() || c == '_')
    }
}
