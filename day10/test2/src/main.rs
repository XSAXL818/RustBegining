fn main() {
    

    test1();
    println!("--------");


}

struct Point<T> {
    x:T,
    y:T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 实现特定的T
impl Point<i32> {
    fn y(&self) -> &i32 {
        &self.y
    }
}

#[derive(Debug)]
struct PointTwo<T,U> {
    x: T,
    y: U,
}

impl<T,U> PointTwo<T,U> {

    fn mix<V,W>(self,other:PointTwo<V,W>) -> (PointTwo<T,W>,PointTwo<V,U>) {
        (
            PointTwo{
                x: self.x,
                y: other.y
            },
            PointTwo{
                x: other.x,
                y: self.y
            }
        )
    }
}


fn test1(){
    let p1 = PointTwo {
        x : 1,
        y : "123" 
    };
    let p2 = PointTwo {
        x : "2",
        y : 456 
    };

    let (p3,p4) = p1.mix(p2);
    println!("{:?}",p3);

    println!("{:?}",p4);
}
