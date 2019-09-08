use super::*;

pub(crate) enum Executors {
    Piped(PipedExecutor),
    Conditional(ConditionalExecutor),
    Indeterminate(IndeterminateExecutor),
    Loop(LoopExecutor),
}

pub(crate) struct IndeterminateExecutor(fn(&dyn Context, input: &[Node]) -> Data);

impl IndeterminateExecutor {
    pub fn new(new_fn: fn(&dyn Context, input: &[Node]) -> Data) -> Self {
        Self(new_fn)
    }
}

pub(crate) struct ConditionalExecutor(
    fn(&dyn Context, condition: &Node, positive: &Node, negative: &Node) -> Data,
);

impl ConditionalExecutor {
    pub fn new(
        new_fn: fn(&dyn Context, condition: &Node, positive: &Node, negative: &Node) -> Data,
    ) -> Self {
        Self(new_fn)
    }
}
pub(crate) struct PipedExecutor(fn(&dyn Context, left: &Node, right: &Node) -> Data);

impl PipedExecutor {
    pub fn new(new_fn: fn(&dyn Context, left: &Node, right: &Node) -> Data) -> Self {
        Self(new_fn)
    }
}

pub(crate) struct LoopExecutor(
    fn(&dyn Context, ctx_name: &Node, arr: &Node, to_loop: &Node) -> Data,
);

impl LoopExecutor {
    pub fn new(
        new_fn: fn(&dyn Context, val_name: &Node, val_array: &Node, exec: &Node) -> Data,
    ) -> Self {
        Self(new_fn)
    }
}

pub(crate) trait Executor {
    fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Data;
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
    fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Data {
        self.0(ctx, &nodes)
    }
}

impl Executor for PipedExecutor {
    fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Data {
        self.0(ctx, &nodes[0], &nodes[1])
    }
}

impl Executor for ConditionalExecutor {
    fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Data {
        self.0(ctx, &nodes[0], &nodes[1], &nodes[2])
    }
}

impl Executor for LoopExecutor {
    fn exec(&self, ctx: &dyn Context, nodes: &[Node]) -> Data {
        self.0(ctx, &nodes[0], &nodes[1], &nodes[2])
    }
}
