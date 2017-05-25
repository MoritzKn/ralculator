use parser::ParseError;
use std::collections::HashSet;
use text_range::TextRange;

pub struct InputError {
    /// The error message
    pub msg: String,
    /// The columns in which the error occurred
    pub pos: TextRange,
}

impl InputError {
    pub fn from_parser_error(err: ParseError) -> InputError {
        InputError {
            msg: generate_expected_string(&err.expected),
            pos: TextRange {
                start: err.offset,
                end: err.column,
            },
        }
    }
}

fn generate_expected_string(expected: &HashSet<&'static str>) -> String {
    let expected = expected
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    format!("expected {}", or_list(&expected))
}


fn or_list(items: &[String]) -> String {
    let len = items.len();
    match len {
        0 => String::new(),
        1 => items[0].clone(),
        _ => {
            let last = len - 1;
            format!("{} or {}", items[0..last].join(", "), items[last])
        }
    }
}


#[test]
fn test_or_list() {
    assert_eq!(or_list(&vec![String::from("foo")]), String::from("foo"));

    assert_eq!(
        or_list(&vec![String::from("foo"), String::from("bar")]),
        String::from("foo or bar")
    );

    assert_eq!(
        or_list(
            &vec![
                String::from("foo"),
                String::from("bar"),
                String::from("baz"),
            ]
        ),
        String::from("foo, bar or baz")
    );

    assert_eq!(
        or_list(
            &vec![
                String::from("a"),
                String::from("b"),
                String::from("c"),
                String::from("d"),
                String::from("e"),
            ]
        ),
        String::from("a, b, c, d or e")
    );
}
