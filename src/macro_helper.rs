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
    pub function_signature: String
}

pub fn get_inject_matches(attr_str: &str) -> Vec<InjectMatch> {
    let without_parens = &attr_str[1..attr_str.len() - 1];

    let mut result = vec![];

    for part in without_parens.split("\" ,") {
        let sub_parts: Vec<_> = part.split("=").collect();
        let identifier = sub_parts[0].trim().to_string();
        let function_to_mock = sub_parts[1].trim().replace("\"", "");

        result.push(InjectMatch { identifier: identifier, function_to_mock: function_to_mock });
    }

    result
}

pub fn get_mock_matches(attr_str: &str) -> Vec<MockMatch> {
    let without_parens = &attr_str[1..attr_str.len() - 1];

    let mut result = vec![];

    for part in without_parens.split("\" ,") {
        let sub_parts: Vec<_> = part.split("=").collect();
        let identifier = sub_parts[0].trim().to_string();

        let function_portions: Vec<_> = sub_parts[1].split(":").collect();
        let function_to_mock = function_portions[0].trim().to_string().replace("\"", "");
        let function_signature = function_portions[1].trim().replace("\"", "");

        result.push(MockMatch {
            identifier: identifier,
            function_to_mock: function_to_mock,
            function_signature: function_signature
        });
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn inject_macro_values_should_parse_correctly() {
        let token_string = r#"( id_1 = "db_fake" , id_2 = "other_fake" )"#;

        let inject_matches = get_inject_matches(token_string);
        assert_eq!(inject_matches, vec![
            InjectMatch { identifier: "id_1".to_string(), function_to_mock: "db_fake".to_string() },
            InjectMatch { identifier: "id_2".to_string(), function_to_mock: "other_fake".to_string() },
        ]);
    }

    #[test]
    fn mock_macro_values_should_parse_correctly() {
        let token_string = r#"(
            id_1 = "external_db_call: fn(u32) -> String" , id_2 =
            "other_call: fn() -> String" )"#;

        let mock_matches = get_mock_matches(token_string);
        assert_eq!(mock_matches, vec![
            MockMatch {
                identifier: "id_1".to_string(),
                function_to_mock: "external_db_call".to_string(),
                function_signature: "fn(u32) -> String".to_string()
            },
            MockMatch{
                identifier: "id_2".to_string(),
                function_to_mock: "other_call".to_string(),
                function_signature: "fn() -> String".to_string()
            },
        ]);
    }
}