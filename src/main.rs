#[allow(dead_code)]

mod lexer;
mod token;
mod parser;
// mod compiler;
mod pdf_document;

use lexer::Lexer;
// use compiler::Compiler;
use parser::Parser;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut lexer = Lexer::new("
    <Page>
    </Page>
".into());

    let tokens = lexer.get_output();
    let parser = Parser::new(tokens);
    let doc = parser.get_document();

    write_to_file(doc.into())
}

fn write_to_file(string: String) -> std::io::Result<()> {
    let mut file = File::create("test.pdf")?;
    file.write_all(string.as_bytes())?;
    Ok(())
}
