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

fn test5() {
    let str = "😊🥰🤣❤️😍";
    for ch in str.chars() {
        print!("{}",ch);
    }
    println!();

    // 正好第一个表情
    let spl = &str[0..4];
    // 取得的字节在第一个表情内，报错
    // let spl = &str[0..3];
    println!("切片={}",spl);   
}


fn test4() {
    let s1 = String::from("value");
    // let ch = s1[0];
    // println!("{}",ch);

    let s1 = "01234😶‍🌫️";
    println!("bytes={}",s1.len());

    let s1 = "😶‍🌫️😊🤖🎮🥇🌵🍵";
    for b in s1.bytes() {
        print!("{} ",b);
    }
    println!();

    for ch in s1.chars() {
        print!("{} ",ch);
    }
    println!();

    println!("s1={}",s1);

}


fn test3() {
    let s1 = String::from("hello");
    let s2 = String::from("value");
    let s3 = s1 + " " + &s2;
    println!("s3={}",s3);

    // println!("{},{},{}",s2,s3,s1);

    let s1 = String::from("1");
    let s2 = String::from("2");
    let s3 = String::from("3");

    let s3 = s1 + "-" + &s2 + "-" + &s3;
    println!("s3={}",s3);


    let s1 = String::from("1");
    let s2 = String::from("2");
    let s3 = String::from("3");
    println!("{}",format!("{}-{}-{}",s1,s2,s3));
    println!("{},{},{}",s1,s2,s3);




}

fn test2() {
    let mut s = String::from("hello");
    s.push_str(" ROBOT🤖");
    println!("{}",s);

    let s1 = String::from("value");
    s.push_str(&s1);
    println!("{}",s1);

    let ch = '😶';
    s.push(ch);
    println!("{}",s);
}

fn test1(){
    let s1 = String::new();

    let s1 = "str ";
    let s1 = s1.to_string();
    let s1 = "Str".to_string();

    let s1 = String::from("robot🤖");



}
