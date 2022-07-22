struct LinkedList{
    head: Option<Box<Node>>,
    size: usize,
}

struct Node{
    value: u32,
    /* cant use node directly, 
    as that will be borrowing */ 
    next: Option<Box<Node>>, 
}

impl Node {
    pub fn new(value: u32, next: Option<Box<Node>>)-> Node{
        Node{value: value,next: next}
    }
    
}

impl LinkedList{
    pub fn new()->LinkedList{
        LinkedList { head: None, size:0 }
    }

    pub fn get_size(&self)->usize{
        self.size
    }
}

fn main() {

    let x: Box<u32> = Box::new(10); //box puts value to heap
    //auto deference, x is same to *x
    println!("{}", x);
    println!("{}", *x); 
    
}
