extern crate getopts;
use std::io::BufferedReader;
use std::io::File;
use std::io;
use std::os;

mod tokenizer;

fn main () {
    let args: Vec<String> = os::args();
    let mut code = "".to_string();

    if args.len() > 1 {
        let file = args[1].clone();

        let path = Path::new(file);
        let mut file = BufferedReader::new(File::open(&path));
        for line in file.lines() {
            code.push_str((line.unwrap()).as_slice());
        }
    } else {
        for line in io::stdin().lock().lines() {
            code.push_str((line.unwrap()).as_slice());
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
#[test]
fn tokens_recognizes_idents() {
    assert!(tokenizer::tokens("1 + 1") == vec![
            tokenizer::Token { value: "1".to_string(), ttype: tokenizer::TokenType::Identifier },
            tokenizer::Token { value: "+".to_string(), ttype: tokenizer::TokenType::Identifier },
            tokenizer::Token { value: "1".to_string(), ttype: tokenizer::TokenType::Identifier }
        ])
}
