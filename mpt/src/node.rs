use std::{cell::Cell, rc::Rc};

use crate::{Hash,Path,Data};
pub(crate) enum Node{
    Full(FullNode),
    Short(ShortNode),
    Value(ValueNode)
}
// trait Node{
//     fn cache(self:&Self)->(&Hash,bool);
//     fn which(self:&Self)->&'static NodeType;
//     fn alter_hash(self:&mut Self,src:Hash);
// }
struct NodeFlag{
    hash:Option<Hash>,
    dirty:bool
}
impl NodeFlag{
    fn new()->NodeFlag{
        NodeFlag{
            hash:None,
            dirty:false
        }
    }
    fn alter_hash(self:&mut Self,src:Hash){
        if let Some(_)=self.hash{
            self.dirty=true;
        }
        self.hash=Some(src);
    }
}
pub type NodePointer=Rc<Node>;
pub type DataPointer=Rc<Data>;
const FULL_NODE_CHILDREN_SIZE:usize=16;
pub(crate) struct FullNode{
    children:[Option<NodePointer>;FULL_NODE_CHILDREN_SIZE],
    flags:NodeFlag
}
const ARRAY_INIT_VALUE: Option<NodePointer> = None;
impl FullNode{
    pub(crate) fn new()->FullNode{
        FullNode{
            children:[ARRAY_INIT_VALUE;FULL_NODE_CHILDREN_SIZE],
            flags:NodeFlag::new()
        }
    }
    pub(crate) fn add_child(self:&mut Self,pos:Path,node:NodePointer)->Result<(),&'static str>{
        let pos=pos as usize;
        if pos >=FULL_NODE_CHILDREN_SIZE{
            Err("FullNode::add_child:outbounds")
        }else{
            match &mut self.children[pos]{
                Some(_)=> Err("FullNode::add_child:there is a child already"),
                None=>{
                    self.children[pos]=Some(node);
                    Ok(())
                }
            }
        }
    }
    pub(crate) fn del_child(self:&mut Self,pos:Path)->Result<(),&'static str>{
        let pos=pos as usize;
        if pos >=FULL_NODE_CHILDREN_SIZE{
            Err("FullNode::del_child:outbounds")
        }else{
            match &mut self.children[pos]{
                None=> Err("FullNode::del_child:there isn't a child yet"),
                Some(_)=>{
                    self.children[pos].take();
                    //self.children[pos]=Some(node);
                    Ok(())
                }
            }
        }
    }
    pub(crate) fn get_child(self:&Self,pos:Path)->Result<Option<NodePointer>,&'static str>{
        let pos=pos as usize;
        if pos>=FULL_NODE_CHILDREN_SIZE{
            Err("FullNode::get_child:outbounds")
        }else{
            match &self.children[pos]{
                None=> Ok(None),
                Some(ptr)=>{
                    Ok(Some(ptr.clone()))
                }
            }
        }
    }
    pub(crate) fn cache(self:&Self)->(&Option<Hash>,bool){
        (&self.flags.hash,self.flags.dirty)
    }
    pub(crate) fn alter_hash(self:&mut Self,src:Hash){
        self.flags.alter_hash(src);
    }
}
pub(crate) struct ShortNode{
    key:Vec<Path>,
    val:Option<NodePointer>,
    flags:NodeFlag
}
impl ShortNode{
    pub(crate) fn new()->ShortNode{
        ShortNode{
            key:Vec::<Path>::new(),
            val:None,
            flags:NodeFlag::new()
        }
    }
    pub(crate) fn new_from_path(path:&[Path])->ShortNode{
        ShortNode{
            key:path.to_vec(),
            val:None,
            flags:NodeFlag::new()
        }
    }
    pub(crate) fn get_child(self:&Self)->Option<NodePointer>{
        match &self.val{
            None=>None,
            Some(ptr)=>Some(ptr.clone())
        }
    }
    pub(crate) fn add_child(self:&mut Self,node:NodePointer)->Result<(),&'static str>{
        match &mut self.val{
            None=>{
                self.val=Some(node);
                Ok(())
            }
            Some(_)=>
                Err("ShortNode::add_child:there is a child already")
        }
    }
    pub(crate) fn del_child(self:&mut Self)->Result<(),&'static str>{
        match &mut self.val{
            None=>{
                Err("ShortNode::del_child:there isn't a child yet")
            }
            Some(_)=>{
                self.val.take();
                Ok(())
            }
        }
    }
    pub(crate) fn get_keylen(self:&Self)->usize{
        self.key.len()
    }
    pub(crate) fn match_path_with(self:&Self,path:&[Path])->usize{
        let plen=path.len();
        let nlen=self.key.len();
        let mut ret:usize=0;
        {
            let mut j=0;
            for i in 0..plen{
                if self.key[j]!=path[i]{
                    break;
                }
                j+=1;
                ret+=1;
                if j==nlen-1{
                    break;
                }
            }
        }
        ret
    }//多少个 使用时直接 其中[0,pos)匹配
    pub(crate) fn cut_at(self:&mut Self,pos:usize)->Result<(),&'static str>{//剩余[0,pos)
        if pos>self.key.len(){
            Err("ShortNode::cut_at:not long enough")
        }else{
            self.del_child();
            //self.key.
            self.key=self.key[..pos].to_vec();
            Ok(())
        }
    }
    //[pos,len)
    pub(crate) fn subkey_from(self:&Self,pos:usize)->Result<ShortNode,&'static str>{
        let mut ret=ShortNode::new_from_path(&self.key[pos..]);
        ret.add_child(self.get_child().unwrap())?;
        Ok(ret)
    }
    pub(crate) fn cache(self:&Self)->(&Option<Hash>,bool){
        (&self.flags.hash,self.flags.dirty)
    }
    pub(crate) fn alter_hash(self:&mut Self,src:Hash){
        self.flags.alter_hash(src);
    }
}
pub(crate) struct ValueNode{
    data:Option<DataPointer>,
    flags:NodeFlag
}
impl ValueNode{
    pub(crate) fn new()->ValueNode{
        ValueNode{
            data:None,
            flags:NodeFlag::new()
        }
    }
    pub(crate) fn new_with_data(d:Data)->ValueNode{
        ValueNode{
            data:None,
            flags:NodeFlag::new()
        }
    }
    pub(crate) fn set(&mut self,d:Data){
        self.data.take();
        self.data=Some(DataPointer::new(d));
    }
    // pub(crate) fn get_as_ref<'a,'b>(&'a self)->&'b Option<Data>
    // where 'a:'b
    // {
    //     &self.data
    // }
    pub(crate) fn get(&self)->Option<DataPointer>{
        match &self.data{
            None=>None,
            Some(v)=>Some(v.clone())
        }
        //self.data.clone()
    }
    pub(crate) fn cache(self:&Self)->(&Option<Hash>,bool){
        (&self.flags.hash,self.flags.dirty)
    }
    pub(crate) fn alter_hash(self:&mut Self,src:Hash){
        self.flags.alter_hash(src);
    }
}
#[cfg(test)]
mod tests{
    use std::rc::Rc;

    use super::{FullNode, Node, NodePointer, ShortNode};
    #[test]
    fn new_node_test(){
        FullNode::new().add_child(1, Rc::from(Node::Short(ShortNode::new()))).unwrap_or_else(|s:&str|{
            println!("{s}");
        });
    }
    // #[test]
    // fn node_pointer_to_node_test(){
    //     let ptr:NodePointer=Rc::new(Node::Short(FullNode::new()));
    //     // if let FullNode = *ptr.as_ref(){
    //     //     println!("Successful");
    //     // }
    //     /let x=(*ptr as FullNode);
    // }
}