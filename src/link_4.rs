pub fn main(){
    println!("\n\nLinked List Tests:");

    println!("\ntest0:");
    test0();
    println!("\ntest1:");
    test1();
    println!("\ntest2:");
    test2();
    println!("\ntest3:");
    test3();
    println!("\ntest4:");
    test4();

}

pub fn test0(){
    let l : List<i32> = List::new();
    println!("{:?}",l);
    assert_eq!(l.size(),0);
    let l = List{ head: Some(Box::new(Node{ elem: 4, next: None })), len: 1 };
    assert_eq!(l.size(),1);
    let s : String = format!("{:?}",l);
    assert_eq!(s.contains("Vec"),false);
}

pub fn test1(){
    let elem1 = 4;
    let elem2 = 10;
    let elem3 = 20;
    let mut l: List<i32> = List{head:None,len:0};
    assert_eq!(l.get(0),None);
    assert_eq!(l.get(1),None);
    let mut l : List<i32> = List{head:Some(Box::new(Node{elem:elem1,next:Some(Box::new(Node{elem:elem2,next:Some(Box::new(Node{elem:elem3,next:None}))}))})), len:3};
    assert_eq!(l.get(0),Some(&4));
    assert_eq!(l.get(5),None);
    assert_eq!(l.get(2),Some(&20));
}

pub fn test2(){
    let elem1 = Content::new_with("what".to_string(),true,2);
    let elem2 = Content::new_with("this".to_string(),false,8);
    let elem3 = Content::new_with("dope".to_string(),true,18);
    let mut l : List<Content> = List{head:Some(Box::new(Node{elem:elem1,next:Some(Box::new(Node{elem:elem2,next:Some(Box::new(Node{elem:elem3,next:None}))}))})), len:3};
    println!("{:?}",l);
    l.remove(0);
    println!("{:?}",l);
    l.remove(6);
    println!("{:?}",l);
    l.remove(1);
    println!("{:?}",l);
    l.remove(0);
    println!("{:?}",l);
}

pub fn test3(){
    let elem1 = Content::new_with("what".to_string(),true,2);
    let elem2 = Content::new_with("this".to_string(),false,8);
    let elem3 = Content::new_with("dope".to_string(),true,18);
    let mut l : List<Content> = List{head:Some(Box::new(Node{elem:elem1,next:Some(Box::new(Node{elem:elem2,next:Some(Box::new(Node{elem:elem3,next:None}))}))})), len:3};
    println!("{:?}",l);
    l.pop();
    println!("{:?}",l);
    l.pop();
    println!("{:?}",l);
    l.pop();
    println!("{:?}",l);
    l.pop();
    println!("{:?}",l);
}

pub fn test4(){
    let elem1 = Content::new_with("what".to_string(),true,2);
    let elem2 = Content::new_with("this".to_string(),false,8);
    let elem3 = Content::new_with("dope".to_string(),true,18);
    let mut l : List<Content> = List{head:Some(Box::new(Node{elem:elem1,next:Some(Box::new(Node{elem:elem2,next:Some(Box::new(Node{elem:elem3,next:None}))}))})), len:3};
    println!("{:?}",l);
    l.pop_last();
    println!("{:?}",l);
    l.pop_last();
    println!("{:?}",l);
    l.pop_last();
    println!("{:?}",l);
    l.pop_last();
    println!("{:?}",l);
}

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    len: i32,
}
type Link<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}
#[derive(Debug)]
pub struct Content {
    s : String, b : bool, i : i32,
}
impl Content {
    pub fn new_with(s:String, b:bool, i:i32) -> Content {
        return Content{s,b,i};
    }
}

use std::fmt::Pointer;
impl<T> List<T> {

    pub fn new() -> Self {
        List { head: None, len: 0 }
    }

    pub fn size(&self) -> i32 {
        self.len
    }

    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            self.len -= 1;
        }
    }

    pub fn pop_last(&mut self) {
        let mut node:&mut Option<Box<Node<T>>> = &mut self.head;

        loop{
            match node {
                None=>{
                    break;
                }
                Some(cur)=>{
                    if cur.next.is_some(){
                        if cur.next.as_ref().unwrap().next.is_none() {
                            cur.next = None;
                            self.len -= 1;
                            break;
                        }else{
                            node = &mut cur.next;
                        }
                    }else {
                        self.head = None;
                        self.len -= 1;
                        break;
                    }
                }
            }


        }
    }

    pub fn remove(&mut self, indexToRem:i32)->Result<(),String>{
        let mut node = &mut self.head;
        let mut counter = 0;
        loop{
            match node {
                None=>{
                    Err::<T, String>("Wrong position".to_string());
                    break;

                }
                Some(cur) if counter==indexToRem =>{
                    *node = cur.next.take();
                    self.len -= 1;
                    break;
                }
                Some(cur)=>{
                    node = &mut cur.next;
                }
            }
            counter +=1;
        }

        return Ok(());


    }

    pub fn get(&self, index:i32)->Option<&T>{
        let mut node = &self.head;
        let mut counter = 0;
        loop{
            match node {
                None=>{
                    return None;
                }
                Some(cur) if counter==index =>{
                    return Some(& cur.elem);
                }
                Some(cur)=>{
                    node = &cur.next;
                }
            }
            counter +=1;
        }
    }
}
