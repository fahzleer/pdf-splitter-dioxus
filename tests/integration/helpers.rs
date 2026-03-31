use std::path::Path;
use lopdf::{Dictionary, Document, Object, Stream};

pub fn create_test_pdf(path: &Path, pages: usize) {
    let mut doc = Document::with_version("1.4");
    let pages_id = doc.new_object_id();
    let mut kids = vec![];

    for i in 1..=pages {
        let content = Stream::new(
            Dictionary::new(),
            format!("BT /F1 12 Tf 100 700 Td (Page {}) Tj ET", i).into_bytes(),
        );
        let content_id = doc.add_object(content);

        let mut page_dict = Dictionary::new();
        page_dict.set("Type", Object::Name(b"Page".to_vec()));
        page_dict.set("Parent", Object::Reference(pages_id));
        page_dict.set("MediaBox", Object::Array(vec![
            Object::Integer(0), Object::Integer(0),
            Object::Integer(612), Object::Integer(792),
        ]));
        page_dict.set("Contents", Object::Reference(content_id));

        let page_id = doc.add_object(Object::Dictionary(page_dict));
        kids.push(Object::Reference(page_id));
    }

    let mut pages_dict = Dictionary::new();
    pages_dict.set("Type", Object::Name(b"Pages".to_vec()));
    pages_dict.set("Kids", Object::Array(kids));
    pages_dict.set("Count", Object::Integer(pages as i64));
    doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

    let mut catalog_dict = Dictionary::new();
    catalog_dict.set("Type", Object::Name(b"Catalog".to_vec()));
    catalog_dict.set("Pages", Object::Reference(pages_id));
    let catalog_id = doc.add_object(Object::Dictionary(catalog_dict));

    doc.trailer.set("Root", Object::Reference(catalog_id));
    doc.save(path).unwrap();
}
