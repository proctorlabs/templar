use crate::*;
use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "templar.pest"]
struct TemplarParser;

macro_rules! parse_tokens {
    (@inner $zelf:ident : $value:expr => $result:ident, ( expression $( $rest:tt )* ) -> ( $( $matches:tt )* )) => {
        parse_tokens!(@inner $zelf : $value => $result, ( $( $rest )* ) -> (
            Rule::number_lit => $result.push(Node::Data(
                $value.as_str()
                    .parse::<i64>()
                    .map_err(|e| TemplarError::ParseFailure(format!("{}", e)))?
                    .into(),
            )),
            Rule::true_lit => $result.push(Node::Data(true.into())),
            Rule::false_lit => $result.push(Node::Data(false.into())),
            Rule::str_lit => $result.push(Node::Data(
                $value.into_inner().as_str().replace("\\'", "'").into(),
            )),
            Rule::null_lit => $result.push(Node::Data(Document::Unit)),
            Rule::value => $result.push($zelf.parse_value($value.into_inner())?),
            Rule::filter => $result = vec![$zelf.parse_filter($result.into(), $value.into_inner())?],
            Rule::op_right => $result = vec![$zelf.parse_op($result.into(), $value.into_inner())?],
            Rule::function => $result.push($zelf.parse_function($value.into_inner())?),
            $( $matches )*
        ))
    };

    (@inner $zelf:ident : $value:expr => $result:ident, ( (op: $op_name:ident) $( $rest:tt )* ) -> ( $( $matches:tt )* )) => {
        parse_tokens!(@inner $zelf : $value => $result, ( $( $rest )* ) -> (
            Rule::op_add => $op_name = "add",
            Rule::op_sub => $op_name = "subtract",
            Rule::op_div => $op_name = "divide",
            Rule::op_mlt => $op_name = "multiply",
            Rule::op_mod => $op_name = "mod",
            $( $matches )*
        ))
    };

    // End
    (@inner $zelf:ident : $value:expr => $result:ident, () -> ($($tree:tt)*) ) => {
        {
            match $value.as_rule() {
                $( $tree )*
                _ => return Err(TemplarError::ParseFailure(
                        format!("Unexpected rule: {}", $value)
                    ))
            }
        }
    };
    // Entry
    ( $zelf:ident : $value:expr => $result:ident, $($rest:tt)* ) => {
        parse_tokens!(@inner $zelf : $value => $result, ($($rest)*) -> ())
    };
}

impl Templar {
    pub(crate) fn parse_text(&self, input: &str, template: bool) -> Result<Template> {
        Ok(if template {
            let tokens = TemplarParser::parse(Rule::template, input)
                .map_err(|e| TemplarError::ParseFailure(format!("{}", e)))?;
            self.parse_raw(tokens)?.into()
        } else {
            let tokens = TemplarParser::parse(Rule::expression, input.trim())
                .map_err(|e| TemplarError::ParseFailure(format!("{}", e)))?;
            self.parse_expr(tokens)?
        }
        .into())
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
            parse_tokens!(self: pair => result, expression);
        }
        Ok(result.into())
    }

    fn parse_op(&self, left: Node, pairs: pest::iterators::Pairs<'_, Rule>) -> Result<Node> {
        let mut right = vec![];
        let mut op_name = "";
        for pair in pairs {
            parse_tokens!(self: pair => right, expression (op: op_name));
        }
        Ok(Node::Filter(Box::new((
            left,
            self.filters
                .get(op_name)
                .ok_or_else(|| TemplarError::FilterNotFound(op_name.into()))?
                .clone(),
            right.into(),
        ))))
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
                Rule::value_key => result.push(
                    pair.into_inner()
                        .next()
                        .unwrap()
                        .into_inner()
                        .as_str()
                        .replace("\\'", "'"),
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
