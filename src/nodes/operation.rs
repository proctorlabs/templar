use super::*;

pub type PipedFn = fn(&dyn Context, &Node, &Node) -> Node;
//pub type ConditionalFn = fn(TemplarResult, TemplarResult) -> Node;

#[derive(Debug)]
pub enum Type {
    Piped,
    //Conditional,
}

pub struct Operation {
    op_type: Type,
    name: &'static str,
    piped: PipedFn,
    nodes: Vec<Node>,
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Operation{{name: {}, type: {:?}, nodes: {:?}}}",
            self.name, self.op_type, self.nodes
        )
    }
}

impl Operation {
    pub(crate) fn exec(&self, ctx: &dyn Context) -> Node {
        match self.op_type {
            Type::Piped => (self.piped)(ctx, &self.nodes[0], &self.nodes[1]),
        }
    }
}

macro_rules! map_operations {
    ($( $can:ident : $name:ident , )*) => {
        #[derive(Debug, PartialEq)]
        pub enum Operations {
            $( $can , )*
        }

        impl Operations {
            pub fn build(&self, nodes: Vec<Node>) -> Operation {
                match self {
                    $(
                    Operations::$can => Operation {
                        name: stringify!($name),
                        piped: $name,
                        op_type: Type::Piped,
                        nodes,
                    },
                    )*
                }
            }
        }
    };
}

map_operations! {
    Add:add,
    Subtract:subtract,
    Divide:divide,
    Multiply:multiply,
    Modulus:modulus,
    And:and,
    Or:or,
    Equals:equals,
    NotEquals:not_equals,
    GreaterThan:greater_than,
    LessThan:less_than,
    GreaterThanEquals:greater_than_equals,
    LessThanEquals:less_than_equals,
    Concat:concat,
}

macro_rules! define_operations {
    ($( $name:ident ( $l:ident , $r:ident ) -> { $( $tail:tt )* } ; )*) => {
        $(
            fn $name(ctx: &dyn Context, left: &Node, right: &Node) -> Node {
                match (
                    left.exec(ctx).into_document(),
                    right.exec(ctx).into_document(),
                ) {
                    (Ok($l), Ok($r)) => Node::Data(Document::from( $( $tail )* )),
                    (Err(e), _) | (_, Err(e)) => Node::Error(e),
                }
            }
        )*
    };
}

define_operations! {
    add (l, r) -> { l.as_i64().unwrap_or_default() + r.as_i64().unwrap_or_default() };
    subtract(l, r) -> { l.as_i64().unwrap_or_default() - r.as_i64().unwrap_or_default() };
    divide(l, r) -> { l.as_i64().unwrap_or_default() / r.as_i64().unwrap_or_default() };
    multiply(l, r) -> { l.as_i64().unwrap_or_default() * r.as_i64().unwrap_or_default() };
    modulus(l, r) -> { l.as_i64().unwrap_or_default() % r.as_i64().unwrap_or_default() };
    and(l, r) -> { l.as_bool().unwrap_or_default() && r.as_bool().unwrap_or_default() };
    or(l, r) -> { l.as_bool().unwrap_or_default() || r.as_bool().unwrap_or_default() };
    equals(l, r) -> { l == r };
    not_equals(l, r) -> { l != r };
    greater_than(l, r) -> { l > r };
    greater_than_equals(l, r) -> { l >= r };
    less_than(l, r) -> { l < r };
    less_than_equals(l, r) -> { l <= r };
    concat(l, r) -> { format!("{}{}", l, r) };
}
