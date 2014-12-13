use std::fmt;

#[deriving(PartialEq)]
pub enum TokenType { String, Identifier, OpenParen, CloseParen, Lambda, Assign, Separator }

#[deriving(PartialEq)]
pub struct Token {
    pub value: String,
    pub ttype: TokenType
}
impl fmt::Show for Token {
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

pub fn ttype_of (chr: char) -> TokenType {
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
        _ => TokenType::Identifier
    }
}

pub fn tokens<'a>(string: &str) -> Vec<Token> {
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

        for chr in chars {
            let ttype = ttype_of(chr);
            match chr {
                '{' => paren_num += 1,
                '[' => paren_num += 1,
                '(' => paren_num += 1,
                '.' => dot_num += 1,

                '}' => {
                    paren_num -= 1;
                    while dot_num != 0 {
                        res.push(Token { value: ".".to_string(), ttype: TokenType::CloseParen });
                        dot_num -= 1;
                    }
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

            if ttype != TokenType::Identifier {
                res.push(Token { value: chr_string, ttype: ttype })
            }

            i += 1;
        }
    }
    if dot_num != 0 || paren_num != 0 { panic!("Unmatching parens!") }
    res
}
