use crate::*;
use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "templar.pest"]
struct TemplarParser;

pub fn parse_template(input: &str) -> Result<Vec<Node>, String> {
    let tokens = TemplarParser::parse(Rule::template, input).map_err(|e| format!("{}", e))?;
    let metadata = parse_tokens(tokens)?;
    Ok(metadata)
}

fn parse_tokens(pairs: pest::iterators::Pairs<'_, Rule>) -> std::result::Result<Vec<Node>, String> {
    let mut result = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::raw_block => result.push(Node::Data(pair.as_str().into())),
            Rule::number_lit => result.push(Node::Data(
                pair.as_str()
                    .parse::<i64>()
                    .map_err(|e| format!("{}", e))?
                    .into(),
            )),
            Rule::true_lit => result.push(Node::Data(true.into())),
            Rule::false_lit => result.push(Node::Data(false.into())),
            Rule::string_lit => result.push(Node::Data(
                pair.into_inner().as_str().replace("\\'", "'").into(),
            )),
            Rule::method => return Err("Method not supported".into()),
            Rule::EOI => return Ok(result),
            _ => return Err("".into()),
        }
    }
    Ok(result)
}
