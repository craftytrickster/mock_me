use proc_macro::TokenStream;

pub const HEADER: &'static str = r#"
    extern crate mock_me_test_context;
    let _mock_me_test_context_instance = mock_me_test_context::get_test_context();
"#;

#[derive(Debug)]
pub struct InjectMatch {
    pub identifier: String,
    pub function_to_mock: String,
}

#[derive(Debug)]
pub struct MockMatch {
    pub identifier: String,
    pub function_to_mock: String,
    pub function_signature: String
}

pub fn get_inject_matches(attr: TokenStream) -> Vec<InjectMatch> {
    let attr_str = attr.to_string();
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

pub fn get_mock_matches(attr: TokenStream) -> Vec<MockMatch> {
    let attr_str = attr.to_string();
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