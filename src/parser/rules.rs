use super::*;

macro_rules! parse_token {
    (expression : $rule:expr => $tree:expr) => {
        $tree.push($tree.templar.parse_match($rule.into_inner())?)?;
    };
    (content : $rule:expr => $tree:expr) => {
        $tree.push(Node::Data($rule.as_str().into()))?
    };
    (template : $rule:expr => $tree:expr) => {
        $tree.push($tree.templar.parse_match($rule.into_inner())?.set_operation(Operations::Concat))?
    };
    (for : $rule:expr => $tree:expr) => {{
        $tree.set_op(Operations::ForLoop)?;
        $tree.finish_op()?;
        return Ok($tree.into_node()?.into_scope())
    }};
    (scope : $rule:expr => $tree:expr) => {
        $tree.push($tree.templar.parse_match($rule.into_inner())?.set_operation(Operations::Concat).into_scope())?
    };
    (true => $tree:expr) => {
        $tree.push(Node::Data(true.into()))?;
    };
    (false => $tree:expr) => {
        $tree.push(Node::Data(false.into()))?;
    };
    (str ' ' : $rule:expr => $tree:expr) => {{
        $tree.push({
            let mut result = String::new();
            for pair in $rule.clone().into_inner() {
                match pair.as_rule() {
                    Rule::str_single => result.push_str(&$rule.as_str().replace("\\'", "'")),
                    Rule::str_double => result.push_str(&$rule.as_str().replace("\\\"", "\"")),
                    Rule::str_backtick => result.push_str(&$rule.as_str().replace("\\`", "`")),
                    _ => parse_token!(!pair),
                }
            }
            result.truncate(result.len() - 1);
            result.remove(0);
            Node::Data(result.into())
        })?;
    }};
    (nil => $tree:expr) => {
        $tree.push(Node::Data(Document::Unit.into()))?;
    };
    (args : $rule:expr => $tree:expr) => {
        $tree.push($tree.templar.parse_match($rule.into_inner())?)?;
    };
    (op : $name:ident => $tree:expr) => {
        $tree.set_op(Operations::$name)?
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
                    Rule::expression_cap => { res.insert(last_lit.take(), $tree.templar.parse_match(pair.into_inner())?); },
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
    pub(crate) fn parse_match(&self, pairs: pest::iterators::Pairs<'_, Rule>) -> Result<Node> {
        let mut tree = ParseTree::new(self);
        for pair in pairs {
            match pair.as_rule() {
                Rule::template_inner
                | Rule::template_block
                | Rule::ctrl_block_if
                | Rule::ctrl_block_else
                | Rule::ctrl_block_loop => parse_token!(template: pair => tree),
                Rule::ctrl_block_end_loop => parse_token!(for: pair => tree),
                Rule::ctrl_block_scope => parse_token!(scope: pair => tree),
                Rule::expression_cap => parse_token!(expression: pair => tree),
                Rule::content => parse_token!(content: pair => tree),
                Rule::filter => parse_token!(filter: pair => tree),
                Rule::function => parse_token!(fn: pair => tree),
                Rule::value => parse_token!(value: pair => tree),
                Rule::number_lit => parse_token!(number: pair => tree),
                Rule::true_lit => parse_token!(true => tree),
                Rule::false_lit => parse_token!(false => tree),
                Rule::string_lit => parse_token!(str ' ': pair => tree),
                Rule::null_lit => parse_token!(nil => tree),
                Rule::array_lit => parse_token!(array: pair => tree),
                Rule::map_lit => parse_token!(map: pair => tree),
                Rule::kw_if => parse_token!(op: IfThen => tree),
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
                Rule::op_set => parse_token!(op: Set => tree),
                Rule::EOI | Rule::ctrl_block_end_if => tree.finish_op()?,
                _ => parse_token!(!pair),
            }
        }
        Ok(tree.into_node()?)
    }
}
