pub fn main(){
    println!("\n\nBasic Sum Box Tests:");

    println!("\ntest1:");
    test1();
    println!("\ntest2:");
    test2();

}

pub fn test1(){
    let s = vec!["asd".to_string(), "where".to_string(), "what".to_string()];
    println!("boxed s: {:?}", basicbox_sum(s));
}

pub fn test2(){
    let s = vec!["nope".to_string(), "game".to_string(), "bananas".to_string()];
    println!("boxed s: {:?}", basicbox_sum(s));
}

pub fn basicbox_sum(vec:Vec<String>)->Vec<Box<usize>>{
    let mut ret = Vec::new();
    let mut sum = 0;
    for el in vec{


        ret.push(Box::new(el.len()));
        sum+=el.len();
    }
    ret.push(Box::new(sum));
    ret
}