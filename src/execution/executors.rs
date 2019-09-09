use super::*;

pub struct Metadata {
    pub minimum_nodes: u16,
    pub maximum_nodes: Option<u16>,
}

pub(crate) enum Executors {
    Piped(PipedExecutor),
    Conditional(ConditionalExecutor),
    Indeterminate(IndeterminateExecutor),
    Loop(LoopExecutor),
}

pub(crate) struct IndeterminateExecutor(fn(&dyn Context, input: &[Node]) -> Data);

impl IndeterminateExecutor {
    #[inline]
    pub fn new(new_fn: fn(&dyn Context, input: &[Node]) -> Data) -> Self {
        Self(new_fn)
    }

    #[inline]
    pub(crate) fn metadata() -> &'static Metadata {
        &Metadata {
            minimum_nodes: 0,
            maximum_nodes: None,
        }
    }
}

pub(crate) struct ConditionalExecutor(
    fn(&dyn Context, condition: &Node, positive: &Node, negative: &Node) -> Data,
);

impl ConditionalExecutor {
    #[inline]
    pub fn new(
        new_fn: fn(&dyn Context, condition: &Node, positive: &Node, negative: &Node) -> Data,
    ) -> Self {
        Self(new_fn)
    }

    #[inline]
    pub(crate) fn metadata() -> &'static Metadata {
        &Metadata {
            minimum_nodes: 2,
            maximum_nodes: Some(3),
        }
    }
}
pub(crate) struct PipedExecutor(fn(&dyn Context, left: &Node, right: &Node) -> Data);

impl PipedExecutor {
    #[inline]
    pub fn new(new_fn: fn(&dyn Context, left: &Node, right: &Node) -> Data) -> Self {
        Self(new_fn)
    }

    #[inline]
    pub(crate) fn metadata() -> &'static Metadata {
        &Metadata {
            minimum_nodes: 2,
            maximum_nodes: Some(2),
        }
    }
}

pub(crate) struct LoopExecutor(
    fn(&dyn Context, ctx_name: &Node, arr: &Node, to_loop: &Node) -> Data,
);

impl LoopExecutor {
    #[inline]
    pub fn new(
        new_fn: fn(&dyn Context, val_name: &Node, val_array: &Node, exec: &Node) -> Data,
    ) -> Self {
        Self(new_fn)
    }

    #[inline]
    pub(crate) fn metadata() -> &'static Metadata {
        &Metadata {
            minimum_nodes: 3,
            maximum_nodes: Some(3),
        }
    }
}

pub(crate) trait Executor {
    fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Data;
}

impl From<PipedExecutor> for Executors {
    #[inline]
    fn from(t: PipedExecutor) -> Self {
        Self::Piped(t)
    }
}

impl From<ConditionalExecutor> for Executors {
    #[inline]
    fn from(t: ConditionalExecutor) -> Self {
        Self::Conditional(t)
    }
}

impl From<IndeterminateExecutor> for Executors {
    #[inline]
    fn from(t: IndeterminateExecutor) -> Self {
        Self::Indeterminate(t)
    }
}

impl From<LoopExecutor> for Executors {
    #[inline]
    fn from(t: LoopExecutor) -> Self {
        Self::Loop(t)
    }
}

impl Executor for Executors {
    #[inline]
    fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Data {
        match self {
            Self::Piped(ref ex) => ex.exec(ctx, nodes),
            Self::Conditional(ref ex) => ex.exec(ctx, nodes),
            Self::Indeterminate(ref ex) => ex.exec(ctx, nodes),
            Self::Loop(ref ex) => ex.exec(ctx, nodes),
        }
    }
}

impl Executor for IndeterminateExecutor {
    #[inline]
    fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Data {
        self.0(ctx, &nodes)
    }
}

impl Executor for PipedExecutor {
    #[inline]
    fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Data {
        self.0(ctx, &nodes[0], &nodes[1])
    }
}

impl Executor for ConditionalExecutor {
    #[inline]
    fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Data {
        self.0(ctx, &nodes[0], &nodes[1], &nodes[2])
    }
}

impl Executor for LoopExecutor {
    #[inline]
    fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Data {
        self.0(ctx, &nodes[0], &nodes[1], &nodes[2])
    }
}
