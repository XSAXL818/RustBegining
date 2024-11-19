

// 计算长方形的面积
fn main() {

    let r1 = Rectangle{
        len:5,
        wid:5
    };
    println!("area={}",get_area(&r1));
    println!("{:?}",r1);
}

// 需要注意使用引用，不使用的话，main则不再具有r1的所有权
fn get_area( r:&Rectangle ) -> i32 {
    r.len * r.wid
}

// 自己写一下试试
#[derive(Debug)]
struct Rectangle{
    len : i32,
    wid: i32
}
