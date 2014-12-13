#![feature(macro_rules)]
#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
extern crate getopts;
use std::io::BufferedReader;
use std::io::File;
use std::io;
use std::os;
use regex::Regex;

mod tokenizer;

fn main () {
    let args: Vec<String> = os::args();
    let mut code = "".to_string();
    let eol = regex!(r"$");
    let newline = regex!(r"\n");

    if args.len() > 1 {
        let file = args[1].clone();

        let path = Path::new(file);
        let mut file = BufferedReader::new(File::open(&path));
        for line in file.lines() {
            code.push_str(
                newline.replace_all(eol.replace_all(line.unwrap().as_slice(), " ").as_slice(), " \n").as_slice()
            );
        }
    } else {
        for line in io::stdin().lock().lines() {
            code.push_str(
                newline.replace_all(eol.replace_all(line.unwrap().as_slice(), " ").as_slice(), " \n").as_slice()
            );
        }
    }

    let vec = tokenizer::tokens(code.as_slice());
    for tokenline in vec.iter() {
        println!("{}", tokenline);
    }
}



// ---TESTS---
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
fn ttype_of_open_paren_recognised() {
    for open in ['{', '[', '(', '.'].iter() {
        assert!(tokenizer::ttype_of(*open) == tokenizer::TokenType::OpenParen);
    }
}
#[test]
fn ttype_of_close_paren_recognised() {
    for close in ['}', ']', ')'].iter() {
        assert!(tokenizer::ttype_of(*close) == tokenizer::TokenType::CloseParen);
    }
}
#[test]
fn ttype_of_other_chars_recognised() {
    assert!(tokenizer::ttype_of('λ') == tokenizer::TokenType::Lambda);
    assert!(tokenizer::ttype_of('↦') == tokenizer::TokenType::Assign);
    assert!(tokenizer::ttype_of(',') == tokenizer::TokenType::Separator);
}
#[test]
fn ttype_of_other_ascii_are_idents() {
    for c in ASCII_KEYS.iter() {
        assert!(tokenizer::ttype_of(*c) == tokenizer::TokenType::Identifier)
    }
}

#[test]
#[should_fail]
fn tokens_chokes_on_unmatched_paren() {
    tokenizer::tokens("(");
}
#[test]
#[should_fail]
fn tokens_chokes_on_unmatched_brace() {
    tokenizer::tokens("[");
}
#[test]
#[should_fail]
fn tokens_chokes_on_unmatched_curly() {
    tokenizer::tokens("{");
}
#[test]
#[should_fail]
fn tokens_chokes_on_unmatched_cparen() {
    tokenizer::tokens(")");
}
#[test]
#[should_fail]
fn tokens_chokes_on_unmatched_cbrace() {
    tokenizer::tokens("]");
}
#[test]
#[should_fail]
fn tokens_chokes_on_unmatched_ccurly() {
    tokenizer::tokens("}");
}

macro_rules! token(
    ($str:expr, $kind:ident) => (
        tokenizer::Token { value: $str.to_string(), ttype: tokenizer::TokenType::$kind }
    );
)
#[test]
fn tokens_recognizes_idents() {
    assert!(tokenizer::tokens("1 + 1") == vec![
            token!("1", Identifier),
            token!("+", Identifier),
            token!("1", Identifier)
        ])
}
#[test]
fn tokens_recognizes_dots() {
    assert!(tokenizer::tokens("{... }") == vec![
            token!("{", OpenParen),
            token!(".", OpenParen),
            token!(".", OpenParen),
            token!(".", OpenParen),
            token!("}", CloseParen)
        ])
}
#[test]
fn tokens_recognizes_lambdas() {
    assert!(tokenizer::tokens("{λx.λy.λz. z + y + z}") == vec![
            token!("{", OpenParen),
            token!("λ", Lambda),
            token!("x", OpenParen),
            token!(".", OpenParen),
            token!("λ", Lambda),
            token!("y", OpenParen),
            token!(".", OpenParen),
            token!("λ", Lambda),
            token!("z", OpenParen),
            token!(".", OpenParen),
            token!("z", Identifier),
            token!("+", Identifier),
            token!("y", Identifier),
            token!("+", Identifier),
            token!("z", CloseParen),
            token!("}", CloseParen)
        ])
}
fn tokens_recognizes_assigns() {
    assert!(tokenizer::tokens("main ↦ 1") == vec![
            token!("main", Identifier),
            token!("↦", Assign),
            token!("1", Identifier)
        ])
}
fn tokens_recognizes_multichar_identifiers() {
    assert!(tokenizer::tokens("{... }") == vec![
            token!("{", OpenParen),
            token!(".", OpenParen),
            token!(".", OpenParen),
            token!(".", OpenParen),
            token!("}", CloseParen),
        ])
}
