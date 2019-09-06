use crate::nodes::*;
use crate::*;
use pest::Parser;
use pest_derive::*;
use std::collections::BTreeMap;
use std::mem::replace;
use tree::ParseTree;

#[derive(Parser)]
#[grammar = "templar.pest"]
struct TemplarParser;

mod tree {
    use super::*;

    pub struct ParseTree<'a> {
        pub templar: &'a Templar,
        tree: Vec<Node>,
        current_op: Option<Operations>,
    }

    impl<'a> ParseTree<'a> {
        pub fn new(templar: &'a Templar) -> ParseTree<'a> {
            ParseTree {
                tree: vec![],
                current_op: None,
                templar,
            }
        }

        pub fn set_op(&mut self, op: Operations) {
            self.current_op = Some(op);
        }

        pub fn push(&mut self, node: Node) -> Result<()> {
            if self.current_op.is_some() {
                let op = self.current_op.take().unwrap();
                let mut tree = replace(&mut self.tree, vec![]);
                tree.push(node);
                let node = Node::Operation(op.build(tree));
                self.tree.push(node);
                self.current_op = None;
            } else {
                self.tree.push(node);
            }
            Ok(())
        }

        pub fn filter(&mut self, filter: &str, args: Node) -> Result<()> {
            let tree = replace(&mut self.tree, vec![]);
            self.tree = vec![Node::Filter(Box::new((
                tree.into(),
                self.templar
                    .filters
                    .get(filter)
                    .ok_or_else(|| TemplarError::FilterNotFound(filter.into()))?
                    .clone(),
                args,
            )))];
            Ok(())
        }

        pub fn into_node(self) -> Result<Node> {
            if self.current_op != None {
                return Err(TemplarError::ParseFailure(
                    "Operation not completed!".into(),
                ));
            }
            Ok(self.tree.into())
        }

        pub fn into_nodes(self) -> Result<Vec<Node>> {
            Ok(self.tree)
        }
    }
}

