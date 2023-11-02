use super::PDFObjectKind;

#[derive(Debug)]
pub struct PDFText;

impl PDFText {
    pub fn new() -> PDFText {
        PDFText::default()
    }
}

impl Default for PDFText {
    fn default() -> Self {
        PDFText {}
    }
}

impl PDFObjectKind for PDFText {}
