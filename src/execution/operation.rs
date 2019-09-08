use super::*;

pub struct Metadata {
    pub minimum_nodes: u16,
    pub maximum_nodes: Option<u16>,
}

lazy_static! {
    static ref PIPE_METADATA: Metadata = Metadata {
        minimum_nodes: 2,
        maximum_nodes: Some(2),
    };
    static ref CONDITION_METADATA: Metadata = Metadata {
        minimum_nodes: 2,
        maximum_nodes: Some(3),
    };
    static ref LOOP_METADATA: Metadata = Metadata {
        minimum_nodes: 3,
        maximum_nodes: Some(3),
    };
    static ref INDETERMINATE_METADATA: Metadata = Metadata {
        minimum_nodes: 0,
        maximum_nodes: None,
    };
}

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

macro_rules! map_operations {
    ( $( # $pipe:ident : $pipe_name:ident , )*
      $( ? $condition:ident : $condition_name:ident , )*
      $( * $indeterm:ident : $indeterm_name:ident , )*
      $( o $loopp:ident : $loopp_name:ident , )* ) =>
    {
        #[derive(Debug, PartialEq)]
        pub enum Operations {
            $( $pipe , )*
            $( $condition , )*
            $( $indeterm , )*
            $( $loopp , )*
        }

        impl Operations {
            pub fn metadata(&self) -> &Metadata {
                match self {
                    $( Operations::$pipe => &PIPE_METADATA, )*
                    $( Operations::$condition => &CONDITION_METADATA, )*
                    $( Operations::$indeterm => &INDETERMINATE_METADATA, )*
                    $( Operations::$loopp => &LOOP_METADATA, )*
                }
            }

            pub fn build(&self, nodes: Vec<Node>) -> Operation {
                match self {
                    $( Operations::$pipe => Operation {
                        name: stringify!($pipe_name),
                        oper: PipedExecutor::new($pipe_name).into(),
                        nodes,
                    }, )*
                    $( Operations::$condition => Operation {
                        name: stringify!($condition_name),
                        oper: ConditionalExecutor::new($condition_name).into(),
                        nodes,
                    }, )*
                    $( Operations::$indeterm => Operation {
                        name: stringify!($indeterm_name),
                        oper: IndeterminateExecutor::new($indeterm_name).into(),
                        nodes,
                    }, )*
                    $( Operations::$loopp => Operation {
                        name: stringify!($loopp_name),
                        oper: LoopExecutor::new($loopp_name).into(),
                        nodes,
                    }, )*
                }
            }
        }
    };
}

map_operations! {
    # Add:add,
    # Subtract:subtract,
    # Divide:divide,
    # Multiply:multiply,
    # Modulus:modulus,
    # And:and,
    # Or:or,
    # Equals:equals,
    # NotEquals:not_equals,
    # GreaterThan:greater_than,
    # LessThan:less_than,
    # GreaterThanEquals:greater_than_equals,
    # LessThanEquals:less_than_equals,
    # Set:set,
    ? IfThen:if_then,
    * Concat:concat,
    o ForLoop:for_loop,
}

macro_rules! define_operations {
    ( $( # $pipe_name:ident ( $pip_ctx:ident, $l:ident , $r:ident ) -> { $( $tail:tt )* } ; )*
      $( ? $condition_name:ident ( $cnd_ctx:ident , $cnd:ident , $p:ident , $n:ident ) -> { $( $tail2:tt )* } ; )*
      $( * $indeterm_name:ident ( $ind_ctx:ident , $input:ident ) -> { $( $indeterm_tail:tt )* } ; )* ) =>
    {
        $(
            fn $pipe_name($pip_ctx: &dyn Context, left: &Node, right: &Node) -> Data {
                match (left.exec($pip_ctx).into_document(), right.exec($pip_ctx).into_document()) {
                    (Ok($l), Ok($r)) => Data::from( $( $tail )* ),
                    (Err(e), _) | (_, Err(e)) => Data::from(e),
                }
            }
        )*
        $(
            fn $condition_name($cnd_ctx: &dyn Context, cnd: &Node, $p: &Node, $n: &Node) -> Data {
                let $cnd = cnd.exec($cnd_ctx).into_document();
                $( $tail2 )*
            }
        )*
        $(
            fn $indeterm_name($ind_ctx: &dyn Context, $input: &[Node]) -> Data {
                $( $indeterm_tail )*
            }
        )*
    };
}

define_operations! {
    # add (c, l, r) -> { number!(l) + number!(r) };
    # subtract(c, l, r) -> { number!(l) - number!(r) };
    # divide(c, l, r) -> { number!(l) / number!(r) };
    # multiply(c, l, r) -> { number!(l) * number!(r) };
    # modulus(c, l, r) -> { number!(l) % number!(r) };
    # and(c, l, r) -> { l.as_bool().unwrap_or_default() && r.as_bool().unwrap_or_default() };
    # or(c, l, r) -> { l.as_bool().unwrap_or_default() || r.as_bool().unwrap_or_default() };
    # equals(c, l, r) -> { l == r };
    # not_equals(c, l, r) -> { l != r };
    # greater_than(c, l, r) -> { l > r };
    # greater_than_equals(c, l, r) -> { l >= r };
    # less_than(c, l, r) -> { l < r };
    # less_than_equals(c, l, r) -> { l <= r };
    ? if_then(ctx, cnd, p, n) -> {
        match cnd {
            Ok(Document::Bool(true)) => p.exec(ctx),
            Ok(Document::Bool(false)) => n.exec(ctx),
            Err(e) => e.into(),
            _ => TemplarError::RenderFailure("If condition must evaluate to boolean!".into()).into(),
        }
    };
    * concat(ctx, input) -> {
        let results: Result<Vec<String>> =
            input.iter().map(|node| Ok(node.render(ctx)?)).collect();
        match results {
            Ok(result) => {
                result.iter().fold(String::new(), |mut acc, s| {
                    acc.push_str(&s);
                    acc
                 }).into()
            }
            Err(err) => err.into(),
        }
    };
}

fn for_loop(ctx: &dyn Context, val_name: &Node, array_path: &Node, exec: &Node) -> Data {
    // Get the result for the value we're iterating over
    let array_exec = array_path.exec(ctx).into_document();
    if let Err(e) = array_exec {
        return e.into();
    }
    let mut array = array_exec.unwrap();

    // Now we get the path for the scope-local value and iterate whatever the result is
    match (val_name, &mut array) {
        (Node::Value(set_path), Document::Seq(items)) => {
            let mut result = String::new();
            let ref_vec = set_path.iter().collect::<Vec<_>>();
            for item in items.drain(0..) {
                ctx.set_path(&ref_vec, item);
                let res = exec.exec(ctx).into_document();
                if let Err(e) = res {
                    return e.into();
                }
                result.push_str(&res.unwrap().to_string());
            }
            result.into()
        }
        (Node::Value(set_path), Document::Map(items)) => {
            let mut result = String::new();
            let ref_vec = set_path.iter().collect::<Vec<_>>();
            for (k, v) in items.iter_mut() {
                let mut entry = BTreeMap::new();
                entry.insert("key".into(), k.clone()); //cloning the keys is better than rebalancing the tree
                entry.insert("value".into(), v.take());
                ctx.set_path(&ref_vec, entry.into());
                let res = exec.exec(ctx).into_document();
                if let Err(e) = res {
                    return e.into();
                }
                result.push_str(&res.unwrap().to_string());
            }
            result.into()
        }
        (Node::Value(ref set_path), _) => {
            let ref_vec = set_path.iter().collect::<Vec<_>>();
            ctx.set_path(&ref_vec, array);
            exec.exec(ctx)
        }
        _ => Data::from(TemplarError::RenderFailure(
            "Unexpected render failure in for loop".into(),
        )),
    }
}

fn set(ctx: &dyn Context, left: &Node, right: &Node) -> Data {
    let val = right.exec(ctx).into_document();
    match (left, val) {
        (_, Err(e)) => e.into(),
        (Node::Value(path), Ok(ref mut val)) => {
            let ref_vec = path.iter().collect::<Vec<_>>();
            ctx.set_path(&ref_vec, val.take());
            Data::empty()
        }
        (eval, Ok(ref mut val)) => {
            let path = eval.exec(ctx).into_document();
            if let Err(e) = path {
                return e.into();
            }
            let value = path.unwrap().to_string();
            ctx.set_path(&[&value], val.take());
            Data::empty()
        }
    }
}
