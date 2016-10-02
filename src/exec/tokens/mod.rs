mod token_types;

use self::token_types::{TokenTypeChecker, NUMBER_TOKEN_CHECKER, SPACE_TOKEN_CHECKER,
                        OPERATOR_TOKEN_CHECKER};

pub use self::token_types::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub pos: usize,
    pub token_type: TokenType,
    pub content: String,
}

static TOKEN_TYPE_CHECKERS: [TokenTypeChecker; 3] =
    [NUMBER_TOKEN_CHECKER, OPERATOR_TOKEN_CHECKER, SPACE_TOKEN_CHECKER];

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut curr_token_content = String::new();
    let mut curr_token_type = TokenType::None;
    let mut curr_token_type_checker = &SPACE_TOKEN_CHECKER;
    let mut curr_toke_beginn = 0;

    for (pos, c) in input.chars().enumerate() {

        if !(curr_token_type_checker.ends_with)(c) {
            curr_token_content.push(c);
            continue;
        }

        // push last token
        if curr_token_type != TokenType::None {
            tokens.push(Token {
                token_type: curr_token_type,
                content: curr_token_content.clone(),
                pos: curr_toke_beginn,
            });
        }

        let mut char_is_covered = false;

        for token_type_checker in TOKEN_TYPE_CHECKERS.into_iter() {
            if (token_type_checker.starts_with)(c) {
                println!("Checked {} with {} checker",
                         c,
                         token_type_checker.token_type);

                char_is_covered = true;
                // start new token
                curr_token_content = String::new();
                curr_token_content.push(c);
                curr_token_type = token_type_checker.token_type;
                curr_token_type_checker = token_type_checker;
                curr_toke_beginn = pos;

                println!("New token {}", curr_token_type);

                break;
            }
        }

        if !char_is_covered {
            // error
            println!("Error: the char '{}' is not covered by any of the supplied token type \
                      checkers",
                     c);
        }
    }

    if curr_token_type != TokenType::None {
        tokens.push(Token {
            token_type: curr_token_type,
            content: curr_token_content.clone(),
            pos: curr_toke_beginn,
        });
    }

    return tokens;
}
