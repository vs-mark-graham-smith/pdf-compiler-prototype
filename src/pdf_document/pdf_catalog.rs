use super::pdf_identifier::PDFObjectIdentifier;

#[derive(Debug)]
pub struct PDFCatalog {
    identifier: PDFObjectIdentifier,
    pages_identifier: PDFObjectIdentifier
}

impl PDFCatalog {
    pub fn new(
        identifier: PDFObjectIdentifier,
        pages_identifier: PDFObjectIdentifier
    ) -> Self {
        Self {
            identifier,
            pages_identifier
        }
    }
}

impl From<PDFCatalog> for String {
    fn from(catalog: PDFCatalog) -> Self {
        let mut catalog_obj: String = catalog.identifier.into();

        catalog_obj.push_str("<<
    /Type /Catalog
    /Pages ");

        catalog_obj.push_str(catalog.pages_identifier.get_ref().as_str());
        catalog_obj.push_str("\n>>\nendobj\n");
        catalog_obj
    }
}
