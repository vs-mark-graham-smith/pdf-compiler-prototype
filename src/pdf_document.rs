pub mod pdf_page;
pub mod pdf_catalog;
pub mod pdf_identifier;

use std::collections::VecDeque;

use pdf_catalog::PDFCatalog;
use pdf_page::PDFPages;
use pdf_identifier::PDFObjectIdentifier;

#[derive(Debug)]
pub struct PDFDocument {
    pub id_counter: i32,
    pub catalog: PDFCatalog,
    pub pages: PDFPages
}

impl From<PDFDocument> for String {
    fn from(document: PDFDocument) -> Self {
        let mut doc_string = String::from("\n%PDF-1. 4 \n");
        // let catalog_string: String = document.catalog.into();
        doc_string.push_str(<PDFCatalog as Into<String>>::into(document.catalog).as_str());
        doc_string.push_str(<PDFPages as Into<String>>::into(document.pages).as_str());


        doc_string.push_str("
xref
trailer
    << /Size 8
        /Root 1 0 R
    >>
startxref
625
%%EOF
");

        doc_string
    }
}

impl PDFDocument {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_next_available_identifier(&mut self) -> PDFObjectIdentifier {
        self.id_counter += 1;
        PDFObjectIdentifier::new(self.id_counter.clone())

    }
}

impl Default for PDFDocument {
    fn default() -> Self {
        Self {
            id_counter: 2,
            catalog: PDFCatalog::new(
                PDFObjectIdentifier::new(1),
                PDFObjectIdentifier::new(2)
            ),
            pages: PDFPages::new(
                PDFObjectIdentifier::new(2),
                Vec::new(),
                VecDeque::new()
            )
        }
    }
}
