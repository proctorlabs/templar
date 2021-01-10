use super::*;

pub struct Operation {
    oper: Executors,
    name: String,
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
    pub(crate) fn exec(&self, ctx: &impl Context) -> Data {
        let wrapper = ctx.wrap();
        Executor::exec(&self.oper, &wrapper, &self.nodes)
    }

    pub(crate) fn from_filter(mut name: String, ex: FilterExecutor, nodes: Vec<Node>) -> Self {
        name.shrink_to_fit();
        Operation {
            name,
            oper: Executors::Filter(ex),
            nodes,
        }
    }

    pub(crate) fn from_function(mut name: String, ex: FunctionExecutor, node: Node) -> Self {
        name.shrink_to_fit();
        Operation {
            name,
            oper: Executors::Function(ex),
            nodes: vec![node],
        }
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
                        name: stringify!($name).into(),
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
            fn $pipe_name(ctx: &ContextWrapper, left: &Node, right: &Node) -> Data {
                match (data_unwrap!(left.exec(ctx)), data_unwrap!(right.exec(ctx))) {
                    ($l, $r) => Data::from( $( $tail )* ),
                }
            }
        )*
    };
}

macro_rules! number {
    ($doc:ident) => {
        match $doc.cast::<i64>() {
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
    and(l, r) -> { l.cast::<bool>().unwrap_or_default() && r.cast::<bool>().unwrap_or_default() };
    or(l, r) -> { l.cast::<bool>().unwrap_or_default() || r.cast::<bool>().unwrap_or_default() };
    equals(l, r) -> { l == r };
    not_equals(l, r) -> { l != r };
    greater_than(l, r) -> { l > r };
    greater_than_equals(l, r) -> { l >= r };
    less_than(l, r) -> { l < r };
    less_than_equals(l, r) -> { l <= r };
}

fn if_then(ctx: &ContextWrapper, cnd: &Node, p: &Node, n: &Node) -> Data {
    let cnd = cnd.exec(ctx).into_result();
    match cnd {
        Ok(Document::Bool(true)) => p.exec(ctx),
        Ok(Document::Bool(false)) => n.exec(ctx),
        Err(e) => e.into(),
        _ => TemplarError::RenderFailure("If condition must evaluate to boolean!".into()).into(),
    }
}

fn concat(ctx: &ContextWrapper, input: &[Node]) -> Data {
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

fn for_loop(ctx: &ContextWrapper, val_name: &Node, array_path: &Node, exec: &Node) -> Data {
    // Get the result for the value we're iterating over
    let array_exec = array_path.exec(ctx).into_result();
    if let Err(e) = array_exec {
        return e.into();
    }
    let mut array = array_exec.unwrap();

    // Now we get the path for the scope-local value and iterate whatever the result is
    match (val_name, &mut array) {
        (Node::Value(set_path), Document::Seq(items)) => {
            let mut result = String::new();
            let ref_vec: Vec<&Document> = set_path.iter().collect();
            for item in items.drain(0..) {
                let r = ctx.set_path(&ref_vec, item);
                if r.is_err() {
                    return Data::check(r);
                }
                let res = exec.exec(ctx).into_result();
                if let Err(e) = res {
                    return e.into();
                }
                result.push_str(&res.unwrap().to_string());
            }
            result.into()
        }
        (Node::Value(set_path), Document::Map(items)) => {
            let mut result = String::new();
            let ref_vec: Vec<&Document> = set_path.iter().collect();
            for (k, v) in items.iter_mut() {
                let mut entry = BTreeMap::new();
                entry.insert("key".into(), k.clone()); //cloning the keys is better than rebalancing the tree
                entry.insert("value".into(), v.take());
                let r = ctx.set_path(&ref_vec, Document::from(entry));
                if r.is_err() {
                    return Data::check(r);
                }
                let res = exec.exec(ctx).into_result();
                if let Err(e) = res {
                    return e.into();
                }
                result.push_str(&res.unwrap().to_string());
            }
            result.into()
        }
        (Node::Value(ref set_path), _) => {
            let ref_vec: Vec<&Document> = set_path.iter().collect();
            let r = ctx.set_path(&ref_vec, array);
            if r.is_err() {
                return Data::check(r);
            }
            exec.exec(ctx)
        }
        _ => Data::from(TemplarError::RenderFailure(
            "Unexpected render failure in for loop".into(),
        )),
    }
}

fn set(ctx: &ContextWrapper, left: &Node, right: &Node) -> Data {
    let val = right.exec(ctx).into_result();
    match (left, val) {
        (_, Err(e)) => e.into(),
        (Node::Value(path), Ok(ref mut val)) => {
            let ref_vec: Vec<&Document> = path.iter().collect();
            Data::check(ctx.set_path(&ref_vec, val.take()))
        }
        (eval, Ok(ref mut val)) => {
            let path = eval.exec(ctx).into_result();
            if let Err(e) = path {
                return e.into();
            }
            let value = path.unwrap();
            Data::check(ctx.set_path(&[&value], val.take()))
        }
    }
}
