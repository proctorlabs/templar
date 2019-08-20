#[cfg(test)]
mod test;

mod nodes;
mod parser;

pub use nodes::Node;
pub use unstructured::Document;

pub struct Templar;

impl Templar {
    pub fn parse_str(val: &str) -> Result<Vec<Node>, String> {
        parser::parse_template(val)
    }
}
