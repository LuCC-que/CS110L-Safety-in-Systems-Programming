use std::fmt::{write, self};

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
        Node{value: value, next: next}
    }
    
}

impl LinkedList{
    pub fn new()->LinkedList{
        LinkedList { head: None, size:0 }
    }

    pub fn get_size(&self)->usize{
        self.size
    }

    pub fn is_empty(&self)->bool{
        self.size == 0
    }

    pub fn push(&mut self, value:u32){
        //take() Takes the value out of the option, leaving a None in its place.
        let new_node : Box<Node> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);

        //another way to do it is to initialize new_node as Box<Node>, 
        //and also wrap the new_node with Some()

        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<u32>{
        //? return None if None
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)

    }

    pub fn display(&self){
        let mut current: &Option<Box<Node>> = &self.head;
        let mut result = String::new();
        
        loop {
            match current{
                Some(node) =>{
                    //do  things

                    result = format!("{} {}", result, node.value);

                    //option is a enum that has null and Some(val)
                    //it self doesnt contain information about val
                    //so have to use node instead of Option<Node<>> 
                    current = &node.next;
                },
                None => break, // no more next
            }
        }

        println!("{}", result);

    }
}

/*
    defining traits
*/
impl fmt::Display for LinkedList{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        let mut current : &Option<Box<Node>> = &self.head;
        let mut result = String::new();

        loop{
            match current {
                Some(node)=>{
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

/* how it really drop the memory */
/*
* when moving to next value, current get replaced
* to another value, compiler automatically 
* drop that value off
*/
impl Drop for LinkedList{
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current{
            
            //moving to next value, the current out of scope
            //then the drop occurs
            current = node.next.take();

            
        }
    }
}


fn main() {

    let x: Box<u32> = Box::new(10); //box puts value to heap
    //auto deference, x is same to *x
    println!("test without ast{}", x);
    println!("test with ast{}", *x); 

    let mut list : LinkedList = LinkedList::new(); 

    //test if it is empty
    //function aliasing
    println!("{}", LinkedList::is_empty(&list));
    //self connection
    println!("{}", list.is_empty());

    //example of take()
    let mut x: Option<u32> = Some(5);
    let x_ref: &mut Option<u32> = &mut x;
    //take() Takes the value out of the option, leaving a None in its place.
    println!("result of take: {:?}", x_ref.take());
    println!("left at x: {:?}", x);

    for i in 1..10{
        list.push(i); //have to declare as mut first
    }
    list.display();

    println!("now the size is {}", list.get_size());
    println!("Top element: {}", list.pop().unwrap()); //unwrap remove the some
    print!("what is the size{}", list.get_size());
    list.display();

    println!("using the trait to print : {}", list);

    
}
