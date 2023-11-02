pub mod pdf_object;

use pdf_object::PDFObject;
use pdf_object::PDFObjectKind;
use pdf_object::pdf_page::PDFPage;

use pdf::PDFPage;

#[derive(Debug)]
pub struct PDFDocument {
    pages: Vec<PDFPage>,
}

impl PDFDocument {
    pub fn new() -> PDFDocument {
        PDFDocument {
            pages: Vec::new()
        }
    }

    pub fn push_object(
        &mut self,
        page: PDFPage
    ) {
        self.pages.push(page)
    }
}

impl Default for PDFDocument {
    fn default() -> Self {
        Self {
            objects: Vec::new()
        }
    }
}
