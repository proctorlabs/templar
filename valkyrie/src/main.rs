use derive_more::From;
use pest::Parser;
use pest_derive::*;

const TEST: &str = include_str!("test.valkyrie");

#[derive(Parser)]
#[grammar = "runfile.pest"]
struct BuildfileParser;

#[derive(Debug, Clone, From)]
pub enum Node {
    Task(Task),
    SetLiteral(SetLiteral),
    SetTemplate(SetTemplate),
}

#[derive(Default, Debug, Clone)]
pub struct Task {
    name: String,
    args: Vec<String>,
    content: String,
}

#[derive(Default, Debug, Clone)]
pub struct SetLiteral {
    name: String,
    val: String,
}

#[derive(Default, Debug, Clone)]
pub struct SetTemplate {
    name: String,
    val: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pairs = BuildfileParser::parse(Rule::file, TEST)?;
    let parsed_file = parse_rules(pairs)?;
    println!("Result: {:?}", parsed_file);
    Ok(())
}

type Tokens<'a> = pest::iterators::Pairs<'a, Rule>;

fn parse_rules(tokens: Tokens) -> Result<Vec<Node>, Box<dyn std::error::Error>> {
    let mut results: Vec<Node> = vec![];
    for token in tokens {
        match token.as_rule() {
            Rule::task_block => {
                results.push(parse_task(token.into_inner())?.into());
            }
            Rule::set_value => {
                results.push(parse_set_literal(token.into_inner())?.into());
            }
            Rule::set_template => {
                results.push(parse_set_template(token.into_inner())?.into());
            }
            Rule::EOI => break,
            def => println!("Unexpected token: {:?}", def),
        }
    }
    Ok(results)
}

fn parse_set_template(tokens: Tokens) -> Result<SetTemplate, Box<dyn std::error::Error>> {
    let mut setter: SetTemplate = Default::default();
    for token in tokens {
        match token.as_rule() {
            Rule::ident => {
                setter.name.push_str(token.as_str());
            }
            _ => {
                setter.val.push_str(token.as_str());
            }
        }
    }
    Ok(setter)
}

fn parse_set_literal(tokens: Tokens) -> Result<SetLiteral, Box<dyn std::error::Error>> {
    let mut setter: SetLiteral = Default::default();
    for token in tokens {
        match token.as_rule() {
            Rule::ident => {
                setter.name.push_str(token.as_str());
            }
            _ => {
                setter.val.push_str(token.as_str());
            }
        }
    }
    Ok(setter)
}

fn parse_task(tokens: Tokens) -> Result<Task, Box<dyn std::error::Error>> {
    let mut task: Task = Default::default();
    for token in tokens {
        match token.as_rule() {
            Rule::ident => {
                task.name.push_str(token.as_str());
            }
            Rule::expression_cap => task.args.push(token.as_str().into()),
            Rule::block_content => {
                task.content.push_str(token.as_str());
            }
            def => println!("Unmatched task token: {:?}", def),
        }
    }
    Ok(task)
}
