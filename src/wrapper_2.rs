pub fn main(){
    println!("\n\nWrapper Tests:");

    println!("\ntest1:");
    test1();
    println!("\ntest2:");
    test2();

}

pub fn test1(){
    let w = Wrapper{v:vec![1,2,3,4,5,6,7,8]};
    let mut iw = w.iter();
    println!("first: {}",iw.next().unwrap());
    for el in iw{
        println!("evens: {}",el);
    }
}

pub fn test2(){
    let w = Wrapper{v:vec![11,12,13,14,15,16,17,18]};
    let mut iw = w.iter();
    println!("first: {}",iw.next().unwrap());
    for el in iw{
        println!("evens: {}",el);
    }
}

struct Wrapper{
    v:Vec<i32>,
}

struct WrapperIter{
    v:Vec<i32>,
    next_index:usize,
}

impl Wrapper{
    fn iter(&self)->WrapperIter{
        WrapperIter{v:self.v.clone(),next_index:1}
    }
}

impl Iterator for WrapperIter{
    type Item = i32;

    fn next(&mut self)->Option<i32>{
        if self.next_index < self.v.len(){
            let ret = self.v[self.next_index];
            self.next_index += 2;
            Some(ret)
        }else{
            None
        }
    }
}

