use std::{clone, ops::Index};



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

    test6();
    println!("--------");
}
#[derive(Debug)]
struct Point{
    point: [i32;3],
    index: u8
}
impl Point {
    fn new() -> Point {
        Point {
            point: [1,100,200],
            index: 0,
        }
    }
}

impl Iterator for Point {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {

        let x:usize = (self.index).into();
        if x < 3 {
            self.index += 1;
            Some(self.point[x])
        } else {
            self.index = 0;
            None
        }
        
    }
}



fn test6() {
    println!("遍历坐标系：");
    let pp = Point::new();
    // 实现next后，默认提供into_iter方法，交出所有权
    for i in pp.into_iter() {
        println!("{}",i);
    }
    // println!("{:?}",pp);

    // let v = vec![1,2,3];
    // for i in v {
    //     println!("{}",i);
    // }
}

#[derive(Debug,PartialEq)]
struct NiuMa {
    age: u8,
    school: String
}

impl NiuMa {
    fn new(age:u8,school:String) -> NiuMa {
        NiuMa {
            age,
            school
        }
    }
}
// 会将vec的所有权拿走，建议传入迭代器
fn get_niu_ma(niu_ma:Vec<NiuMa>) -> Vec<NiuMa> {
    niu_ma.into_iter().filter(|x| {
        x.age < 35 && x.school != "孀妃"
    }).collect()
}

fn get_niu_ma_it<I:Iterator<Item = NiuMa>>(it:I) -> Vec<NiuMa> {
    it.filter(|x| {
        x.age < 35 && x.school != "孀妃"
    }).collect()
}

fn test5() {
    let v = vec![NiuMa::new(22, String::from("孀妃")),
        NiuMa::new(23, String::from("二妖妖")),
        NiuMa::new(36, String::from("酒吧舞")),
    ];

    let niu_ma:Vec<&NiuMa> = v.iter().filter(|x| {
        x.age < 35 && x.school != "孀妃"
    }).collect();

    let it = v.iter().filter(|x| {
        x.age < 35 && x.school != "孀妃"
    });

    
    // for i in v.iter() {
    //     println!("{:?}",i);
    // }

    // let niu_ma1 = get_niu_ma(v);

    println!("符合我公司的人财：");
    for i in niu_ma.iter() {
        println!("{:?}",i);
    }

    println!("通过迭代器：");
    for i in it {
        println!("{:?}",i);
    }

    println!("通过函数(迭代器传参)：");
    // 上面有引用，不能在上面调用
    let niu_ma3 = get_niu_ma_it(v.into_iter());
    for i in niu_ma3.iter() {
        println!("{:?}",i);
    }
    
}

fn test4() {
    let v1 = vec![1,2,3,4];
    let it = v1.iter();
    // 使用map获取一个新的迭代器，使用collect并显式指定类型，来获取指定类型的Vec
    // collect是一个消耗型的适配器，一般用于链式调用中
    let v2:Vec<String> = it.map(|x| format!("{}",x) ).collect();
    for elem in v2.iter(){
        println!("{:?}",elem);
    }

}

fn test3() {
    let v = vec![1,2,3,4];
    let it = v.iter();
    let mut sum:i32 = it.sum();
    println!("sum={}",sum);

    // sum = it.sum();// 上一次sum调用，已经耗尽该iter了

    
}


fn test2() {
    let mut v = vec![1,2,3,4,5];
    let mut it = v.iter();
    loop {
        match it.next() {
            Some(ret) => println!("ret={}",ret),
            None => break
        }
    }

    // for i in v.into_iter() {
        
    // }
    // println!("{:?}",v);

    for i in v.iter_mut() {
        *i = 10;
    }
    for i in v.iter() {
        println!("{}",i);
    }
}

fn test1(){
    let v = vec![1,2,3,4,5];
    let it = v.iter();
    for elem in it {
        println!("{}",elem);
    }
    let it = v.iter();
}
