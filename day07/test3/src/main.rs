use std::{
    dbg,
    collections::{
        HashMap,
        HashSet as Set,
        BTreeMap
    }
};

use std::io;
use std::io::Write;

// use std::io::{self,Write};



fn main() {
    

    test1();
    println!("--------");


    test2();
    println!("--------");

    test3();
    println!("--------");
    

}

fn test3() {
    
    use std::collections::*;
    let mut x = LinkedList::new();
    x.push_back("elt");


}

fn test2() {

    // 假设xxx是一个自己常用的工具库
    mod xxx {
        use test3;
        // xxx里含有test3所有的 pub 修饰的结构
        // 同时将这些条目修改为private，不供外部访问
        
    }
    // xxx::test3::nwnu();// 报错

    mod xxx_pub {
        pub use test3;
        // 将test3的所有pub结构引入，
        // 并修饰为pub，外部可以访问
    }
    xxx_pub::test3::nwnu();

}


fn test1(){
    let mut map = HashMap::new();
    map.insert(1, 3);

    let mut set = Set::new();
    set.insert("value");
}
