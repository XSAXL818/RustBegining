
const MAX_LENTH:u32 = 100_000;

fn main() {
    
    let const_x = 4;
    // const_x = 5;// 报错

    println!("{}",const_x);

    let mut x = 4;
    println!("{}",x);
    x = 5;
    println!("{}",x);

    println!("最大长度为{}",MAX_LENTH);


    // Shadowing隐藏机制
    let x1 = 4;
    // x1 = x1+1;// 报错
    let x1 = x1+1;
    println!("Shadowing:{}",x1);
    let x1 = "可以是不同的类型";
    println!("Shadowing:{}",x1);

    let mut x2 = "字符串类型";
    // x2 = str.len();// 不可以更改类型
    let mut x2 = 3;

    println!("{}",x2);

}
