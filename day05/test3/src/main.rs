fn main() {
    
    let rect = Rectangle{
        len:5,
        wid:5
    };
    println!("{:?},eara={}",rect,rect.eare());

    println!("{},{}",rect.eare(),(&rect).eare());

    let rect1 = Rectangle{
        len: 9,
        wid: 10
    };

    let rect2 = Rectangle{
        len: 7,
        wid: 7
    };

    println!("{:?},{:?},前者是否可以包容后者: {}",rect1,rect2,rect1.is_contain(&rect2));

    println!("关联函数：{:?}",Rectangle::square(5));
}

// 优化原先计算长方形结构的面积
// 由于计算eare函数只适用于长方形，可以将其绑定
#[derive(Debug)]
struct Rectangle{
    len:u32,
    wid:u32
}

impl Rectangle{
    fn eare(&self) -> u32 {
        return self.len * self.wid;
    }
    fn is_contain(&self,other:&Rectangle) -> bool {
        if self.len > other.len && self.wid > other.wid{
            return true;
        }
        false
    }

    fn square(size:u32) -> Rectangle{
        Rectangle{
            len:size,
            wid:size
        }
    }
}
