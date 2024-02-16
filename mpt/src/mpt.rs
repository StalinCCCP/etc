use crate::{
    node::{Node::{Full,Short,Value},FullNode,ShortNode,ValueNode,NodePointer,DataPointer},
    hashing::to_hash,
    defs::{Path,Hash,Data},
};
pub struct MPT{
    root:NodePointer
}
fn query_short(key:&[Path],p:&ShortNode)->Result<Option<DataPointer>,&'static str>{
    let klen=key.len();
    let plen=p.get_keylen();
    if plen>klen{
        Ok(None)
    }else{
        let matched=p.match_path_with(key);
        let child=p.get_child().unwrap();
        let child=child.as_ref();
        if matched==klen{
            match child{
                Value(v)=>Ok(query_value(v)),
                _=>Err("MPT::query_short:corrupted"),
            }
        }else{
            match &child{
                Value(_)=>Ok(None),
                Short(p)=>query_short(&key[matched..],p),
                Full(p)=>query_full(&key[matched..],p)
            }
        }
    }
}
fn query_full(key:& [Path],p:&FullNode)->Result<Option<DataPointer>,&'static str>{
    let cur=key[0];
    if let None=p.get_child(cur)?{
        Ok(None)
    }else{
        let child=p.get_child(cur).unwrap().unwrap();
        let child=child.as_ref();
        match child{
            Value(p)=>Ok(query_value(p)),
            Full(p)=>query_full(&key[1..],p),
            Short(p)=>query_short(&key[1..],p)
        }
    }
}
fn query_value(p:&ValueNode)->Option<DataPointer>{
    p.get()
}
impl MPT{
    pub fn new()->MPT{
        MPT { root: NodePointer::new(Full(FullNode::new())) }
    }
    pub fn query(&self,key:&[Path])->Result<Option<DataPointer>,&'static str>{
        if let Full(root)=self.root.as_ref(){
            query_full(key,root)
        }else {
            Err("MPT::query:It's just bullshit")
        }
    }
}