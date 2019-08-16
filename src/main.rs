use std::fmt;

fn main(){
    test();
}

struct Node{
    val: usize,
    lch: Option<Box<Node>>,
    rch: Option<Box<Node>>,
}

impl Node{
    fn new(init: usize) -> Node{
        Node {
            val: init,
            lch: None,
            rch: None
        }
    }

    fn insert(self: &mut Node,x: usize){
        if(x < self.val){
            // insert to lch
            match &mut self.lch{
                None => {
                    self.lch = Some(Box::new(Node::new(x)))
                } 
                Some(lch) => {
                    lch.insert(x);
                } 
            }
        }else{
            match &mut self.rch{
                None => {
                    self.rch = Some(Box::new(Node::new(x)))
                } 
                Some(rhc) => {
                    rhc.insert(x);
                } 
            }
        }
    }
}

impl ToString for Node{
    fn to_string(&self) -> String{
        let lch_str = match &self.lch {
            None => String::from(""),
            Some(lch) => lch.to_string()
        };
        let rch_str = match &self.rch {
            None => String::from(""),
            Some(rch) => rch.to_string()
        };

        format!("{}-(l{} r{})",
        self.val,lch_str,rch_str)
    }
}

fn test(){
    let mut node = Node::new(2);
    node.insert(1);
    node.insert(3);
    node.insert(6);
    node.insert(7);
    node.insert(5);

    println!("{}", node.to_string());
}