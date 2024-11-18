fn main() {
    
    test1();
    println!("-----------");


    test2();
    println!("-----------");

    test3();
    println!("-----------");

    test4();
    println!("-----------");

    test5();
    println!("-----------");

}

fn test5(){
    let x = [0,1,2,3,4,5];
    let sli = &x[..4];
    for i in sli {
        print!("{},",i);
    }
    println!();


}

fn test4(){
    let x = String::from("value");
    println!("len={}",get_len(&x[..]));

    let x = "12314";
    println!("len={}",get_len(&x[..]));
}

fn get_len(s : &str) -> usize{
    s.len()
}

fn test3(){
    let s = String::from("0123 567");
    let sli = get_plus(&s);

    // s.push_str("string");// 前面出现了不可变引用，
    // fn push_str(&mut self)
    println!("切片={}",sli);
}


fn get_plus(s : &String) -> &str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}


fn test2(){
    let x = "01234 6789";
    let pre = &x[0..5];
    let last = &x[6..10];

    println!("pre={},last={}",pre,last);

}


fn test1(){
    let s1 = String::from("value");
    get_first_world_index(&s1);

    let s1 = String::from("有中文");
    get_first_world_index(&s1);
    
    let cnt = s1.chars().count();
    println!("字符个数:{}",cnt);
}

fn get_first_world_index(str : &String) -> usize {
    
    let bytes = str.as_bytes();
    println!("len={},bytes={}",str.len(),bytes.len());
    for (i , &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    return str.len()
}
