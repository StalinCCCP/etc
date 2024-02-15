use sha2::{ Digest, Sha256};
use std::{cell::RefCell, ops::DerefMut};
enum Node{
    NotLeaf(Vec<u8>),
    Leaf(String),
}
// struct Node{
//     data:
//     is_leaf:bool
// }
fn nodevec_create(n:usize)->Vec<RefCell<Node>>{
    let mut ret=Vec::<RefCell<Node>>::with_capacity(n);
    for _ in 0..n{
        ret.push(RefCell::from(Node::NotLeaf(Vec::<u8>::new())))
    }
    ret
}
pub struct MerkelTree{
    node:Vec<RefCell<Node>>
    //lvl:usize
}
fn to_hash<T:AsRef<[u8]>>(s:&T)->Vec<u8>{
    let mut hasher = Sha256::new();
    hasher.update(s);
    hasher.finalize().to_vec()
}
impl MerkelTree{
    fn build_hash(v:&Vec<RefCell<Node>>,index:usize)->Vec<u8>{
        //let mut V=v.borrow_mut();
        match v[index].borrow_mut().deref_mut(){
            Node::Leaf(s)=>to_hash(&s),
            Node::NotLeaf(s)=>{
                let mut l=MerkelTree::build_hash(v,index<<1);
                let mut r=MerkelTree::build_hash(v,index<<1|1);
                l.append(&mut r);
                *s=to_hash(&l);
                Vec::<u8>::clone(s)
            }
        }
    }
    pub fn build(s:&Vec<String>)->MerkelTree{
        let n=s.len();
        let is_odd=if s.len()%2==1 {true} else {false};
        if is_odd{
            let mut v=nodevec_create(2*n+2);//Vec::<Node>::with_capacity(2*n+2);//[0,2*n-1]
            {
                let mut j=0;
                for i in n+1..2*n+1{
                    v[i]=RefCell::from(Node::Leaf(s[j].clone()));
                    j+=1;
                }
            }
            v[2*n+1]=RefCell::from(Node::Leaf(s[n-1].clone()));
            //let v=RefCell::new(v);
            MerkelTree::build_hash(&v,1);
            MerkelTree{node:v}
        }else{
            let mut v=nodevec_create(2*n);//[0,2*n-1]
            {
                let mut j=0;
                for i in n..2*n{
                    v[i]=RefCell::from(Node::Leaf(s[j].clone()));
                    j+=1;
                }
            }
            //let v=RefCell::new(v);
            MerkelTree::build_hash(&v,1);
            MerkelTree{node:v}
        }
        
        //let mut v=Vec::<Node>::with_capacity(2*n);
        //定义最底层为第0层
    }
    fn verify_index(self:&Self,index:usize)->bool{
        match &self.node[index].borrow_mut().deref_mut(){
            Node::Leaf(_)=>true,
            Node::NotLeaf(s)=>{
                if self.verify_index(index<<1)&&self.verify_index(index<<1|1){
                    let mut l=Vec::<u8>::new();
                    let mut r=Vec::<u8>::new();
                    l=match &self.node[index<<1].borrow_mut().deref_mut(){
                        Node::Leaf(s)=>to_hash(s),
                        Node::NotLeaf(s)=>Vec::<u8>::clone(s)
                    };
                    r=match &self.node[index<<1|1].borrow_mut().deref_mut(){
                        Node::Leaf(s)=>to_hash(s),
                        Node::NotLeaf(s)=>Vec::<u8>::clone(s)
                    };
                    l.append(&mut r);
                    // println!("{:?}",*s);
                    // println!("{:?}",to_hash(&l));
                    *s==to_hash(&l)
                }else{
                    false
                }
            }
        }
    }
    pub fn verify(self:&Self)->bool{
        MerkelTree::verify_index(&self,1)
    }
}

#[cfg(test)]
mod tests {

    use crate::MerkelTree;

    #[test]
    fn merkel_tree_test(){
        let str=vec![String::from("testing"),String::from("hello"),String::from("It's almost done")];
        let out=if MerkelTree::build(&str).verify() {"success"}else{"unsuccess"};
        println!("{out}");
    }
}