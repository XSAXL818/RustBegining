fn main() {
    
    test1();
    println!("------");


}

fn test1(){
    let some_num = Some(5);
    let some_str = Some(String::from("value"));

    let none_num:Option<u32> = None;

    let sum = 4 + some_num.unwrap();
}
