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

fn generate_expected_string(expected: &HashSet<&str>) -> String {
    let mut expected: Vec<&str> = expected.iter().copied().collect();

    expected.sort();

    format!("expected {}", or_list(&expected))
}

#[test]
fn test_generate_expected_string() {
    let mut expected = HashSet::new();
    expected.insert("number");
    expected.insert("abc");
    assert_eq!(
        generate_expected_string(&expected),
        "expected abc or number"
    );
}

fn or_list(items: &[&str]) -> String {
    let len = items.len();
    match len {
        0 => String::new(),
        1 => String::from(items[0]),
        _ => {
            let last = len - 1;
            format!("{} or {}", items[0..last].join(", "), items[last])
        }
    }
}

#[test]
fn test_or_list_with_one_entry() {
    assert_eq!(or_list(&vec!["foo"]), String::from("foo"));
}

#[test]
fn test_or_list_with_two_entries() {
    assert_eq!(or_list(&vec!["foo", "bar"]), String::from("foo or bar"));
}

#[test]
fn test_or_list_with_three_entries() {
    assert_eq!(
        or_list(&vec!["foo", "bar", "baz"]),
        String::from("foo, bar or baz")
    );
}

#[test]
fn test_or_list_with_five_entries() {
    assert_eq!(
        or_list(&vec!["a", "b", "c", "d", "e"]),
        String::from("a, b, c, d or e")
    );
}
