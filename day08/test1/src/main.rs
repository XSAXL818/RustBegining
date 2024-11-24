fn main() {

    test1();
    println!("--------");

    test2();
    println!("--------");

    test3();
    println!("--------");

    test4();
    println!("--------");

    test5();
    println!("--------");
}


#[derive(Debug)]
enum Box {
    Int(i32),
    Float(f64),
    Str(String)
}

fn test5() {
    let v = vec![
        Box::Int(3),
        Box::Float(3.14),
        Box::Str(String::from("3.1415926")),
    ];

    for i in &v {
        match i {
            Box::Int(data) => println!("{}",data),
            Box::Float(data) => println!("{}",data),
            Box::Str(data) => println!("{}",data),
        }
    }

}


fn test4() {
    let v = vec![1,2,3,6,6];
    // for i in v.iter() {
    //     print!("{} ",i);
    // }
    for i in &v {
        print!("{} ",i);
    }
    println!();

    let mut v = vec![1,2,3,6,6];
    for i in &mut v {
        *i += 10;
    }
    for i in &v {
        print!("{} ",i);
    }
    println!();


}

fn test3() {
    let mut v = vec![1,2,3,4,5];
    let elem1 = &v[0];// 不可变引用

    // v.push(1);// 可变引用

    println!("elem1={}",elem1);
}




fn test2() {
    let v = vec![0,1,2,3,4,5,6];
    let index = 2;
    

    println!("index={},elem={}",index,v[index]);

    let r = &v[2];

    let index = 7;
    match v.get(index) {
        Some(r) => println!("index={},elem={}",index,r),
        None => println!("index={},no elem",index)
    }

}


fn test1(){
    let vec:Vec<i32> = Vec::new();


    let v = vec![1,2,3];

    let mut v = Vec::new();
    v.push("hello");

}




