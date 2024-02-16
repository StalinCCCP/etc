use std::cell::RefCell;
pub(crate) enum Node{
    NotLeaf([u8;32]),
    Leaf(String),
}
pub(crate) fn nodevec_create(n:usize)->Vec<RefCell<Node>>{
    let mut ret=Vec::<RefCell<Node>>::with_capacity(n);
    for _ in 0..n{
        ret.push(RefCell::from(Node::NotLeaf([0;32])))
    }
    ret
}