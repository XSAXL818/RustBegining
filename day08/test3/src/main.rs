use std::{collections::HashMap, default, hash::Hash, iter::zip};

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
    let text = "blue red blue green yellow";

    let mut map = HashMap::new();
    let default = 0;
    for word in text.split_whitespace() {
        // 返回key=word的那个实体entry，如果不存在默认插入<key,0>。返回值为该entry的V的可变引用
        let cnt = map.entry(word).or_insert(default);
        *cnt += 1;
    }
    for (k,v) in &map {
        println!("{},{}",k,v);
    }


}

fn test4() {
    let v1 = vec![String::from("数学"),String::from("英语"),String::from("语文")];
    let v2 = vec![95,74,88];
    let mut map:HashMap<_,_> = v1.iter().zip(v2.iter()).collect();
    for (k,v) in &map {
        println!("{},{}",k,v);
    }
    println!();

    let key = String::from("数学");
    let new_score = 96;
    map.insert(&key, &new_score);

    for (k,v) in &map {
        println!("{},{}",k,v);
    }
    println!();

    let mut map = HashMap::new();
    map.insert(String::from("test"), 99);

    let en = map.entry(String::from("test"));
    println!("entry={:?}",en);
    en.or_insert(10);

    let en = map.entry(String::from("run"));
    println!("entry={:?}",en);
    let x=  en.or_insert(100);
    *x = 999;
    for (k,v) in &map {
        println!("{},{}",k,v);
    }
    

}


fn test3() {
    let mut map = HashMap::new();
    map.insert(String::from("数学"), 95);
    map.insert(String::from("英语"), 74);

    let k = String::from("数学");

    let ret = map.get(&k);
    match ret {
        Some(v) => println!("{}",v),
        None => println!("None")
    };


    for (k,v) in &map {
        println!("{}:{}",k,v);
    }


}

fn test2() {

    let key = String::from("key");
    let value = String::from("value");
    let mut map = HashMap::new();
    map.insert(key, value);

    // println!("{},{}",key,value);

    let key = String::from("key");
    let value = String::from("value");
    let mut map = HashMap::new();
    map.insert(&key, &value);
    println!("{},{}",key,value);


    let mut map = HashMap::new();
    {
        let key = String::from("key");
        let value = String::from("value"); 
        map.insert(&key, &value);
    }
    // for (k,v) in &map {
    //     println!("{},{}",k,v);
    // }
}


fn test1(){
    let mut scores:HashMap<String,u8> = HashMap::new();

    scores.insert(String::from("数学"), 95);
    scores.insert(String::from("英语"), 73);


    let course = vec!["数学","英语"];
    let scores  = vec![95,73];
    
    let cs:HashMap<_,_> = course.iter().zip(scores.iter()).collect();
    

    for (k,v) in &cs {
        println!("{}={}",k,v);
    }

}
