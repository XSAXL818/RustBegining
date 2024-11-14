use std::cmp::Ordering;



fn main() {
    let x = 3;
    const MAX:i32 = 6;
    if x > 3{
        println!("x is greater than 3");
    } else {
        println!("x is less than or equal to 3");
    }
    
    // 使用match重构
    match x.cmp(&6){
        Ordering::Less => println!("x is less than 6"),
        Ordering::Equal => println!("x is equal to 6"),
        Ordering::Greater => println!("x is greater than 6"),
    }


    let mut cnt = 1;
    loop{
        println!("Hello, world!");
        if cnt == 5{
            break;
        }
        cnt += 1;
    }

    let mut cnt = 1;
    while cnt != 6{
        println!("Hello, world!");
        cnt += 1;
    }

    let arr = [10, 20, 30, 40, 50];
    for e in arr.iter(){
        println!("The value is: {}", e);
    }

    for i in (1..4).rev(){
        println!("{}!", i);
    }
    
}
