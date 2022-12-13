pub fn main(){
    println!("\n\nDoublable Tests:");

    println!("\ntest1:");
    test1();
    println!("\ntest2:");
    test2();

}

pub fn test1(){
    let x = 5;
    printdouble(x);
    let s = "what".to_string();
    println!("normal s: {:?}",s);
    printdouble(s);
    let y = 8;
    printdouble(y);
}

pub fn test2(){
    let x = 10;
    printdouble(x);
    let s = "who".to_string();
    println!("normal s: {:?}",s);
    printdouble(s);
    let y = 8;
    printdouble(y);
}

trait Doublable{
    fn gimme_double(&self)->Self;
}

impl Doublable for i32{
    fn gimme_double(&self)->Self{
        self*2
    }
}

impl Doublable for String{
    fn gimme_double(&self)->Self{
        self.clone() + &self.clone()
    }
}

fn printdouble<T:Doublable + std::fmt::Debug>(t:T){
    println!("doubling {:?} is {:?}",t,t.gimme_double());
}
