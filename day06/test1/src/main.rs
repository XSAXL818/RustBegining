
fn main() {
   
    test1();
    println!("---------");

    test2();
    println!("---------");

    test3();
    println!("---------");

}

#[derive(Debug)]
enum Msg{
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

impl Msg{
    fn call(&self){
        println!("{:?}",self);
    }
}

fn test3(){
    let msg1 = Msg::Quit;
    let msg2 = Msg::Move { x: 3, y: 5 };

    msg1.call();
    msg2.call();
}




#[derive(Debug)]
struct IP{
    version:IpAddrKind,
    addr:String
}

#[derive(Debug)]
enum EnumIP{
    V4(u8, u8, u8, u8),
    V6(String)
}


fn test2(){
    let ip1 = IP{
        version:IpAddrKind::V4,
        addr:String::from("127.0.0.1")
    };
    println!("ip1={:?}",ip1);

    // 使用枚举改善

    let ip2 = EnumIP::V4(127, 0, 0, 1);
    println!("ip2={:?}",ip2);

    let ip3 =   EnumIP::V6(String::from("::1"));

    println!("ip3={:?}",ip3);

}






fn test1(){
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;
    println!("{:?},{:?}",ipv4,ipv6);
}

#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6
}
