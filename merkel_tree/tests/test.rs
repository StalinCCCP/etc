#[cfg(test)]
mod tests {

    use merkel_tree::MerkelTree;
    #[test]
    fn merkel_tree_test(){
        let str=vec![String::from("testing"),String::from("hello"),String::from("It's almost done")];
        let out=if MerkelTree::build(&str).verify() {"success"}else{"unsuccess"};
        println!("{out}");
    }
    use std::boxed::Box;
    fn reref_test(){


        //let y=&x;
        //y=&z;
    }
}