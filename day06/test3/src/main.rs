fn main() {
    
    test1();
    println!("-------");


    test2();
    println!("-------");

    test3();
    println!("-------");

    test4();
    println!("-------");

}

fn test4(){
    match 5 {
        _ => 1
    };

    let num = 16;
    // 只想知道是不是4的倍数
    match num%4 {
        0 => println!("Yes"),
        _ => ()
    };

    // 只针对一种模式匹配，使用if let 简化
    if let 0 = num%4 {
        println!("Yes");
    };

    let ret = if let 0 = num%4 {
        0
    } else {
        1
    };
    println!("ret={}",ret);


}

fn test3(){
    let x = Some(32);
    println!("{:?}",add_one(x));

    let x : Option<i32> = None;
    println!("{:?}",add_one(x));


}

// 传入一个option，如果不为None则自增1
fn add_one(x : Option<i32> ) -> Option<i32>{
    match x {
        Option::None => None, // 可以使用Option::获取，也可以直接写
        Some(num) => Some(num+1)
    }
}


fn test2(){

    let coin = NewCoin::Ten(Province::Henan);
    let ret = match coin {
        NewCoin::One => 1,
        NewCoin::Two => 2,
        NewCoin::Five => 5,
        NewCoin::Ten(province) => {
            println!("from {:?}",province);
            10
        }
    };
    println!("ret={}",ret);

}

#[derive(Debug)]
enum Province{
    Henan,
    Shaanxi
}

enum NewCoin{
    One, 
    Two,
    Five,
    Ten(Province)
}

enum Coin{
    One,
    Five,
    Ten,
    Twenty
}

fn test1(){

    let one = Coin::One;
    println!("{}",value_in_cents(&one));


    let ret = match one {
        Coin::One => {
            println!("一元");
            1
        }
        Coin::Five => 5,
        Coin::Ten => 10,
        Coin::Twenty => 20
    };

    println!("ret={}",ret);
}





fn value_in_cents(coin:&Coin) -> u8 {
    match coin {
        Coin::One => 1,
        Coin::Five => 5,
        Coin::Ten => 10,
        Coin::Twenty => 20
    }
}