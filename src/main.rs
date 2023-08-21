use lexer::scanner::Scanner;

mod lexer;
mod parser;
mod vm;

fn main() {
    let source = std::fs::read("./test.hd").unwrap();
    // dbg!(&source);
    let scanner = Scanner::new(&source);
    for token in scanner {
        dbg!(token.unwrap());
    }
}
