mod pdf_object;

use std::collections::VecDeque;

use pdf_object::PDFObject;
use crate::pdf_document::pdf_identifier::PDFObjectIdentifier;

#[derive(Debug)]
pub struct PDFPages {
    pub identifier: PDFObjectIdentifier,
    pub page_identifiers: Vec<PDFObjectIdentifier>,
    pub pages: VecDeque<PDFPage>,
}

impl From<PDFPages> for String {
    fn from(pages: PDFPages) -> Self {
        let mut return_string: String = pages.identifier.into();

        return_string.push_str("<< /Type /Pages
        /Kids [ ");

        for page_identifier in &pages.page_identifiers {
            return_string.push_str(page_identifier.get_ref().as_str());
        }
        return_string.push_str("]
        /Count ");

        return_string.push_str(pages.page_identifiers.len().to_string().as_str());

        return_string.push_str("\n>>\nendobj\n");

        for page in pages.pages {
            return_string.push_str(<PDFPage as Into<String>>::into(page).as_str());
        }

        return_string
    }
}

impl PDFPages {
    pub fn new(
        identifier: PDFObjectIdentifier,
        page_identifiers: Vec<PDFObjectIdentifier>,
        pages: VecDeque<PDFPage>,
    ) -> Self {
        Self {
            identifier,
            page_identifiers,
            pages
        }
    }

    pub fn add_page(&mut self, page: PDFPage) {
        self.page_identifiers.push(page.get_identifier());
        self.pages.push_back(page);
    }
}

#[derive(Debug)]
pub struct PDFPage {
    parent_identifier: PDFObjectIdentifier,
    identifier: PDFObjectIdentifier,
    objects: Vec<PDFObject>,
}

impl From<PDFPage> for String {
    fn from(page: PDFPage) -> Self {
        let mut return_string: String = page.identifier.into();


        return_string.push_str("<< /Type /Page
        /Parent ");

        return_string.push_str(page.parent_identifier.get_ref().as_str());

        return_string.push_str("
        /MediaBox [ 0 0 612 792 ]
        /Resources <<>>
    >>
endobj
");
        return_string
    }
}

impl PDFPage {
    pub fn new(
        identifier: PDFObjectIdentifier,
        parent_identifier: PDFObjectIdentifier
    ) -> Self {
        Self {
            parent_identifier,
            identifier,
            objects: Vec::new()
        }
    }

    pub fn get_identifier(&self) -> PDFObjectIdentifier {
        self.identifier.clone()
    }
}
