use crate::*;
use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "templar.pest"]
struct TemplarParser;

type Tree<'a> = (Vec<Node>, Option<&'a str>, &'a Templar);

macro_rules! parse_token {
    (number : $rule:expr => $tree:expr) => {
        parse_token!(push: Node::Data(
            $rule
                .as_str()
                .parse::<i64>()
                .map_err(|e| TemplarError::ParseFailure(format!("{}", e)))?
                .into(),
        ) => $tree)
    };
    (true => $tree:expr) => {
        parse_token!(push: Node::Data(true.into()) => $tree)
    };
    (false => $tree:expr) => {
        parse_token!(push: Node::Data(false.into()) => $tree)
    };
    (str ' ' : $rule:expr => $tree:expr) => {
        parse_token!(push: Node::Data($rule.into_inner().as_str().replace("\\'", "'").into()) => $tree)
    };
    (nil => $tree:expr) => {
        parse_token!(push: Node::Data(Document::Unit) => $tree)
    };

    (raw : $rule:expr => $tree:expr) => {
        parse_token!(push: Node::Data($rule.as_str().into()) => $tree)
    };
    (parens : $rule:expr => $tree:expr) => {
        parse_token!(push: $tree.2.parse_match($rule.into_inner())? => $tree)
    };
    (fn : $rule:expr => $tree:expr) => {
        parse_token!(push: {
            let mut tree: Tree = (vec![], None, $tree.2);
            let mut name = String::new();
            for pair in $rule.into_inner() {
                match pair.as_rule() {
                    Rule::ident => name = parse_token!(ident: pair),
                    Rule::parens_block => parse_token!(parens: pair => tree),
                    _ => parse_token!(!pair),
                }
            }
            Node::Method(Box::new((
                tree.2.functions
                    .get(&name)
                    .ok_or_else(|| TemplarError::FunctionNotFound(name.into()))?
                    .clone(),
                tree.0.into(),
            )))
        } => $tree);
    };
    (filter : $rule:expr => $tree:expr) => {{
        let mut tree: Tree = (vec![], None, $tree.2);
        let mut name = String::new();
        for pair in $rule.into_inner() {
            match pair.as_rule() {
                Rule::ident => name = parse_token!(ident: pair),
                Rule::parens_block => parse_token!(parens: pair => tree),
                _ => parse_token!(!pair),
            }
        }
        $tree.0 = vec![Node::Filter(Box::new((
            $tree.0.into(),
            $tree.2.filters
                .get(&name)
                .ok_or_else(|| TemplarError::FilterNotFound(name.into()))?
                .clone(),
            tree.0.into(),
        )))]
    }};
    (value : $rule:expr => $tree:expr) => {
        parse_token!(push: {
            let mut result = vec![];
            for pair in $rule.into_inner() {
                match pair.as_rule() {
                    Rule::ident => result.push(parse_token!(ident: pair)),
                    Rule::value_key => result.push(parse_token!(value_key: pair)),
                    _ => parse_token!(!pair),
                }
            }
            Node::Value(result)
        } => $tree)
    };
    (ident : $rule:expr) => {
        $rule.as_str().into()
    };
    (value_key : $rule:expr) => {
        $rule
            .into_inner()
            .next()
            .unwrap()
            .into_inner()
            .as_str()
            .replace("\\'", "'")
    };
    (push : $current:expr => $tree:expr) => {{
        if let Some(op) = $tree.1 {
            $tree.0 = vec![Node::Filter(Box::new((
                $tree.0.into(),
                $tree.2.filters
                    .get(op)
                    .ok_or_else(|| TemplarError::FilterNotFound(op.to_string()))?
                    .clone(),
                $current,
            )))];
            $tree.1 = None;
        } else {
            $tree.0.push($current);
        }
    }};
    (! $rule:expr) => {{
        return Err(TemplarError::ParseFailure(format!(
            "Unexpected rule while parsing expression: {}",
            $rule
        )));
    }};
}

impl Templar {
    #[inline]
    pub fn parse_template(&self, input: &str) -> Result<Template> {
        Ok(self
            .parse_match(
                TemplarParser::parse(Rule::template, input)
                    .map_err(|e| TemplarError::ParseFailure(format!("{}", e)))?,
            )?
            .into())
    }

    #[inline]
    pub fn parse_expression(&self, input: &str) -> Result<Template> {
        Ok(self
            .parse_match(
                TemplarParser::parse(Rule::expression, input.trim())
                    .map_err(|e| TemplarError::ParseFailure(format!("{}", e)))?,
            )?
            .into())
    }

    fn parse_match(&self, pairs: pest::iterators::Pairs<'_, Rule>) -> Result<Node> {
        let mut tree: Tree = (vec![], None, self);
        for pair in pairs {
            match pair.as_rule() {
                Rule::template_block => tree.0.push(self.parse_match(pair.into_inner())?),
                Rule::raw_block => parse_token!(raw: pair => tree),
                Rule::number_lit => parse_token!(number: pair => tree),
                Rule::true_lit => parse_token!(true => tree),
                Rule::false_lit => parse_token!(false => tree),
                Rule::str_lit => parse_token!(str' ': pair => tree),
                Rule::null_lit => parse_token!(nil => tree),
                Rule::value => parse_token!(value: pair => tree),
                Rule::filter => parse_token!(filter: pair => tree),
                Rule::function => parse_token!(fn: pair => tree),
                Rule::op_add => tree.1 = Some("add"),
                Rule::op_sub => tree.1 = Some("subtract"),
                Rule::op_div => tree.1 = Some("divide"),
                Rule::op_mlt => tree.1 = Some("multiply"),
                Rule::op_mod => tree.1 = Some("mod"),
                Rule::op_and => tree.1 = Some("and"),
                Rule::op_or => tree.1 = Some("or"),
                Rule::op_eq => tree.1 = Some("equals"),
                Rule::op_ne => tree.1 = Some("not_equals"),
                Rule::op_gt => tree.1 = Some("greater_than"),
                Rule::op_gte => tree.1 = Some("greater_than_equals"),
                Rule::op_lt => tree.1 = Some("less_than"),
                Rule::op_lte => tree.1 = Some("less_than_equals"),
                Rule::EOI => break,
                _ => parse_token!(!pair),
            }
        }
        Ok(tree.0.into())
    }
}
