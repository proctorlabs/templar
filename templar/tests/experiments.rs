use std::fmt;
use templar::TemplarError;
use unstructured::UnstructuredDataTrait;

#[derive(Clone)]
pub enum UDData {
    Thing,
}

impl fmt::Display for UDData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<UDDatat>")
    }
}

impl UnstructuredDataTrait for UDData {
    type ErrorType = TemplarError;
    type OtherType = UDData;
}

pub type TestData = unstructured::Unstructured<UDData>;

#[test]
fn custom_filter_repeater() -> Result<(), TemplarError> {
    Ok(())
}

// use unstructured::UnstructuredDataTrait;

// #[derive(Clone)]
// pub enum Node2 {
//     Expr(Vec<NodeData>),
//     Scope(Box<NodeData>),
//     Operation(Arc<Operation>),
// }

// impl fmt::Display for Node2 {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "<Node>")
//     }
// }

// impl UnstructuredDataTrait for Node2 {
//     type ErrorType = TemplarError;
//     type OtherType = Node2;
// }

// pub type NodeData = unstructured::Unstructured<Node2>;

// pub trait NodeTrait {
//     fn exec(&self, ctx: &impl Context) -> NodeData;
//     fn set_operation(self, op: Operations) -> NodeData;
//     fn into_scope(self) -> NodeData;
//     fn into_document(self) -> Result<NodeData>;
//     fn render(&self, ctx: &impl Context) -> Result<String>;
// }

// impl NodeTrait for NodeData {
//     fn exec(&self, ctx: &impl Context) -> NodeData {
//         match self {
//             // Self::Other(Node2::Operation(op)) => op.exec(ctx),
//             // Self::Value(a) => {
//             //     let docs = a.iter().collect::<Vec<&Document>>();
//             //     ctx.get_path(&docs)
//             // }
//             Self::Other(Node2::Scope(i)) => {
//                 let local_context = ctx.create_scope();
//                 i.exec(&local_context)
//             }
//             Self::Other(Node2::Expr(a)) => {
//                 let mut res: Vec<NodeData> = a.iter().map(|n| n.exec(ctx)).collect();
//                 if res.is_empty() {
//                     Self::Unassigned
//                 } else if res.len() == 1 {
//                     res.pop().unwrap()
//                 } else {
//                     NodeData::from(res)
//                 }
//             }
//             _ => self.clone(),
//         }
//     }

//     fn set_operation(self, op: Operations) -> NodeData {
//         match self {
//             // Self::Other(Node2::Expr(nodes)) => Node2::Operation(Arc::new(op.build(nodes))),
//             _ => self,
//         }
//     }

//     fn into_scope(self) -> NodeData {
//         Self::Other(Node2::Scope(Box::new(self)))
//     }

//     fn into_document(self) -> Result<NodeData> {
//         Ok(self)
//     }

//     fn render(&self, ctx: &impl Context) -> Result<String> {
//         // self.exec(ctx).render()
//         Ok("TODO".into())
//     }
// }
