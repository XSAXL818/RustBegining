fn main() {
    
    test1();
    test2();
    
}

fn test1(){
    let mut a = 3;
    {
        a = 5;
        let mut b = 4;
        println!("a = {}", a);
    }
    // b = 4;// 不可行
}

fn test2(){
    

    let mut ss = "hello";
    println!("{:p}",ss);
    ss = "world";
    println!("{:p}",ss);
    ss = "hello";
    println!("{:p}",ss);
    ss = "hell";
    println!("{:p}",ss);


    println!("-----------------");

    let mut s = String::from("hello");
    println!("{:p}",&s);
    s.push_str(", xsaxl");
    println!("{:p}",&s);
}