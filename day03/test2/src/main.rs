fn main() {
    
    let str = "321";
    // let num = str.trim().parse().expect("不是一个数字");
    let num:i32 = str.trim().parse().expect("不是一个数字");
    println!("{}",num);


    let emoji = '🦀';
    println!("emoji字符：{}",emoji);


    let tup = (1,2,3);
    let tup: (i64, f64) = (44, 22.32);// 显示指定类型
    
    let (x,y )= tup;

    println!("元组：{} {}",x,y);

    println!("元组：{} {}",tup.0,tup.1);

    let a:[i32;5] = [1,2,3,4,5];

    let a = [3;4];

    function(x);
    let x = {
        println!("代码块");
        x+3 //  不使用分号;结尾，那么这是一个表达式，会作为代码块{}的返回值。
    };
    println!("X:{}",x);

    let x1 = 3;
    let x2 = 4;
    let x3 = if_nq_add_two(x1,x2);
    println!("x:{}",x3);

    let x1 = 3;
    let x2 = 3;
    let x3 = if_nq_add_two(x1,x2);
    println!("x:{}",x3);


}

fn function(x:i64){
    println!("传入的实参为：{}",x);
}

fn if_nq_add_two(x:i32,y:i32)->i32{
    if( x == y ){
        return x
    }
    x+y
}