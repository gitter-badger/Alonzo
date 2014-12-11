#![feature(macro_rules)]
enum TokenType { StringVal, Operator, Name, Number, Nothing }
struct Token {
    token_type: TokenType,
    string_value: Option<&str>,
    number_value: Option<int>,
    from: uint,
    to: uint
}
impl Token {
    fn value () -> (Option<int>, Option<&str>) {
        match number_value {
            n @ Some(_) => (n, None),
            None => (None, match string_value {
                s @ Some(_) => (None, s),
                None => (None, None)
            })
        }
    }
}

fn has_char (c: Option<&char>) -> (bool, char) {
    match c {
        Some(c) => (true, *c),
        None    => (false, ' ')
    }
}
trait Val {
    fn dummy () {}
}

fn tokens(prefix: &str, suffix: &str, strr: &str) -> Vec<Token> {
let mut successfull_get: bool = true;
    macro_rules! get(
        ($idx:expr, $thing:expr, $to:ident) => (
            match $thing.get($idx) {
                Some(c) => {
                    successfull_get = true;
                    $to = *c;
                },
                None    => successfull_get = false
            }
        );
    )

    let mut c: char;                      // The current character.
    let mut from: uint;                   // The index of the start of the token.
    let mut i = 0u;                  // The index of the current character.
    let mut n: int;                      // The number value.
    let mut q: &str;                      // The quote character.
    let mut strval: String;                    // The string value.
    let length = strr.len();  // Vectors have no set length!
    let mut string: Vec<char> = vec![];  // You can't access a specific index into a string slice or string, so we have to convert ours to a vector! (0_0)
    for c in strr.bytes() { // Iterate  over the bytes in the &str
        string.push(c as char)  // Add that char to the vector named string
    }


    let mut result: Vec<Token> = vec![];            // An array to hold the results.
    let make = |ttype: TokenType, value| {
        let number_value: int;
        let string_value: &str;

    };

    // Begin tokenization. If the source string is empty, return nothing.
    if string == "" {
        return result
    }

    // If prefix and suffix strings are not provided, supply defaults.
    if prefix == "" {
        prefix = "<>+-&";
    }
    if suffix == "" {
        suffix = "=>&:";
    }


    // Loop through string text, one character at a time.

    get!(i, string, c);

    while successfull_get {
        from = i;

        // Ignore whitespace.

        if c <= ' ' {
            i += 1;
            get!(i, string, c);
            // name.
        } else if (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') {
            strval = String::from_char(1, c);
            i += 1;
            loop {
                get!(i, string, c);
                if (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') ||
                    (c >= '0' && c <= '9') || c == '_' {
                    strval.push(c);
                    i += 1;
                } else {
                    break;
                }
            }
            result.push(make(TokenType::Name, strval));

            // number.

            // A number cannot start with a decimal point. It must start with a digit,
            // possibly '0'.

        } else if c >= '0' && c <= '9' {
            strval = String::from_char(1, c);
            i += 1;

            // Look for more digits.

            loop {
                get!(i, string, c);
                if c < '0' || c > '9' {
                    break;
                }
                i += 1;
                strval.push(c);
            }

            // Look for a decimal fraction part.

            if c == '.' {
                i += 1;
                strval.push(c);
                loop {
                    get!(i, string, c);
                    if c < '0' || c > '9' {
                        break;
                    }
                    i += 1;
                    strval.push(c);
                }
            }

            // Look for an exponent part.

            if c == 'e' || c == 'E' {
                i += 1;
                strval.push(c);
                get!(i, string, c);
                if c == '-' || c == '+' {
                    i += 1;
                    strval.push(c);
                    get!(i, string, c);
                }
                if c < '0' || c > '9' {
                    make(TokenType::Number, strval).error("Bad exponent");
                }
                while (c >= '0' && c <= '9') || i == 0 {
                    i += 1;
                    strval.push(c);
                    get!(i, string, c);
                }
            }

            // Make sure the next character is not a letter.

            if c >= 'a' && c <= 'z' {
                strval.push(c);
                i += 1;
                make(TokenType::Number, strval).error("Bad number");
            }

            // Convert the string value to a number. If it is finite, then it is a good
            // token.

            n = from_str(strval.as_slice()).unwrap();
            result.push(make(TokenType::Number, n));

            // strvaling

        } else if c == '\"' || c == '\'' {
            strval = "".to_string();
            q = c.to_string().as_slice();
            i += 1;
            loop {
                get!(i, string, c);
                if (c < ' ') {
                    make(TokenType::StringVal, strval).error(if c == '\n' || c == '\r' {
                                                  "Unterminated string."
                                              } else {
                                                  "Control character in string."
                                              }, make(TokenType::Nothing, strval));
                }

                // Look for the closing quote.

                if c == q {
                    break;
                }

                // Look for escapement.

                if c == '\\' {
                    i += 1;
                    if i >= length {
                        make(TokenType::StringVal, strval).error("Unterminated string");
                    }
                    get!(i, string, c);
                    match c {
                        'n' => (c = '\n'),
                        'r' => (c = '\r'),
                        't' => (c = '\t'),
                        'u' => {
                            if (i >= length) {
                                make(TokenType::StringVal, strval).error("Unterminated string");
                            }
                            c = string.slice(i + 1, 4) as int;
                            if c < 0 {
                                make(TokenType::StringVal, strval).error("Unterminated string");
                            }
                            c = String::from_char(1, c as char).as_slice();
                            i += 4;
                        }
                    }
                }
                strval.push(c);
                i += 1;
            }
            i += 1;
            result.push(make(TokenType::StringVal, strval));
            get!(i, string, c);

            // comment.

        } else if c == '/' && string[i + 1] == '/' {
            i += 1;
            loop {
                get!(i, string, c);
                if c == '\n' || c == '\r' {
                    break;
                }
                i += 1;
            }

            // combining

        } else if (prefix.indexOf(c) >= 0) {
            strval = String::from_char(1, c);
            i += 1;
            loop {
                get!(i, string, c);
                if i >= length || suffix.indexOf(c) < 0 {
                    break;
                }
                strval.push(c);
                i += 1;
            }
            result.push(make(TokenType::Operator, strval));

            // single-character operator

        } else {
            i += 1;
            result.push(make(TokenType::Operator, c));
            get!(i, string, c);
        }
    }
    return result
}
fn main () {
    println!("{}", tokens("", "", "1 + 1"));
}
