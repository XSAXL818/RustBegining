fn main() {
    
    test1();
    println!("------------------------");

    test2();
    println!("------------------------");
    
    test3();
    println!("------------------------");

    test4();
    println!("------------------------");

    test5();
    println!("------------------------");

    test6();
    println!("------------------------");
}

fn test6(){

}

fn move_to_me(str:String){
    println!("当前字符串被移动了：{}",str);
}

fn test5(){
    let x = String::from("value");
    move_to_me(x);
    // println!("{}",x);// x已经失效了
}

fn test4(){
    let x = String::from("heihei");
    let y = x.clone();
    println!("x = {} , y = {}",x,y);
}

fn test3(){
    let x = 3;
    let y = x;
    println!("x = {}", x);

    let x = String::from("heihei");
    println!("{:p}",&x);
    let y = x;
    println!("{:p}",&y);
    // println!("x = {}", x);
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