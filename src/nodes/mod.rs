use unstructured::Document;

#[derive(Debug)]
pub enum Node {
    Data(Document),
}