macro_rules! parse_token {
    (expression : $rule:expr => $tree:expr) => {
        $tree.push($tree.templar.parse_match($rule.into_inner())?)?;
    };
    (content : $rule:expr => $tree:expr) => {
        $tree.push(Node::Data($rule.as_str().into()))?
    };
    (template : $rule:expr => $tree:expr) => {
        $tree.push($tree.templar.parse_match($rule.into_inner())?)?
    };
    (true => $tree:expr) => {
        $tree.push(Node::Data(true.into()))?;
    };
    (false => $tree:expr) => {
        $tree.push(Node::Data(false.into()))?;
    };
    (str ' ' : $rule:expr => $tree:expr) => {
        $tree.push(Node::Data($rule.into_inner().as_str().replace("\\'", "'").into()))?;
    };
    (nil => $tree:expr) => {
        $tree.push(Node::Data(Document::Unit))?;
    };
    (args : $rule:expr => $tree:expr) => {
        $tree.push($tree.templar.parse_match($rule.into_inner())?)?;
    };
    (op : $name:ident => $tree:expr) => {
        $tree.set_op(Operations::$name)
    };
    (ident : $rule:expr) => {
        $rule.as_str().into()
    };
    (number : $rule:expr => $tree:expr) => {
        $tree.push(Node::Data(
            $rule
                .as_str()
                .parse::<i64>()
                .map_err(|e| TemplarError::ParseFailure(format!("{}", e)))?
                .into(),
        ))?;
    };
    (array : $rule:expr => $tree:expr) => {
        $tree.push({
            let mut tree = ParseTree::new($tree.templar);
            for pair in $rule.into_inner() {
                match pair.as_rule() {
                    Rule::expression_cap => tree.push(tree.templar.parse_match(pair.into_inner())?)?,
                    _ => parse_token!(!pair),
                }
            }
            Node::Array(tree.into_nodes()?)
        })?;
    };
    (map : $rule:expr => $tree:expr) => {
        $tree.push({
            let mut res = BTreeMap::new();
            let mut last_lit = Document::Unit;
            for pair in $rule.into_inner() {
                match pair.as_rule() {
                    Rule::literal_cap => last_lit = $tree.templar.parse_match(pair.into_inner())?.into_document()?,
                    Rule::expression_cap => {
                        res.insert(last_lit, $tree.templar.parse_match(pair.into_inner())?);
                        last_lit = Document::Unit;
                    },
                    _ => parse_token!(!pair),
                }
            }
            Node::Map(res)
        })?;
    };
    (fn : $rule:expr => $tree:expr) => {
        $tree.push({
            let mut tree = ParseTree::new($tree.templar);
            let mut name = String::new();
            for pair in $rule.into_inner() {
                match pair.as_rule() {
                    Rule::ident => name = parse_token!(ident: pair),
                    Rule::args => parse_token!(args: pair => tree),
                    _ => parse_token!(!pair),
                }
            }
            Node::Function(Box::new((
                tree.templar.functions
                    .get(&name)
                    .ok_or_else(|| TemplarError::FunctionNotFound(name.into()))?
                    .clone(),
                tree.into_node()?,
            )))
        })?;
    };
    (control "if" : $rule:expr => $tree:expr) => {{
        let mut condition = Node::Empty();
        let mut contents = Node::Empty();
        for pair in $rule.into_inner() {
            match pair.as_rule() {
                Rule::expression_cap => condition = $tree.templar.parse_match(pair.into_inner())?,
                Rule::template => contents = $tree.templar.parse_match(pair.into_inner())?,
                _ => parse_token!(!pair),
            }
        }
        $tree.push(Node::Filter(Box::new((
            condition.into(),
            $tree.templar.filters
                .get("then")
                .ok_or_else(|| TemplarError::FilterNotFound("then".into()))?
                .clone(),
            contents.into(),
        ))))?;
    }};
    (filter : $rule:expr => $tree:expr) => {{
        let mut tree = ParseTree::new($tree.templar);
        let mut name = String::new();
        for pair in $rule.into_inner() {
            match pair.as_rule() {
                Rule::ident => name = parse_token!(ident: pair),
                Rule::args => parse_token!(args: pair => tree),
                _ => parse_token!(!pair),
            }
        }
        $tree.filter(&name, tree.into_node()?)?;
    }};
    (value : $rule:expr => $tree:expr) => {
        $tree.push({
            let mut result = vec![];
            for pair in $rule.into_inner() {
                match pair.as_rule() {
                    Rule::ident => result.push(parse_token!(ident: pair)),
                    Rule::value_key => result.push(parse_token!(value_key: pair)),
                    _ => parse_token!(!pair),
                }
            }
            Node::Value(result)
        })?
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
        let mut tree = ParseTree::new(self);
        for pair in pairs {
            match pair.as_rule() {
                Rule::expression_cap => parse_token!(expression: pair => tree),
                Rule::template => parse_token!(template: pair => tree),
                Rule::template_block => parse_token!(template: pair => tree),
                Rule::content => parse_token!(content: pair => tree),
                Rule::ctrl_block_if => parse_token!(control "if": pair => tree),
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
                Rule::op_add => parse_token!(op: Add => tree),
                Rule::op_sub => parse_token!(op: Subtract => tree),
                Rule::op_div => parse_token!(op: Divide => tree),
                Rule::op_mlt => parse_token!(op: Multiply => tree),
                Rule::op_mod => parse_token!(op: Modulus => tree),
                Rule::op_and => parse_token!(op: And => tree),
                Rule::op_or => parse_token!(op: Or => tree),
                Rule::op_eq => parse_token!(op: Equals => tree),
                Rule::op_ne => parse_token!(op: NotEquals => tree),
                Rule::op_gt => parse_token!(op: GreaterThan => tree),
                Rule::op_gte => parse_token!(op: GreaterThanEquals => tree),
                Rule::op_lt => parse_token!(op: LessThan => tree),
                Rule::op_lte => parse_token!(op: LessThanEquals => tree),
                Rule::op_cat => parse_token!(op: Concat => tree),
                Rule::EOI => break,
                _ => parse_token!(!pair),
            }
        }
        Ok(tree.into_node()?)
    }
}
