pub mod pdf_page;
pub mod pdf_text;

pub struct PDFObject {
    attributes: Option<Vec<(String, String)>>,
    parent: Option<Box<PDFObjectKind>>,
    children: Option<Vec<PDFObjectKind>>,
    pdf_object_kind: PDFObjectKind,
}

impl PDFObject {
    pub fn new(pdf_object_kind: PDFObjectKind) -> Self {
        Self {
            pdf_object_kind,
            ..Self::default()
        }
    }

    pub fn add_attributes(&mut self, attributes: Vec<(String, String)>) -> Self {
        self.attributes = Some(attributes);
        self
    }
}

pub enum PDFObjectKind {

}

// pub trait PDFObjectKind {
//     fn get_name(&self) -> &'static str {
//         std::any::type_name::<Self>()
//     }
// }

impl Default for PDFObject {
    fn default() -> Self {
        Self {
            attributes: None,
            parent: None,
            children: None,
            pdf_object_kind: ()
        }
    }
}

impl std::fmt::Debug for PDFObjectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PDFObject: {{{}}}",
            self.get_name(),
        )
    }
}
