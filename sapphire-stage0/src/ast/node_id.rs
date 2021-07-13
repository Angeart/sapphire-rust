use ghost::phantom;

#[phantom]
#[derive(Debug, Clone)]
pub struct NodeId<T: ?Sized>;
