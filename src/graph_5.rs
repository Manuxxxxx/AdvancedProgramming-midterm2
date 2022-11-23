use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;


pub fn main(){
    test1();
    test2();
    test3();
    test4();
}

pub fn test1(){
    let mut el1 = Content{i:10, b:true};
    let mut el2 = Content{i:11, b:true};
    let mut el3 = Content{i:11, b:false};
    assert_eq!(el1<el2,true);
    assert_eq!(el2<el1,false);
    assert_eq!(el2==el3,true);
}


pub fn test2(){
    let mut el1 = Content{i:10, b:true};
    let mut el2 = Content{i:11, b:true};
    let mut el3 = Content{i:11, b:false};
    assert_eq!(el1.samebool(&el2), true);
    assert_eq!(el1.samebool(&el3), false);
}

pub fn test3(){
    let mut g : Graph<Content> = Graph::new();
    println!("{:?}",g);
}

pub fn test4(){
    let mut el1 = Content{i:10, b:true};
    let mut el2 = Content{i:11, b:true};
    let mut el3 = Content{i:12, b:false};
    let mut g = Graph::new();
    println!("{:?}",g);
    g.add_node(el1);
    println!("{:?}",g);
    g.add_node(el2);
    println!("{:?}",g);
    g.add_node(el3);
    println!("{:?}",g);

    let mut el1 = Content{i:10, b:true};
    let mut el2 = Content{i:8, b:false};
    let mut g = Graph::new();
    println!("{:?}",g);
    g.add_node(el1);
    println!("{:?}",g);
    g.add_node(el2);
    println!("{:?}",g);

    let mut el1 = Content{i:10, b:true};
    let mut el2 = Content{i:11, b:true};
    let mut el3 = Content{i:12, b:true};
    let mut g = Graph::new();
    println!("{:?}",g);
    g.add_node(el1);
    println!("{:?}",g);
    g.add_node(el2);
    println!("{:?}",g);
    g.add_node(el3);
    println!("{:?}",g);

    let mut el1 = Content{i:10, b:true};
    let mut el2 = Content{i:9, b:false};
    let mut el3 = Content{i:8, b:true};
    let mut g = Graph::new();
    println!("{:?}",g);
    g.add_node(el1);
    println!("{:?}",g);
    g.add_node(el2);
    println!("{:?}",g);
    g.add_node(el3);
    println!("{:?}",g);
}


type NodeRef<T> = Rc<RefCell<Node<T>>>;
struct Node<T> {
    inner_value: T,
    adjacent: Vec<NodeRef<T>>,
}
impl<T:Debug> Debug for Node<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"iv: {:?}, adj: {}", self.inner_value, self.adjacent.len())
    }
}

#[derive(Debug)]
struct Graph<T> {
    nodes: Vec<NodeRef<T>>,
}
pub trait SameBool{
    fn samebool(&self, other:&Self)->bool;
}

#[derive(Debug)]
pub struct Content{
    pub i:i32,
    pub b:bool
}

impl Content {
    pub fn new_with(i: i32, b: bool) -> Content {
        Content { i, b }
    }
}

impl<T> Graph<T>{
    fn new()->Graph<T>{
        Graph{nodes:Vec::new()}
    }


}

impl<T:SameBool + PartialOrd> Graph<T>{
    fn add_node(&mut self, val:T){
        let mut node = Rc::new(RefCell::new(Node{inner_value:val, adjacent:Vec::new()}));

        for n in self.nodes.iter(){
            if n.borrow().inner_value.samebool(&node.borrow().inner_value){
                n.borrow_mut().adjacent.push(node.clone());

            }

            if n.borrow().inner_value < node.borrow().inner_value{
                node.borrow_mut().adjacent.push(n.clone());
            }
        }


        self.nodes.push(node);
    }
}



impl SameBool for Content{
    fn samebool(&self, other:&Self)->bool{
        self.b == other.b
    }
}

//Two Contents can be compared (<, >, ==) by comparing their i32 field
impl PartialEq for Content{
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
    }


}

impl PartialOrd for Content{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.i.partial_cmp(&other.i)
    }
}