#[derive(Debug, Clone)]
pub struct PDFObjectIdentifier(pub i32);

impl PDFObjectIdentifier {
    pub fn new(i: i32) -> Self {
        Self(i)
    }

    pub fn get_ref(&self) -> String {
        let mut pages_identifier = self.0.to_string();
        pages_identifier.push_str(" 0 R ");
        pages_identifier
    }
}

impl From<PDFObjectIdentifier> for String {
    fn from(value: PDFObjectIdentifier) -> Self {
        let mut str = value.0.clone().to_string();
        str.push_str(" 0 obj \n");
        str
    }
}
