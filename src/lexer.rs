use std::collections::VecDeque;

use anyhow::Result;

#[allow(dead_code)]
use crate::token::{
    Token,
    Taggable,
    StartTag,
    EndTag
};

enum LexerCurrentContext {
    InTag,
    NotInTag
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
    current_context: LexerCurrentContext
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            current_context: LexerCurrentContext::NotInTag,
            ch: 0
        };

        lexer.read_char();

        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let token: Token = match self.ch {
            b'<' => {
                let token = Token::Tag(self.read_tag());
                self.current_context = LexerCurrentContext::InTag;
                return Ok(token);
            }
            b'>' => {
                self.current_context = LexerCurrentContext::NotInTag;
                Token::CloseTag
            },
            b'A'..=b'Z' | b'a'..=b'z' => {
                // IF WE ARE IN A TAG WE ARE CREATING AN ATTRIBUTE:
                match self.current_context {
                    LexerCurrentContext::InTag => return Ok(
                        Token::Attribute(self.read_attribute_as_tuple())
                    ),
                    LexerCurrentContext::NotInTag => return Ok(
                        Token::Content(self.read_content())
                    )
                }
            },
            0 => Token::EOF,
            _ => Token::Illegal
        };

        self.read_char();

        Ok(token)
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn skip_eq_and_apostrophes(&mut self) {
        while self.ch.is_ascii_whitespace() || self.ch == b'=' || self.ch == b'"' {
            self.read_char();
        }
    }

    fn read_content(&mut self) -> String {
        let pos = self.position;
        while self.ch != b'<' {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[pos..self.position]).to_string()
    }

    fn read_tag(&mut self) -> Box<dyn Taggable> {
        let pos = self.position;

        while self.ch.is_ascii_alphabetic() || self.ch == b'/' || self.ch == b'<'  {
            self.read_char();
        }

        let tag_result = String::from_utf8_lossy(&self.input[pos..self.position])
            .to_string();

        match tag_result.strip_prefix("<") {
            Some(tag_string) => {
                if tag_string.starts_with("/") {
                    return Box::new(EndTag {
                        identifier: tag_string.strip_prefix("/").expect("String").to_string()
                    });
                }

                return Box::new(StartTag {
                    identifier: tag_string.to_string()
                });
            },
            None => Box::new(StartTag { identifier: String::from("idk") })
        }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[pos..self.position]).to_string()
    }

    fn read_attribute_value(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch.is_ascii_whitespace() {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[pos..self.position]).to_string()
    }

    fn read_attribute_as_tuple(&mut self) -> (String, String) {
        (self.read_identifier(), {
            self.skip_eq_and_apostrophes();
            let string = self.read_attribute_value();
            self.skip_eq_and_apostrophes();
            string
        })
    }

    pub fn get_output(&mut self) -> VecDeque<Token> {
        let mut token_output: VecDeque<Token> = VecDeque::new();

        while let Ok(next_token) = self.next_token() {
            match next_token {
                Token::EOF => break,
                _ => token_output.push_back(next_token)
            }
        }

        token_output
    }
}
