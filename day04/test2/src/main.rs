fn main() {

    // 不可变引用
    test1();
    println!("------------------");

    // 可变引用
    test2();
    println!("------------------");


    test3();
    println!("------------------");

    test4();
    println!("------------------");

    test5();
    println!("------------------");
    
}

// fn return_ref() -> &String{
//     let x = String::from("value");
//     &x
// }

fn test5(){
    let mut x = String::from("value");
    let y = &x;
    let yy = &mut x;
    yy.push_str("string");

    // println!("{},{}",y,yy); // 出现可变和不可变后不能使用
}

// {}作用域使用多个可变引用
fn test4(){
    let mut x = String::from("value");

    {
        let y = &mut x;
        y.push_str("string");
    }
    println!("{}",x);

    {
        let yy = &mut x;
        yy.push_str("string");
    }
    println!("{}",x);



}


fn get_len(str:&String) -> usize{
    // str.push_str("string");// 默认是不可变的
    str.len()
}
fn test1(){
    let s1 = String::from("value");
    let len = get_len(&s1);
    println!("所有权未移动:{},{}",s1,len);
}

fn get_len1(str: &mut String) -> usize{
    str.push_str("string");
    str.len()
}

fn test2(){
    let mut s = String::from("hello");
    let len = get_len1(&mut s);
    println!("可变引用:{},{}",s,len);
}

// 可变引用的独占性
fn test3(){
    let mut x = String::from("value");
    let x1 = &mut x;
    let x2 = &mut x;

    // println!("{},{}",x1,x2);

}