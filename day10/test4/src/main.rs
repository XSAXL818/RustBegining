fn main() {
    
    test1();
    println!("--------");

    test2();
    println!("--------");

    test3();
    println!("--------");
}

struct Stu<'a> {
    name: &'a str,
}

impl<'a> Stu<'a> {
    fn name(&self) -> &str {
        &self.name
    }

    fn level(&self, age: &str ) -> &str {
        println!("{}",age);
        &self.name
    }
}


// fn first_word<'a>(s:& str) -> &str {
//     let bytes = s.as_bytes();
//     for (i,item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

fn test3() {
    let baby = "小花猫";
    let mother = Cat{baby};

}

struct Cat<'a> {
    baby: &'a str
}

fn test1(){
    // let x;
    // {
    //     let a = 3;
    //     x = &a;
    // }
    // println!("{}",x);
    let str1 = String::from("US");
    let str2 = "China";
    println!("longest={}",longest(str1.as_str(), str2));
}

fn test2() {
    let str1 = String::from("China");
    let ret;
    {
        let str2 = String::from("Us");
        ret = longest(str1.as_str(), str2.as_str());
    }
    // println!("{}",ret);
}

fn longest<'a> (str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1
    }
    str2
}
// fn longest1<'a> (str1: &'a str, str2: & str) -> &'a str {
//     if str1.len() > str2.len() {
//         return str1
//     }
//     str2
// }
// fn longest2() -> &str {
//     let x = String::from("123124");
//     x.as_str()
// }
// fn longest1(str1: &str, str2: &str) -> &str {
//     str1
// }

fn return_pri() -> String {
    let x = String::from("value");
    x
}

