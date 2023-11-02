use crate::pdf_document::PDFDocument;

pub struct Compiler {
    pdf_document: PDFDocument,
    pdf_string: String,
    catalog_pages: Vec<i32>
}

impl Compiler {
    pub fn new(pdf_document: PDFDocument) -> Self {
        Self {
            pdf_document,
            ..Self::default()
        }
    }

    pub fn compile_pdf(&mut self) {
        // We need to look at each page and create it
        self.compile_pages()
    }

    fn compile_pages(&mut self) {
        while let Some(string) = self.compile_page() {

        }
    }

    fn compile_page(&mut self) -> Option<String> {
        // Pop first page and compile it
        if let Some(page) = self.pdf_document.pages.pop_front() {
            let mut return_string = String::new();

            let page_identifier = page.get_identifier().to_string();

            return_string.push_str(page_identifier);

            return_string.push_str(" 0 obj \n");

            let obj_end = String::from("
                << /Type /Page
                    /Parent 3 0 R
                    /MediaBox [ 0 0 612 792 ]
                    /Contents 5 0 R
                    /Resources << /ProcSet 6 0 R
                        /Font << /F1 7 0 R >>
                    >>
                >>
            endobj
            ")
        }
        None
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            pdf_document: PDFDocument::new(),
            pdf_string: String::from(""),
            catalog_pages: Vec::new()
        }
    }
}
