enum TokenType { StringVal, Operator, Name, Number, Nothing }
struct Token<T> {
    token_type: TokenType,
    value: T,
    from: int,
    to: int
}

fn tokens_of(string: &str) -> [Token] { tokens("", "", string) }
fn tokens (prefix: &str, suffix: &str, string: &str) -> [Token] {
    let c: &str;                      // The current character.
    let from: int;                   // The index of the start of the token.
    let i = 0i;                  // The index of the current character.
    let n: int;                      // The number value.
    let q: &str;                      // The quote character.
    let strval: &str;                    // The string value.
    let length = string.len();

    let result: Vec<Token> = vec![];            // An array to hold the results.
    let make = |ttype: TokenType, value| {
        Token {
            token_type: ttype,
            value: value,
            from: from,
            to: i
        }
    };

    // Begin tokenization. If the source string is empty, return nothing.
    if !string {
        return;
    }

    // If prefix and suffix strings are not provided, supply defaults.
    if prefix == "" {
        prefix = "<>+-&";
    }
    if suffix == "" {
        suffix = "=>&:";
    }


    // Loop through string text, one character at a time.

    c = string.get(i);
    while (c) {
        from = i;

        // Ignore whitespace.

        if (c <= " ") {
            i += 1;
            c = string.get(i);
            // name.
        } else if (c >= "a" && c <= "z") || (c >= "A" && c <= "Z") {
            strval = c;
            i += 1;
            loop {
                c = string.get(i);
                if (c >= "a" && c <= "z") || (c >= "A" && c <= "Z") ||
                    (c >= "0" && c <= "9") || c == "_" {
                    strval += c;
                    i += 1;
                } else {
                    break;
                }
            }
            result.push(make(TokenType::Name, strval));

            // number.

            // A number cannot start with a decimal point. It must start with a digit,
            // possibly "0".

        } else if c >= "0" && c <= "9" {
            strval = c;
            i += 1;

            // Look for more digits.

            loop {
                c = string.get(i);
                if (c < "0" || c > "9") {
                    break;
                }
                i += 1;
                strval += c;
            }

            // Look for a decimal fraction part.

            if c == "." {
                i += 1;
                strval += c;
                loop {
                    c = string.get(i);
                    if c < "0" || c > "9" {
                        break;
                    }
                    i += 1;
                    strval += c;
                }
            }

            // Look for an exponent part.

            if c == "e" || c == "E" {
                i += 1;
                strval += c;
                c = string.get(i);
                if c == "-" || c == "+" {
                    i += 1;
                    strval += c;
                    c = string.get(i);
                }
                if c < "0" || c > "9" {
                    make(TokenType::Number, strval).error("Bad exponent");
                }
                while (c >= "0" && c <= "9") || i == 0 {
                    i += 1;
                    strval += c;
                    c = string.charAt(i);
                }
            }

            // Make sure the next character is not a letter.

            if c >= "a" && c <= "z" {
                strval += c;
                i += 1;
                make(TokenType::Number, strval).error("Bad number");
            }

            // Convert the string value to a number. If it is finite, then it is a good
            // token.

            n = strval as int;
            if (n.is_finite()) {
                result.push(make(TokenType::Number, n));
            } else {
                make(TokenType::Number, strval).error("Bad number");
            }

            // strvaling

        } else if (c == "\"" || c == "'") {
            strval = "";
            q = c;
            i += 1;
            loop {
                c = string.get(i);
                if (c < " ") {
                    make(TokenType::StringVal, strval).error(if c == "\n" || c == "\r" || c == "" {
                                                  "Unterminated string."
                                              } else {
                                                  "Control character in string."
                                              }, make(TokenType::Nothing, strval));
                }

                // Look for the closing quote.

                if (c == q) {
                    break;
                }

                // Look for escapement.

                if (c == "\\") {
                    i += 1;
                    if (i >= length) {
                        make(TokenType::StringVal, strval).error("Unterminated string");
                    }
                    c = string.get(i);
                    match c {
                        "n" => (c = "\n"),
                        "r" => (c = "\r"),
                        "t" => (c = "\t"),
                        "u" => {
                            if (i >= length) {
                                make(TokenType::StringVal, strval).error("Unterminated string");
                            }
                            c = string.slice(i + 1, 4) as int;
                            if (c.is_infinite() || c < 0) {
                                make(TokenType::StringVal, strval).error("Unterminated string");
                            }
                            c = String::from_char(1, c as char).as_slice();
                            i += 4;
                        }
                    }
                }
                strval += c;
                i += 1;
            }
            i += 1;
            result.push(make(TokenType::StringVal, strval));
            c = string.get(i);

            // comment.

        } else if (c == "/" && string[i + 1] == "/") {
            i += 1;
            loop {
                c = string.get(i);
                if (c == "\n" || c == "\r" || c == "") {
                    break;
                }
                i += 1;
            }

            // combining

        } else if (prefix.indexOf(c) >= 0) {
            strval = c;
            i += 1;
            loop {
                c = string.charAt(i);
                if (i >= length || suffix.indexOf(c) < 0) {
                    break;
                }
                strval += c;
                i += 1;
            }
            result.push(make(TokenType::Operator, strval));

            // single-character operator

        } else {
            i += 1;
            result.push(make(TokenType::Operator, c));
            c = string.charAt(i);
        }
    }
    return result;
}
fn main () {
    println!("{}", tokens_of("1 + 1"));
}
