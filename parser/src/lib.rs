enum ReservedWords {
    /// `query`
    Query,

    /// `mutation`
    Mutation,

    /// `subscription`
    Subscription,
}

/// Lexical Tokens
enum Token {
    /// `\t`(U+0009) || ` `(U+0020)
    Whitespace {
        len: u8,
    },

    /// `\n`(U+000A), carriage return ((U+000D), Carriage Return (U+000D)New Line (U+000A))
    LineTerminator,

    /// `:`, `::` - Double, might be want this token as well
    Colon,

    /// make it compatible
    Integer {
        value: i64,
    },
    Digits {
        value: String,
    },

    String {
        content: Vec<char>,
    },

    /// start from `#` until line terminate
    Comment {
        content: String,
    },

    /// `,`
    Comma,

    Name {
        content: String,
    },
}

// TODO: remove these functions
fn is_name_start(c: char) -> bool {
    return is_letter(c);
}

fn is_name_continue(c: char) -> bool {
    return is_letter(c) || is_digit(c);
}

fn is_digit(c: char) -> bool {
    let d = c as u64;
    match d {
        0..=9 => return true,
        _ => return false,
    }
}

fn is_letter(c: char) -> bool {
    match c {
        'A'..='Z' | 'a'..='z' => return true,
        _ => return false,
    }
}

fn is_punctuator(c: char) -> bool {
    match c {
        // special case `...`: to ensure we need two more tokens
        '!' | '$' | '&' | '(' | ')' | '=' | '@' | '[' | ']' | '{' | '}' | ':' => return true,
        _ => return false,
    }
}

/// Lexical Analysis to build AST
/// following Unicode specs, ASCII standard is better for displaying (widely supported)
/// surrogate pair is not valid
#[derive(Debug, Clone)]
pub struct Lexer {
    // TODO: replace this source_code field type, better be compatible with every source code
    // characters
    /// U+0000 to U+D7FF or U+E000 to U+10FFFF (characters), these are the only supported tokens
    source_chars: Vec<u8>,
}

/// Syntactic Parse
#[derive(Debug, Clone, Copy)]
pub struct Parser {}
