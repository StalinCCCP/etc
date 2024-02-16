use std::{cell::RefCell, ops::DerefMut};
use crate::{node::{Node,nodevec_create},hashing::to_hash};
pub struct MerkelTree{
    node:Vec<RefCell<Node>>
    //lvl:usize
}

impl MerkelTree{
    fn build_hash(v:&Vec<RefCell<Node>>,index:usize)->Vec<u8>{
        //let mut V=v.borrow_mut();
        match v[index].borrow_mut().deref_mut(){
            Node::Leaf(s)=>to_hash(&s).to_vec(),
            Node::NotLeaf(s)=>{
                let mut l=MerkelTree::build_hash(v,index<<1);
                let mut r=MerkelTree::build_hash(v,index<<1|1);
                l.append(&mut r);
                *s=to_hash(&l);
                s.clone().to_vec()
                //Vec::<u8>::clone(*s.to_vec())
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
                        Node::Leaf(s)=>to_hash(s).to_vec(),
                        Node::NotLeaf(s)=>Vec::<u8>::from(s)
                    };
                    r=match &self.node[index<<1|1].borrow_mut().deref_mut(){
                        Node::Leaf(s)=>to_hash(s).to_vec(),
                        Node::NotLeaf(s)=>Vec::<u8>::from(s)
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
