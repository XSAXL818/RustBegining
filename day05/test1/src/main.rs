use std::any::{type_name, Any};

fn main() {
    
    test1();
    println!("-------------");


    test2();
    println!("-------------");

    test3();
    println!("-------------");

    test4();
    println!("-------------");
}



fn test4(){
    struct Color(i32,i32,i32);
    struct Point3(i32,i32,i32);

    let black = Color(0,0,0);
    let center = Point3(0,0,0);
    


}



fn test3(){
    let u1 = User{
        name:String::from("小薇薇"),
        age:55
    };
    let u2 = User{
        name:u1.name,// 如果使用u1中较多字段，书写繁琐
        age:u1.age
    };

    let u2 = User{
        name:String::from("小白"),
        ..u1 
    };
    println!("name={},age={}",u2.name,u2.age);


}

fn test2(){
    let user = User(String::from("小王"),18);
    println!("name={},age={}",user.name,user.age);
}

fn User(name:String, age:i8) -> User {
    User { name, age }
}

fn test1(){
    let user1 = User{
        name:String::from("小张"),
        age:15
    };
    // user1.age = 3;// 未声明为可变
    println!("name={},age={}",user1.name,user1.age);

    let mut user2 = User{
        name:String::from("小王"),
        age:49
    };
    user2.age=34;
    user2.name=String::from("喜喜");
    println!("name={},age={}",user2.name,user2.age);

}

// 定义struct
struct User {
    name:String,
    age:i8
}
