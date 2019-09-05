use crate::*;
use pest::Parser;
use pest_derive::*;
use std::collections::BTreeMap;

#[derive(Parser)]
#[grammar = "templar.pest"]
struct TemplarParser;

type Tree<'a> = (Vec<Node>, Option<&'a str>, &'a Templar);

macro_rules! parse_token {
    (expression : $rule:expr => $tree:expr) => {
        parse_token!(push: $tree.2.parse_match($rule.into_inner())? => $tree)
    };
    (content : $rule:expr => $tree:expr) => {
        $tree.0.push(Node::Data($rule.as_str().into()))
    };
    (template : $rule:expr => $tree:expr) => {
        $tree.0.push($tree.2.parse_match($rule.into_inner())?)
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
    (args : $rule:expr => $tree:expr) => {
        parse_token!(push: $tree.2.parse_match($rule.into_inner())? => $tree)
    };
    (op : $name:literal => $tree:expr) => {
        $tree.1 = Some($name)
    };
    (ident : $rule:expr) => {
        $rule.as_str().into()
    };
    (number : $rule:expr => $tree:expr) => {
        parse_token!(push: Node::Data(
            $rule
                .as_str()
                .parse::<i64>()
                .map_err(|e| TemplarError::ParseFailure(format!("{}", e)))?
                .into(),
        ) => $tree)
    };
    (array : $rule:expr => $tree:expr) => {
        parse_token!(push: {
            let mut tree: Tree = (vec![], None, $tree.2);
            for pair in $rule.into_inner() {
                match pair.as_rule() {
                    Rule::expression_cap => tree.0.push(tree.2.parse_match(pair.into_inner())?),
                    _ => parse_token!(!pair),
                }
            }
            Node::Array(tree.0.into())
        } => $tree)
    };
    (map : $rule:expr => $tree:expr) => {
        parse_token!(push: {
            let mut res = BTreeMap::new();
            let mut last_lit = Document::Unit;
            for pair in $rule.into_inner() {
                match pair.as_rule() {
                    Rule::literal_cap => last_lit = $tree.2.parse_match(pair.into_inner())?.into_document()?,
                    Rule::expression_cap => {
                        res.insert(last_lit, $tree.2.parse_match(pair.into_inner())?);
                        last_lit = Document::Unit;
                    },
                    _ => parse_token!(!pair),
                }
            }
            Node::Map(res)
        } => $tree)
    };
    (fn : $rule:expr => $tree:expr) => {
        parse_token!(push: {
            let mut tree: Tree = (vec![], None, $tree.2);
            let mut name = String::new();
            for pair in $rule.into_inner() {
                match pair.as_rule() {
                    Rule::ident => name = parse_token!(ident: pair),
                    Rule::args => parse_token!(args: pair => tree),
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
    ("if" : $rule:expr => $tree:expr) => {{
        let mut condition = Node::Empty();
        let mut contents = Node::Empty();
        for pair in $rule.into_inner() {
            match pair.as_rule() {
                Rule::expression_cap => condition = $tree.2.parse_match(pair.into_inner())?,
                Rule::template => contents = $tree.2.parse_match(pair.into_inner())?,
                Rule::tag_start_comment
                | Rule::tag_end_comment
                | Rule::tag_start_expr
                | Rule::tag_start_control
                | Rule::tag_end_control
                | Rule::tag_end_expr => {}
                _ => parse_token!(!pair),
            }
        }
        $tree.0.push(Node::Filter(Box::new((
            condition.into(),
            $tree.2.filters
                .get("then")
                .ok_or_else(|| TemplarError::FilterNotFound("then".into()))?
                .clone(),
            contents.into(),
        ))));
    }};
    (filter : $rule:expr => $tree:expr) => {{
        let mut tree: Tree = (vec![], None, $tree.2);
        let mut name = String::new();
        for pair in $rule.into_inner() {
            match pair.as_rule() {
                Rule::ident => name = parse_token!(ident: pair),
                Rule::args => parse_token!(args: pair => tree),
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
        let result: Node = self
            .parse_match(
                TemplarParser::parse(Rule::template_root, input)
                    .map_err(|e| TemplarError::ParseFailure(format!("{}", e)))?,
            )?
            .make_vector()
            .into();
        Ok(result.into())
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
                Rule::expression_cap => parse_token!(expression: pair => tree),
                Rule::template => parse_token!(template: pair => tree),
                Rule::template_block => parse_token!(template: pair => tree),
                Rule::content => parse_token!(content: pair => tree),
                Rule::ctrl_block_if => parse_token!("if": pair => tree),
                Rule::number_lit => parse_token!(number: pair => tree),
                Rule::true_lit => parse_token!(true => tree),
                Rule::false_lit => parse_token!(false => tree),
                Rule::string_lit => parse_token!(str ' ': pair => tree),
                Rule::null_lit => parse_token!(nil => tree),
                Rule::value => parse_token!(value: pair => tree),
                Rule::filter => parse_token!(filter: pair => tree),
                Rule::function => parse_token!(fn: pair => tree),
                Rule::array_lit => parse_token!(array: pair => tree),
                Rule::map_lit => parse_token!(map: pair => tree),
                Rule::op_add => parse_token!(op: "add" => tree),
                Rule::op_sub => parse_token!(op: "subtract" => tree),
                Rule::op_div => parse_token!(op: "divide" => tree),
                Rule::op_mlt => parse_token!(op: "multiply" => tree),
                Rule::op_mod => parse_token!(op: "mod" => tree),
                Rule::op_and => parse_token!(op: "and" => tree),
                Rule::op_or => parse_token!(op: "or" => tree),
                Rule::op_eq => parse_token!(op: "equals" => tree),
                Rule::op_ne => parse_token!(op: "not_equals" => tree),
                Rule::op_gt => parse_token!(op: "greater_than" => tree),
                Rule::op_gte => parse_token!(op: "greater_than_equals" => tree),
                Rule::op_lt => parse_token!(op: "less_than" => tree),
                Rule::op_lte => parse_token!(op: "less_than_equals" => tree),
                Rule::op_cat => parse_token!(op: "concat" => tree),
                Rule::EOI => break,
                _ => parse_token!(!pair),
            }
        }
        println!("{:?}", tree.0);
        Ok(tree.0.into())
    }
}
