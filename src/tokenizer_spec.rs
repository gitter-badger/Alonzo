#![feature(macro_rules)]
#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
extern crate getopts;
mod tokenizer;

#[cfg(test)]
mod ttype_of_should {
    use tokenizer;
    static ASCII_KEYS: [char, ..75] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
        'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A',
        'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '*',
        '&', '^', '-', '+', '=', '%', '$', '#', '@',
        '!',                '~',                '`'
    ];

#[test]
    fn recognise_open_paren() {
        for open in ['{', '[', '(', '.'].iter() {
            assert!(tokenizer::ttype_of(*open) == tokenizer::TokenType::OpenParen);
        }
    }
#[test]
    fn recognise_close_paren() {
        for close in ['}', ']', ')'].iter() {
            assert!(tokenizer::ttype_of(*close) == tokenizer::TokenType::CloseParen);
        }
    }
#[test]
    fn recognise_other_characters() {
        assert!(tokenizer::ttype_of('λ') == tokenizer::TokenType::Lambda);
        assert!(tokenizer::ttype_of('↦') == tokenizer::TokenType::Assign);
        assert!(tokenizer::ttype_of(',') == tokenizer::TokenType::Separator);
    }
#[test]
    fn everything_else_be_ident() {
        for c in ASCII_KEYS.iter() {
            assert!(tokenizer::ttype_of(*c) == tokenizer::TokenType::Identifier)
        }
    }
}

#[cfg(test)]
mod tokenizer_should {
    use tokenizer;
#[test]
#[should_fail]
    fn choke_on_unmatched_paren() {
        tokenizer::tokens("(");
    }
#[test]
#[should_fail]
    fn choke_on_unmatched_brace() {
        tokenizer::tokens("[");
    }
#[test]
#[should_fail]
    fn choke_on_unmatched_curly() {
        tokenizer::tokens("{");
    }
#[test]
#[should_fail]
    fn choke_on_unmatched_cparen() {
        tokenizer::tokens(")");
    }
#[test]
#[should_fail]
    fn choke_on_unmatched_cbrace() {
        tokenizer::tokens("]");
    }
#[test]
#[should_fail]
    fn choke_on_unmatched_ccurly() {
        tokenizer::tokens("}");
    }

    macro_rules! token(
        ($str:expr, $kind:ident) => (
            tokenizer::Token { value: $str.to_string(), ttype: tokenizer::TokenType::$kind }
            );
        )
#[test]
        fn recognize_idents() {
            assert!(tokenizer::tokens("1 + 1") == vec![
                    token!("1", Identifier),
                    token!("+", Identifier),
                    token!("1", Identifier)
                    ])
        }
#[test]
    fn recognize_dots() {
        assert!(tokenizer::tokens("{... }") == vec![
                token!("{", OpenParen),
                token!(".", OpenParen),
                token!(".", OpenParen),
                token!(".", OpenParen),

                token!(".", CloseParen),
                token!(".", CloseParen),
                token!(".", CloseParen),
                token!("}", CloseParen),
                ])
    }
#[test]
    fn recognize_lambdas() {
        assert!(tokenizer::tokens("{λx.λy.λz. x + y + z}") == vec![
                token!('{', OpenParen),
                token!('λ', Lambda),
                token!('x', OpenParen),
                token!('.', OpenParen),
                token!('λ', Lambda),
                token!('y', OpenParen),
                token!('.', OpenParen),
                token!('λ', Lambda),
                token!('z', OpenParen),
                token!('.', OpenParen),
                token!('x', Identifier),
                token!('+', Identifier),
                token!('y', Identifier),
                token!('+', Identifier),
                token!('z', CloseParen),
                token!('.', CloseParen),
                token!('.', CloseParen),
                token!('.', CloseParen),
                token!('}', CloseParen)
                ])
    }
    fn recognize_assigns_and_identifiers() {
        assert!(tokenizer::tokens("main ↦ 1") == vec![
                token!("main", Identifier),
                token!("↦", Assign),
                token!("1", Identifier)
                ])
    }
}
