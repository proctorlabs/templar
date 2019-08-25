use crate::*;
use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "templar.pest"]
struct TemplarParser;

impl Templar {
    pub(crate) fn parse_template(&self, input: &str) -> Result<Template> {
        let tokens = TemplarParser::parse(Rule::template, input)
            .map_err(|e| TemplarError::ParseFailure(format!("{}", e)))?;
        let metadata = self.parse_raw(tokens)?;
        let result = Node::Expr(metadata);
        Ok(result.into())
    }

    fn parse_raw(&self, pairs: pest::iterators::Pairs<'_, Rule>) -> Result<Vec<Node>> {
        let mut result = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::raw_block => result.push(Node::Data(pair.as_str().into())),
                Rule::template_block => result.push(self.parse_expr(pair.into_inner())?),
                Rule::EOI => return Ok(result),
                _ => {
                    return Err(TemplarError::ParseFailure(format!(
                        "Unexpected rule: {}",
                        pair
                    )))
                }
            }
        }
        Ok(result)
    }

    fn parse_expr(&self, pairs: pest::iterators::Pairs<'_, Rule>) -> Result<Node> {
        let mut result = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::number_lit => result.push(Node::Data(
                    pair.as_str()
                        .parse::<i64>()
                        .map_err(|e| TemplarError::ParseFailure(format!("{}", e)))?
                        .into(),
                )),
                Rule::true_lit => result.push(Node::Data(true.into())),
                Rule::false_lit => result.push(Node::Data(false.into())),
                Rule::str_lit => result.push(Node::Data(
                    pair.into_inner().as_str().replace("\\'", "'").into(),
                )),
                Rule::null_lit => result.push(Node::Data(Document::Unit)),
                Rule::value => result.push(self.parse_value(pair.into_inner())?),
                Rule::filter => {
                    result = vec![self.parse_filter(Node::Expr(result), pair.into_inner())?]
                }
                Rule::function => result.push(self.parse_function(pair.into_inner())?),
                _ => {
                    return Err(TemplarError::ParseFailure(format!(
                        "Unexpected rule: {}",
                        pair
                    )))
                }
            }
        }
        Ok(match result.len() {
            1 => result.pop().unwrap(),
            0 => Node::Empty(),
            _ => Node::Expr(result),
        })
    }

    fn parse_function(&self, pairs: pest::iterators::Pairs<'_, Rule>) -> Result<Node> {
        let mut name = String::new();
        let mut args = Node::Empty();
        for pair in pairs {
            match pair.as_rule() {
                Rule::ident => name = pair.as_str().into(),
                Rule::parens_block => args = self.parse_expr(pair.into_inner())?,
                _ => {
                    return Err(TemplarError::ParseFailure(format!(
                        "Unexpected rule: {}",
                        pair
                    )))
                }
            }
        }
        Ok(Node::Method(Box::new((
            self.functions
                .get(&name)
                .ok_or_else(|| TemplarError::FunctionNotFound(name))?
                .clone(),
            args,
        ))))
    }

    fn parse_filter(&self, left: Node, pairs: pest::iterators::Pairs<'_, Rule>) -> Result<Node> {
        let mut name = String::new();
        let mut args = Node::Empty();
        for pair in pairs {
            match pair.as_rule() {
                Rule::ident => name = pair.as_str().into(),
                Rule::parens_block => args = self.parse_expr(pair.into_inner())?,
                _ => {
                    return Err(TemplarError::ParseFailure(format!(
                        "Unexpected rule: {}",
                        pair
                    )))
                }
            }
        }
        Ok(Node::Filter(Box::new((
            left,
            self.filters
                .get(&name)
                .ok_or_else(|| TemplarError::FilterNotFound(name))?
                .clone(),
            args,
        ))))
    }

    fn parse_value(&self, pairs: pest::iterators::Pairs<'_, Rule>) -> Result<Node> {
        let mut result = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::ident => result.push(
                    pair.as_str()
                        .parse::<String>()
                        .map_err(|e| TemplarError::ParseFailure(format!("{}", e)))?,
                ),
                _ => {
                    return Err(TemplarError::ParseFailure(format!(
                        "Unexpected rule: {}",
                        pair
                    )))
                }
            }
        }
        Ok(Node::Value(result))
    }
}
