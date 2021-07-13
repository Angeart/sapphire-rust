pub mod node_id;
pub mod primitive;
use crate::ast::node_id::NodeId;
use std::collections::HashMap;

pub type NodeMap<T: ?Sized> = HashMap<NodeId<T>, T>;

#[derive(Debug, Clone)]
pub struct Function {
  pub id: NodeId<Function>,
}

#[derive(Debug, Clone)]
pub struct Root {
  pub id: NodeId<Root>,
  pub functions: NodeMap<Function>,
}
