// This file contains the parser, it accepts tokens and returns PDFDocument
//

use std::collections::VecDeque;

use crate::token::Token;
use crate::pdf_document::{
    PDFDocument,
    pdf_page::PDFPages,
    pdf_page::PDFPage
};

pub struct Parser {
    tokens: VecDeque<Token>,
    document: PDFDocument,
    start_tag_opened: bool,
    start_tag_closed: bool,
    end_tag_opened: bool
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self {
            tokens,
            ..Self::default()
        }
    }

    pub fn get_document(mut self) -> PDFDocument {
        self.build_pages();

        self.document
    }

    fn build_pages(&mut self) {
        while let Some(page) = self.build_next_page() {
            self.document.pages.add_page(page);
            self.start_tag_opened = false;
            self.start_tag_closed = false;
            self.end_tag_opened = false;
        }
    }

    fn build_next_page(&mut self) -> Option<PDFPage> {
        // Here we need to get the next page object, therefore starting from our pointer,
        // we need to start reading tokens, when a page tag is opened, we continue
        // reading until it is then </closed>
        let mut pdf_page: Option<PDFPage> = None;

        while let Some(token) = self.get_next_token() {
            match token {
                // If we aren't building a page here, we're doing it wrong.
                Token::CloseTag => {
                    // 1) Start Tag (Page)
                    // 2) Close Tag

                    // I want to return, if page has just now been closed.
                    if (
                        self.start_tag_opened
                        && self.start_tag_closed
                        && self.end_tag_opened
                    ) {
                        self.start_tag_opened = false;
                        self.start_tag_closed = false;
                        self.end_tag_opened = false;

                        break;
                    }

                    if self.start_tag_opened {
                        self.start_tag_closed = true
                    }
                },
                Token::Tag(tag_type) => {
                    match tag_type.get_name() {
                        "parser_v2::token::StartTag" => {
                            match tag_type.retrieve_identifier() {
                                "Page" => {
                                    pdf_page = Some(PDFPage::new(
                                        self.document.get_next_available_identifier(),
                                        self.document.pages.identifier.clone()
                                    )
                                    );
                                    self.start_tag_opened = true;
                                },
                                _ => panic!("I don't know what to do here.")
                            }
                        }
                        "parser_v2::token::EndTag" => { // When page is closed, we return the page if it exists
                            // If this end tag is </Page> and we have Some(pdf_page) return PDFPage, as we have added it's attributes and objects.
                            match pdf_page {
                                Some(_) => self.end_tag_opened = true,
                                None => panic!("Unexpected End Tag"),
                            }
                        },
                        _ => {
                            panic!("Syntax Error: Expected Page.");
                        }
                    }
                },
                _ => panic!("Unhandled Token Type")
            }
        }

        pdf_page
    }

    fn get_next_token(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            tokens: VecDeque::new(),
            document: PDFDocument::new(),
            start_tag_opened: false,
            start_tag_closed: false,
            end_tag_opened: false
        }
    }
}
