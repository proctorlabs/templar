use crate::*;
use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "templar.pest"]
struct TemplarParser;

type Stack<'a> = (Vec<Node>, Option<&'a str>, &'a Templar);

macro_rules! parse_token {
    (number : $rule:expr => $stack:expr) => {
        parse_token!(push: Node::Data(
            $rule
                .as_str()
                .parse::<i64>()
                .map_err(|e| TemplarError::ParseFailure(format!("{}", e)))?
                .into(),
        ) => $stack)
    };
    (true => $stack:expr) => {
        parse_token!(push: Node::Data(true.into()) => $stack)
    };
    (false => $stack:expr) => {
        parse_token!(push: Node::Data(false.into()) => $stack)
    };
    (str ' ' : $rule:expr => $stack:expr) => {
        parse_token!(push: Node::Data($rule.into_inner().as_str().replace("\\'", "'").into()) => $stack)
    };
    (nil => $stack:expr) => {
        parse_token!(push: Node::Data(Document::Unit) => $stack)
    };

    (raw : $rule:expr => $stack:expr) => {
        parse_token!(push: Node::Data($rule.as_str().into()) => $stack)
    };
    (parens : $rule:expr => $stack:expr) => {
        parse_token!(push: $stack.2.parse_match($rule.into_inner())? => $stack)
    };
    (fn : $rule:expr => $stack:expr) => {
        parse_token!(push: {
            let mut stack: Stack = (vec![], None, $stack.2);
            let mut name = String::new();
            for pair in $rule.into_inner() {
                match pair.as_rule() {
                    Rule::ident => name = parse_token!(ident: pair),
                    Rule::parens_block => parse_token!(parens: pair => stack),
                    _ => parse_token!(!pair),
                }
            }
            Node::Method(Box::new((
                stack.2.functions
                    .get(&name)
                    .ok_or_else(|| TemplarError::FunctionNotFound(name.into()))?
                    .clone(),
                stack.0.into(),
            )))
        } => $stack);
    };
    (filter : $rule:expr => $stack:expr) => {{
        let mut stack: Stack = (vec![], None, $stack.2);
        let mut name = String::new();
        for pair in $rule.into_inner() {
            match pair.as_rule() {
                Rule::ident => name = parse_token!(ident: pair),
                Rule::parens_block => parse_token!(parens: pair => stack),
                _ => parse_token!(!pair),
            }
        }
        $stack.0 = vec![Node::Filter(Box::new((
            $stack.0.into(),
            $stack.2.filters
                .get(&name)
                .ok_or_else(|| TemplarError::FilterNotFound(name.into()))?
                .clone(),
            stack.0.into(),
        )))]
    }};
    (value : $rule:expr) => {{
        let mut result = vec![];
        for pair in $rule.into_inner() {
            match pair.as_rule() {
                Rule::ident => result.push(parse_token!(ident: pair)),
                Rule::value_key => result.push(parse_token!(value_key: pair)),
                _ => parse_token!(!pair),
            }
        }
        Node::Value(result)
    }};
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
    (push : $current:expr => $stack:expr) => {{
        if let Some(op) = $stack.1 {
            $stack.0 = vec![Node::Filter(Box::new((
                $stack.0.into(),
                $stack.2.filters
                    .get(op)
                    .ok_or_else(|| TemplarError::FilterNotFound(op.to_string()))?
                    .clone(),
                $current,
            )))];
            $stack.1 = None;
        } else {
            $stack.0.push($current);
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
        let mut stack: Stack = (vec![], None, self);
        for pair in pairs {
            match pair.as_rule() {
                Rule::template_block => stack.0.push(self.parse_match(pair.into_inner())?),
                Rule::raw_block => parse_token!(raw: pair => stack),
                Rule::number_lit => parse_token!(number: pair => stack),
                Rule::true_lit => parse_token!(true => stack),
                Rule::false_lit => parse_token!(false => stack),
                Rule::str_lit => parse_token!(str' ': pair => stack),
                Rule::null_lit => parse_token!(nil => stack),
                Rule::value => stack.0.push(parse_token!(value: pair)),
                Rule::filter => parse_token!(filter: pair => stack),
                Rule::function => parse_token!(fn: pair => stack),
                Rule::op_add => stack.1 = Some("add"),
                Rule::op_sub => stack.1 = Some("subtract"),
                Rule::op_div => stack.1 = Some("divide"),
                Rule::op_mlt => stack.1 = Some("multiply"),
                Rule::op_mod => stack.1 = Some("mod"),
                Rule::op_and => stack.1 = Some("and"),
                Rule::op_or => stack.1 = Some("or"),
                Rule::op_eq => stack.1 = Some("equals"),
                Rule::op_ne => stack.1 = Some("not_equals"),
                Rule::op_gt => stack.1 = Some("greater_than"),
                Rule::op_gte => stack.1 = Some("greater_than_equals"),
                Rule::op_lt => stack.1 = Some("less_than"),
                Rule::op_lte => stack.1 = Some("less_than_equals"),
                Rule::EOI => break,
                _ => parse_token!(!pair),
            }
        }
        Ok(stack.0.into())
    }
}
