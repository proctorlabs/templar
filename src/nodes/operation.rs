use super::*;

use executors::*;

mod executors {
    use super::*;

    pub(crate) enum Executors {
        Piped(PipedExecutor),
        Conditional(ConditionalExecutor),
        Indeterminate(IndeterminateExecutor),
        Loop(LoopExecutor),
    }

    pub(crate) struct IndeterminateExecutor(fn(&dyn Context, input: &[Node]) -> Node);

    impl IndeterminateExecutor {
        pub fn new(new_fn: fn(&dyn Context, input: &[Node]) -> Node) -> Self {
            Self(new_fn)
        }
    }

    pub(crate) struct ConditionalExecutor(
        fn(&dyn Context, condition: &Node, positive: &Node, negative: &Node) -> Node,
    );

    impl ConditionalExecutor {
        pub fn new(
            new_fn: fn(&dyn Context, condition: &Node, positive: &Node, negative: &Node) -> Node,
        ) -> Self {
            Self(new_fn)
        }
    }
    pub(crate) struct PipedExecutor(fn(&dyn Context, left: &Node, right: &Node) -> Node);

    impl PipedExecutor {
        pub fn new(new_fn: fn(&dyn Context, left: &Node, right: &Node) -> Node) -> Self {
            Self(new_fn)
        }
    }

    pub(crate) struct LoopExecutor(
        fn(&dyn Context, ctx_name: &Node, arr: &Node, to_loop: &Node) -> Node,
    );

    impl LoopExecutor {
        pub fn new(
            new_fn: fn(&dyn Context, val_name: &Node, val_array: &Node, exec: &Node) -> Node,
        ) -> Self {
            Self(new_fn)
        }
    }

    pub(crate) trait Executor {
        fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Node;
    }

    impl From<PipedExecutor> for Executors {
        fn from(t: PipedExecutor) -> Self {
            Self::Piped(t)
        }
    }

    impl From<ConditionalExecutor> for Executors {
        fn from(t: ConditionalExecutor) -> Self {
            Self::Conditional(t)
        }
    }

    impl From<IndeterminateExecutor> for Executors {
        fn from(t: IndeterminateExecutor) -> Self {
            Self::Indeterminate(t)
        }
    }

    impl From<LoopExecutor> for Executors {
        fn from(t: LoopExecutor) -> Self {
            Self::Loop(t)
        }
    }

    impl Executor for Executors {
        fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Node {
            match self {
                Self::Piped(ref ex) => ex.exec(ctx, nodes),
                Self::Conditional(ref ex) => ex.exec(ctx, nodes),
                Self::Indeterminate(ref ex) => ex.exec(ctx, nodes),
                Self::Loop(ref ex) => ex.exec(ctx, nodes),
            }
        }
    }

    impl Executor for IndeterminateExecutor {
        fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Node {
            self.0(ctx, &nodes)
        }
    }

    impl Executor for PipedExecutor {
        fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Node {
            self.0(ctx, &nodes[0], &nodes[1])
        }
    }

    impl Executor for ConditionalExecutor {
        fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Node {
            self.0(ctx, &nodes[0], &nodes[1], &nodes[2])
        }
    }

    impl Executor for LoopExecutor {
        fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Node {
            self.0(ctx, &nodes[0], &nodes[1], &nodes[2])
        }
    }
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
    pub(crate) fn exec(&self, ctx: &dyn Context) -> Node {
        Executor::exec(&self.oper, ctx, &self.nodes)
    }
}

macro_rules! number {
    ($doc:ident) => {
        match $doc.as_i64() {
            Some(i) => i,
            None => {
                return Node::Data(
                    TemplarError::RenderFailure("Math operations require numeric types".into())
                        .into(),
                )
            }
        }
    };
}

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
      $( * $indeterm_name:ident ( $ind_ctx:ident , $input:ident ) -> { $( $indeterm_tail:tt )* } ; )*
      $( o $loop_name:ident ( $loop_ctx:ident , $loop_newval:ident , $loop_var:ident , $loop_exec:ident) -> { $( $loop_tail:tt )* } ; )* ) =>
    {
        $(
            fn $pipe_name($pip_ctx: &dyn Context, left: &Node, right: &Node) -> Node {
                match (left.exec($pip_ctx).into_document(), right.exec($pip_ctx).into_document()) {
                    (Ok($l), Ok($r)) => Node::Data(Document::from( $( $tail )* ).into()),
                    (Err(e), _) | (_, Err(e)) => Node::Data(e.into()),
                }
            }
        )*
        $(
            fn $condition_name($cnd_ctx: &dyn Context, cnd: &Node, $p: &Node, $n: &Node) -> Node {
                let $cnd = cnd.exec($cnd_ctx).into_document();
                $( $tail2 )*
            }
        )*
        $(
            fn $indeterm_name($ind_ctx: &dyn Context, $input: &[Node]) -> Node {
                $( $indeterm_tail )*
            }
        )*
        $(
            fn $loop_name($loop_ctx: &dyn Context, $loop_newval: &Node, $loop_var: &Node, $loop_exec: &Node) -> Node {
                $( $loop_tail )*
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
    # set (c, l, r) -> {{
        let id = l.to_string();
        c.set_path(&[&id], r);
        Document::Unit
    }};
    ? if_then(ctx, cnd, p, n) -> {
        match cnd {
            Ok(Document::Bool(true)) => p.exec(ctx),
            Ok(Document::Bool(false)) => n.exec(ctx),
            Err(e) => Node::Data(e.into()),
            _ => Node::Data(TemplarError::RenderFailure("If condition must evaluate to boolean!".into()).into()),
        }
    };
    * concat(ctx, input) -> {
        let results: Result<Vec<String>> =
            input.iter().map(|node| Ok(node.render(ctx)?)).collect();
        match results {
            Ok(result) => {
                Node::Data(result.iter().fold(String::new(), |mut acc, s| {
                    acc.push_str(&s);
                    acc
                 }).into())
            }
            Err(err) => Node::Data(err.into()),
        }
    };
    o for_loop(ctx, val_name, array_path, exec) -> {
        // Get the result for the value we're iterating over
        let array_exec = array_path.exec(ctx).into_document();
        if array_exec.is_err() {
            return array_exec.into();
        }
        let array = array_exec.unwrap();

        // Now we get the path for the scope-local value and iterate whatever the result is
        match (val_name, array) {
            (Node::Value(ref set_path), Document::Seq(ref items)) => {
                let mut result = String::new();
                let ref_vec = set_path.iter().collect::<Vec<_>>();
                for item in items.iter() {
                    ctx.set_path(&ref_vec, item.clone());
                    let res = exec.exec(ctx).into_document();
                    if res.is_err() {
                        return Node::Data(res.unwrap_err().into());
                    }
                    result.push_str(&res.unwrap().to_string());
                }
                Node::Data(result.into())
            }
            _ => Node::Data(Data::from(TemplarError::RenderFailure("Unexpected render failure in for loop".into())))
        }
    };
}
