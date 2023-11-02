#[derive(Debug)]
pub struct PDFObject {
    kind: PDFObjectKind,
    children: Vec<PDFObject>
}

#[derive(Debug)]
pub enum PDFObjectKind {
    Text(String)
}

// #[derive(Debug)]
//pub struct PDFObjectIdentifier(i32, i32);
