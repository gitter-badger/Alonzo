#![stable = "apart from some refactors in the near future, this file is unlikely to change."]
use std::fmt;

#[deriving(PartialEq)]
/// The different token types that are possible.
///
/// TokenType represents the different kinds of tokens there are. One of these is assigned to each
/// of the tokens that are created.
pub enum TokenType { String, Identifier, OpenParen, CloseParen, Lambda, Assign, Separator }

#[deriving(PartialEq)]
/// Tokens are representations of the items in a program.
///
/// Instead of parsing directly on the text, wich would lead to messy, mangled code, most languages
/// use a tokenizer to seperate out concerns. The tokenizer figures out what a thing is, and builds
/// more understandable ASTs out of that, assigning more metadata as it goes. Token is a struct
/// that represents the basic metadata attached to a token.
pub struct Token {
    pub value: String,
    pub ttype: TokenType
}
/// We have implemented the Show trait for Token for easy debugging.
impl fmt::Show for Token {
#![stable = "will most likely never change"]
    /// Here, when asked for formatting, Token returns a string of this format: "(value: '{}', type: {})". In the type, since you cannot implement traits for enumerations, it does a match on it, returning a stringified name.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(value: '{}', type: {})", self.value, match self.ttype {
            TokenType::String => "String",
            TokenType::Identifier => "Identifier",
            TokenType::OpenParen => "OpenParen",
            TokenType::CloseParen => "CloseParen",
            TokenType::Lambda => "Lambda",
            TokenType::Separator => "Separator",
            TokenType::Assign => "Assign"
        })
    }
}

/// Although small and fairly simple, ttype_of is the heart and soul of the tokenizer.
///
/// It assigns and returns the proper metadata for each token fed into it. Here is the table:
///
///         '{': OpenParen
///         '[': OpenParen
///         '(': OpenParen
///         '.': OpenParen
///         ',': Separator
///         '}': CloseParen
///         ']': CloseParen
///         ')': CloseParen
///         'λ': Lambda
///         '↦': Assign
///          _ : Identifier

pub fn ttype_of (chr: char) -> TokenType {
#![stable = "will probably never change, and is perfectly clear and well-tested"]
    match chr {
        '{' => TokenType::OpenParen,
        '[' => TokenType::OpenParen,
        '(' => TokenType::OpenParen,
        '.' => TokenType::OpenParen,
        ',' => TokenType::Separator,
        '}' => TokenType::CloseParen,
        ']' => TokenType::CloseParen,
        ')' => TokenType::CloseParen,
        'λ' => TokenType::Lambda,
        '↦' => TokenType::Assign,
         _  => TokenType::Identifier
    }
}

/// The tokens function is the center of the tokenizer. It splits, separates, checks, matadatas,
/// and organizes each and every character in the code string.
///
/// Tokens goes through the given code, line by line, and spits each line up into characters, then,
/// iterating over each character, examines it's type using ttype_of. Then, based on the type, and
/// the action also associated with that token, takes action. Then, it pushes the token onto the
/// token vector. There is a special tokenizer action assiciated with most special characters.
/// These special token actions can only be perforemed within the tokenizer, otherwise it would be
/// to late to make these expantions and changes.
pub fn tokens(string: &str) -> Vec<Token> {
#![unstable = "apart from using an ugly hack to retain last identifier on line if there is no non-identifier token after it"]
    let mut res: Vec<Token> = vec![];
    let mut paren_num = 0i;
    let mut dot_num = 0i;

    let mut lines = string.split_str("\n");
    for line in lines {
        let mut chars = line.chars();
        let mut current_ident = "".to_string();
        let mut quote = 0i;
        let mut stringbq = "".to_string();
        let mut i = 0;
        let mut brace = false;

        for chr in chars {
            let ttype = ttype_of(chr);
            match chr {
                '{' => paren_num += 1,
                '[' => paren_num += 1,
                '(' => paren_num += 1,
                '.' => dot_num += 1,

                '}' => {
                    paren_num -= 1;
                    brace = true;
                },
                ']' => paren_num -= 1,
                ')' => paren_num -= 1,
                'λ' => {},
                ',' => {},
                '↦' => {},
                '"' => {
                    if quote == 1 {
                        res.push(Token { value: String::from_str(stringbq.as_slice()), ttype: TokenType::String });
                        stringbq = "".to_string();
                        quote = 0;
                    } else {
                        quote += 1
                    }
                },
                ' ' => { if quote == 1 { stringbq.push(' ') } },
                _ => {
                    if quote == 1 && chr != '"' {
                        stringbq.push(chr);
                    } else {
                        current_ident.push(chr);
                    }
                }
            }
            let mut chr_in_nonwords = false;

            for c in ['{', '}', '(', ')', '[', ']', ',', '.', 'λ', '"', '↦', '\n', ' '].iter() {
                chr_in_nonwords = chr == *c;
                if chr_in_nonwords { break; }
            }

            if (chr_in_nonwords || i == line.len()-1) && current_ident != "".to_string() {
                res.push(Token { value: current_ident, ttype: ttype });
                current_ident = "".to_string();
            }


            let chr_string = String::from_char(1, chr);

            if ttype != TokenType::Identifier && chr != '}' {
                res.push(Token { value: chr_string, ttype: ttype })
            } else if brace {
                while dot_num != 0 {
                    res.push(Token { value: ".".to_string(), ttype: TokenType::CloseParen });
                    dot_num -= 1;
                }
                res.push(Token { value: "}".to_string(), ttype: ttype });
                brace = false;
            }
            i += 1;
        }
    }
    if dot_num != 0 || paren_num != 0 { panic!("Unmatching parens!") }
    res
}
