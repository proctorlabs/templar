use super::*;

use executors::*;

mod executors {
    use super::*;

    pub(crate) enum Executors {
        Piped(PipedExecutor),
        Conditional(ConditionalExecutor),
        Indeterminate(IndeterminateExecutor),
    }

    pub(crate) struct IndeterminateExecutor(fn(&dyn Context, input: &[Node]) -> Node);

    impl IndeterminateExecutor {
        pub fn new(new_fn: fn(&dyn Context, input: &[Node]) -> Node) -> Self {
            IndeterminateExecutor(new_fn)
        }
    }

    pub(crate) struct ConditionalExecutor(
        fn(&dyn Context, condition: &Node, positive: &Node, negative: &Node) -> Node,
    );

    impl ConditionalExecutor {
        pub fn new(
            new_fn: fn(&dyn Context, condition: &Node, positive: &Node, negative: &Node) -> Node,
        ) -> Self {
            ConditionalExecutor(new_fn)
        }
    }
    pub(crate) struct PipedExecutor(fn(&dyn Context, left: &Node, right: &Node) -> Node);

    impl PipedExecutor {
        pub fn new(new_fn: fn(&dyn Context, left: &Node, right: &Node) -> Node) -> Self {
            PipedExecutor(new_fn)
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

    impl Executor for Executors {
        fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Node {
            match self {
                Self::Piped(ref ex) => ex.exec(ctx, nodes),
                Self::Conditional(ref ex) => ex.exec(ctx, nodes),
                Self::Indeterminate(ref ex) => ex.exec(ctx, nodes),
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
                return Node::Error(TemplarError::RenderFailure(
                    "Math operations require numeric types".into(),
                ))
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
    static ref INDETERMINATE_METADATA: Metadata = Metadata {
        minimum_nodes: 0,
        maximum_nodes: None,
    };
}

macro_rules! map_operations {
    ( $( # $pipe:ident : $pipe_name:ident , )*
      $( ? $condition:ident : $condition_name:ident , )*
      $( * $indeterm:ident : $indeterm_name:ident , )* ) =>
    {
        #[derive(Debug, PartialEq)]
        pub enum Operations {
            $( $pipe , )*
            $( $condition , )*
            $( $indeterm , )*
        }

        impl Operations {
            pub fn metadata(&self) -> &Metadata {
                match self {
                    $( Operations::$pipe => &PIPE_METADATA, )*
                    $( Operations::$condition => &CONDITION_METADATA, )*
                    $( Operations::$indeterm => &INDETERMINATE_METADATA, )*
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
                    } )*
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
    ? IfThen:if_then,
    * Concat:concat,
}

macro_rules! define_operations {
    ( $( # $pipe_name:ident ( $l:ident , $r:ident ) -> { $( $tail:tt )* } ; )*
      $( ? $condition_name:ident ( $cnd_ctx:ident , $cnd:ident , $p:ident , $n:ident ) -> { $( $tail2:tt )* } ; )*
      $( * $indeterm_name:ident ( $ind_ctx:ident , $input:ident ) -> { $( $tail3:tt )* } ; )* ) =>
    {
        $(
            fn $pipe_name(ctx: &dyn Context, left: &Node, right: &Node) -> Node {
                match (left.exec(ctx).into_document(), right.exec(ctx).into_document()) {
                    (Ok($l), Ok($r)) => Node::Data(Document::from( $( $tail )* )),
                    (Err(e), _) | (_, Err(e)) => Node::Error(e),
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
                $( $tail3 )*
            }
        )*
    };
}

define_operations! {
    # add (l, r) -> { number!(l) + number!(r) };
    # subtract(l, r) -> { number!(l) - number!(r) };
    # divide(l, r) -> { number!(l) / number!(r) };
    # multiply(l, r) -> { number!(l) * number!(r) };
    # modulus(l, r) -> { number!(l) % number!(r) };
    # and(l, r) -> { l.as_bool().unwrap_or_default() && r.as_bool().unwrap_or_default() };
    # or(l, r) -> { l.as_bool().unwrap_or_default() || r.as_bool().unwrap_or_default() };
    # equals(l, r) -> { l == r };
    # not_equals(l, r) -> { l != r };
    # greater_than(l, r) -> { l > r };
    # greater_than_equals(l, r) -> { l >= r };
    # less_than(l, r) -> { l < r };
    # less_than_equals(l, r) -> { l <= r };
    ? if_then(ctx, cnd, p, n) -> {
        match cnd {
            Ok(Document::Bool(true)) => p.exec(ctx),
            Ok(Document::Bool(false)) => n.exec(ctx),
            Err(e) => Node::Error(e),
            _ => Node::Error(TemplarError::RenderFailure("If condition must evaluate to boolean!".into())),
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
            Err(err) => Node::Error(err),
        }
    };
}
