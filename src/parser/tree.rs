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

    pub fn set_op(&mut self, op: Operations) -> Result<()> {
        if self.current_op.is_some() {
            self.finish_op()?;
        }
        self.current_op = Some(op);
        Ok(())
    }

    pub fn finish_op(&mut self) -> Result<()> {
        if let Some(ref op) = self.current_op {
            let len = self.tree.len();
            let metadata = op.metadata();
            if len < metadata.minimum_nodes as usize {
                return Err(TemplarError::ParseFailure(format!(
                    "{:?} op requires at least {} nodes",
                    op, metadata.minimum_nodes
                )));
            }
            if let Some(max) = metadata.maximum_nodes {
                if len > max as usize {
                    return Err(TemplarError::ParseFailure(format!(
                        "{:?} op has a maximum of {} nodes",
                        op, max
                    )));
                }
                while self.tree.len() < max as usize {
                    self.tree.push(Node::default())
                }
            }
            let tree = replace(&mut self.tree, vec![]);
            let node = Node::Operation(Arc::new(op.build(tree)));
            self.tree.push(node);
            self.current_op = None;
        }
        Ok(())
    }

    pub fn push(&mut self, node: Node) -> Result<()> {
        self.tree.push(node);
        if let Some(ref op) = self.current_op {
            let metadata = op.metadata();
            if metadata.maximum_nodes.is_some()
                && self.tree.len() == metadata.maximum_nodes.unwrap() as usize
            {
                return self.finish_op();
            }
        }
        Ok(())
    }

    pub fn filter(&mut self, filter: &str, args: Node) -> Result<()> {
        self.finish_op()?;
        let tree = replace(&mut self.tree, vec![]);
        // let nodes = vec![tree.into(), args];
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

    pub fn into_node(mut self) -> Result<Node> {
        self.finish_op()?;
        Ok(self.tree.into())
    }

    pub fn into_nodes(mut self) -> Result<Vec<Node>> {
        self.finish_op()?;
        Ok(self.tree)
    }
}
