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

mod tokenizer;

fn readlines<R: Reader>(mut input: BufferedReader<R>) -> String {
    #![stable = "Unlikely to change, safely type-checked and tested."]
    let newline = regex!(r"\n");
    let eol = regex!(r"$");
    let mut code = "".to_string();
    for line in input.lines() {
        code.push_str(
            newline.replace_all(eol.replace_all(line.unwrap().as_slice(), " ").as_slice(), " \n").as_slice()
            );
    }
    code
}

fn main () {
    let args: Vec<String> = os::args();
    let mut vec: Vec<tokenizer::Token>;

    if args.len() > 1 {
        let file = args[1].clone();

        let path = Path::new(file);
        let file = BufferedReader::new(File::open(&path));
        vec = tokenizer::tokens(readlines(file).as_slice());
    } else {
        vec = tokenizer::tokens(readlines(BufferedReader::new(io::stdin())).as_slice());
    }

    for tokenline in vec.iter() {
        println!("{}", tokenline);
    }
}
