use text_range::TextRange;
use parser::ParseError;

pub struct InputError {
    /// The error message
    pub msg: String,
    /// The columns in which the error occurred
    pub pos: TextRange,
}

impl InputError {
    pub fn from_parser_error(err: ParseError) -> InputError {
        InputError {
            // TODO: Formt message
            msg: String::from(""),
            pos: TextRange {
                start: err.offset,
                end: err.column,
            },
        }
    }
}
