use super::*;

pub struct Operation {
    oper: Executors,
    name: &'static str,
    nodes: Vec<Node>,
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Operation{{name: {}, nodes: {:?}}}",
            self.name, self.nodes
        )
    }
}

impl Operation {
    pub(crate) fn exec(&self, ctx: &dyn Context) -> Data {
        Executor::exec(&self.oper, ctx, &self.nodes)
    }
}

macro_rules! map_operations {
    ( $( $executor:ident : $name:ident : $fn_name:ident ; )* ) =>
    {
        #[derive(Debug, PartialEq)]
        pub enum Operations {
            $( $name , )*
        }

        impl Operations {
            pub fn metadata(&self) -> &'static Metadata {
                match self {
                    $( Operations::$name => $executor::metadata(), )*
                }
            }

            pub fn build(&self, nodes: Vec<Node>) -> Operation {
                match self {
                    $( Operations::$name => Operation {
                        name: stringify!($name),
                        oper: $executor::new($fn_name).into(),
                        nodes,
                    }, )*
                }
            }
        }
    };
}

map_operations! {
    PipedExecutor: Add:add;
    PipedExecutor: Subtract:subtract;
    PipedExecutor: Divide:divide;
    PipedExecutor: Multiply:multiply;
    PipedExecutor: Modulus:modulus;
    PipedExecutor: And:and;
    PipedExecutor: Or:or;
    PipedExecutor: Equals:equals;
    PipedExecutor: NotEquals:not_equals;
    PipedExecutor: GreaterThan:greater_than;
    PipedExecutor: LessThan:less_than;
    PipedExecutor: GreaterThanEquals:greater_than_equals;
    PipedExecutor: LessThanEquals:less_than_equals;
    PipedExecutor: Set:set;
    ConditionalExecutor: IfThen:if_then;
    IndeterminateExecutor: Concat:concat;
    LoopExecutor: ForLoop:for_loop;
}

macro_rules! simple_pipe {
    ( $( $pipe_name:ident ( $l:ident , $r:ident ) -> { $( $tail:tt )* } ; )* ) => {
        $(
            fn $pipe_name(ctx: &dyn Context, left: &Node, right: &Node) -> Data {
                match (left.exec(ctx).result(), right.exec(ctx).result()) {
                    (Ok($l), Ok($r)) => Data::from( $( $tail )* ),
                    (Err(e), _) | (_, Err(e)) => Data::from(e),
                }
            }
        )*
    };
}

macro_rules! number {
    ($doc:ident) => {
        match $doc.as_i64() {
            Some(i) => i,
            None => {
                return TemplarError::RenderFailure("Math operations require numeric types".into())
                    .into()
            }
        }
    };
}

simple_pipe! {
    add (l, r) -> { number!(l) + number!(r) };
    subtract(l, r) -> { number!(l) - number!(r) };
    divide(l, r) -> { number!(l) / number!(r) };
    multiply(l, r) -> { number!(l) * number!(r) };
    modulus(l, r) -> { number!(l) % number!(r) };
    and(l, r) -> { l.as_bool().unwrap_or_default() && r.as_bool().unwrap_or_default() };
    or(l, r) -> { l.as_bool().unwrap_or_default() || r.as_bool().unwrap_or_default() };
    equals(l, r) -> { l == r };
    not_equals(l, r) -> { l != r };
    greater_than(l, r) -> { l > r };
    greater_than_equals(l, r) -> { l >= r };
    less_than(l, r) -> { l < r };
    less_than_equals(l, r) -> { l <= r };
}

fn if_then(ctx: &dyn Context, cnd: &Node, p: &Node, n: &Node) -> Data {
    let cnd = cnd.exec(ctx).result();
    match cnd {
        Ok(Document::Bool(true)) => p.exec(ctx),
        Ok(Document::Bool(false)) => n.exec(ctx),
        Err(e) => e.into(),
        _ => TemplarError::RenderFailure("If condition must evaluate to boolean!".into()).into(),
    }
}

fn concat(ctx: &dyn Context, input: &[Node]) -> Data {
    let results: Result<Vec<String>> = input.iter().map(|node| Ok(node.render(ctx)?)).collect();
    match results {
        Ok(result) => result
            .iter()
            .fold(String::new(), |mut acc, s| {
                acc.push_str(&s);
                acc
            })
            .into(),
        Err(err) => err.into(),
    }
}

fn for_loop(ctx: &dyn Context, val_name: &Node, array_path: &Node, exec: &Node) -> Data {
    // Get the result for the value we're iterating over
    let array_exec = array_path.exec(ctx).result();
    if let Err(e) = array_exec {
        return e.into();
    }
    let mut array = array_exec.unwrap();

    // Now we get the path for the scope-local value and iterate whatever the result is
    match (val_name, &mut array) {
        (Node::Value(set_path), Document::Seq(items)) => {
            let mut result = String::new();
            let ref_vec: Vec<Document> = set_path.iter().map(|p| p.into()).collect();
            for item in items.drain(0..) {
                ctx.set_path(&ref_vec, item);
                let res = exec.exec(ctx).result();
                if let Err(e) = res {
                    return e.into();
                }
                result.push_str(&res.unwrap().to_string());
            }
            result.into()
        }
        (Node::Value(set_path), Document::Map(items)) => {
            let mut result = String::new();
            let ref_vec: Vec<Document> = set_path.iter().map(|p| p.into()).collect();
            for (k, v) in items.iter_mut() {
                let mut entry = BTreeMap::new();
                entry.insert("key".into(), k.clone()); //cloning the keys is better than rebalancing the tree
                entry.insert("value".into(), v.take());
                ctx.set_path(&ref_vec, entry.into());
                let res = exec.exec(ctx).result();
                if let Err(e) = res {
                    return e.into();
                }
                result.push_str(&res.unwrap().to_string());
            }
            result.into()
        }
        (Node::Value(ref set_path), _) => {
            let ref_vec: Vec<Document> = set_path.iter().map(|p| p.into()).collect();
            ctx.set_path(&ref_vec, array);
            exec.exec(ctx)
        }
        _ => Data::from(TemplarError::RenderFailure(
            "Unexpected render failure in for loop".into(),
        )),
    }
}

fn set(ctx: &dyn Context, left: &Node, right: &Node) -> Data {
    let val = right.exec(ctx).result();
    match (left, val) {
        (_, Err(e)) => e.into(),
        (Node::Value(path), Ok(ref mut val)) => {
            let ref_vec: Vec<Document> = path.iter().map(|p| p.into()).collect();
            ctx.set_path(&ref_vec, val.take());
            Data::empty()
        }
        (eval, Ok(ref mut val)) => {
            let path = eval.exec(ctx).result();
            if let Err(e) = path {
                return e.into();
            }
            let value = path.unwrap();
            ctx.set_path(&[value], val.take());
            Data::empty()
        }
    }
}
