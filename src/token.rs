pub trait Taggable {
    fn get_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn retrieve_identifier(&self) -> &str;
}

use core::fmt::Debug;

impl core::fmt::Debug for dyn Taggable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}{{{}}}",
            self.get_name(),
            self.retrieve_identifier()
        )
    }
}

impl core::fmt::Display for dyn Taggable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{{{}}}",
            self.get_name(),
            self.retrieve_identifier()
        )
    }
}

#[derive(Debug)]
pub struct StartTag {
    pub identifier: String
}

#[derive(Debug)]
pub struct EndTag {
    pub identifier: String
}

impl Taggable for StartTag {
    fn retrieve_identifier(&self) -> &str {
        &self.identifier
    }
}

impl Taggable for EndTag {
    fn retrieve_identifier(&self) -> &str {
        &self.identifier
    }
}

#[derive(Debug)]
pub enum Token {
    Tag(Box<dyn Taggable>),
    CloseTag,
    Attribute((String, String)),
    Content(String),
    Illegal,
    EOF,
}
