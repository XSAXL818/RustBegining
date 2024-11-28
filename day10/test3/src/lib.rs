use std::{fmt::{format, Debug, Display}, thread::sleep};



// 将类型转变为String字符
pub trait ToString {
    fn to_string(&self) -> String; 
}

// 使用debug的默认实现
pub trait ToStringByDefault {
    fn to_string_default(&self) -> String {
        String::from("默认实现")
    }
}

pub trait CatTrait {
    fn test2(&self);

    fn test1(&self) {
        println!("默认实现中调用：未实现的函数");
        self.test2();// self.test2调用的函数是 test2(&self)
    }
    
}

pub struct Cat {
    pub name:String,
    pub age: u8
}

impl CatTrait for Cat {
    fn test2(&self) {
        println!("test2");
    }
}

pub struct Car {
    pub version: String,
    pub name: String,
    pub price: f32,
    pub brand: String
}

pub struct Dog {
    pub name: String,
    pub host: String,
    pub age: u8
}


impl ToString for Car {
    fn to_string(&self) -> String {
        format!("Car:{{version={},\n\tname={},\n\tprice={},\n\tbrand={}}}",self.version,self.name,self.price,self.brand)
    }
}

impl ToString for Dog {
    fn to_string(&self) -> String {
        format!("Dog:{{name={},\n\thost={},\n\tage={}}}",self.name,self.host,self.age).to_string()
    }
}


impl ToStringByDefault for Dog {
    
}

impl ToStringByDefault for Car {
    fn to_string_default(&self) -> String {
        format!("Car: {{ {},{},{},{} }}",self.version,self.name,self.price,self.brand)
    }
}


pub struct HomeLand {
    pub name: String,
    pub age: u8
}

pub struct DouBao {
    pub name: String,
    pub model: String
}

pub trait MyDisplay {
    fn to_string(&self) -> String;
}

impl MyDisplay for HomeLand {
    fn to_string(&self) -> String {
        format!("name={},age={}",self.name,self.age)
    }
}

impl MyDisplay for DouBao {
    fn to_string(&self) -> String {
        format!("name={},model={}",self.name,self.model)
    }
}

// pub fn my_println(data: impl MyDisplay) {
//     println!("{}",data.to_string());
// }

pub fn my_println<T:MyDisplay>(data: T) {
    println!("{}",data.to_string());
}

// 
pub fn good1_def(data1: impl MyDisplay, data2: impl MyDisplay) {
    println!("{}",data1.to_string());
    println!("{}",data2.to_string());
}
pub fn good1<T:MyDisplay>(data1: T, data2: T) {
    println!("{}",data1.to_string());
    println!("{}",data2.to_string());
}

pub fn good2_def(data1: impl MyDisplay + Display, data2: impl MyDisplay + Display) {
    println!("{}",data1.to_string());
    println!("{}",data2.to_string());
}
pub fn good2<T:MyDisplay + Display>(data1: T, data2: T) {
    println!("{}",data1.to_string());
    println!("{}",data2.to_string());
}

pub fn notify<T: MyDisplay + Display, U: Clone + Debug> ( d1: T, d2: U ) -> String {
    format!("{},{:?}",d1,d2)
}

pub fn notify1<T: MyDisplay + Display, U: Clone + Debug> ( d1: T, d2: U ) -> String {
    format!("{},{:?}",d1,d2)
}

//where 优化
pub fn notify1_plus<T,U>( d1: T, d2: U ) -> String 
where
    T: MyDisplay + Display,
    U: Clone + Debug
{
    format!("{},{:?}",d1,d2)
}


pub struct  Struct { }

pub fn return_trait(num : i32) -> impl Display {
    if num >= 10 {
        return String::from("XXX")
    }

    808.to_string()  
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];// 此处T类型没有实现Copy，报错

    for &item in list.iter() {
        if item > largest { //  > 是std::cmp::PartialOrd这个trait
            largest = item
        }
    }

    largest
}

fn largest1<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();// 此处T类型没有实现Copy，报错

    for item in list.iter() {
        if item > &largest { //  > 是std::cmp::PartialOrd这个trait
            largest = item.clone()
        }
    }

    largest
}

fn largest2<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];// 此处T类型没有实现Copy，报错

    for item in list.iter() {
        if item > &largest { //  > 是std::cmp::PartialOrd这个trait
            largest = item
        }
    }

    largest
}

pub struct Mouse<T> {
    x: T,
    y: T
}

impl<T> Mouse<T> {
    fn new( x: T, y: T ) -> Self {
        Self {x,y}
    }
}


// impl<T: Clone + Display> Mouse<T> {
//     fn in_clone_display() {
//         println!("实现Clone和Display的类型才有")
//     }
// }



pub trait  MyToString {
    fn my_to_string(&self) -> String;
}

// 对所有实现了MyDisplay1的类型，都实现MyToString
impl <T: Display> MyToString for T {
    fn my_to_string(&self) -> String {
        format!("my_to_string={}",self)
    }
}
