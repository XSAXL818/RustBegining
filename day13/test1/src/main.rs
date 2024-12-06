fn main() {
    

    test1();
    println!("--------");

    test2();
    println!("--------");

}


fn test2() {
    let x = String::from("value");
    
    // 假设线程一
    {
        let cacher = move |src:String|  src.contains(&x);
        let src = String::from("new value");
        println!("contains:{}",cacher(src));
    }

    // println!("{}",x);
}



struct Load<T>
where T: Fn(i32) -> i32 
{
    pub calcu: T,
    pub value: Option<i32>
}


impl<T> Load<T> 
where T : Fn(i32) -> i32 
{
    fn new(calcu:T) -> Load<T> {
        Load{
            calcu,
            value:None
        }
    } 

    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let x = (self.calcu)(arg);
                self.value = Some(x);
                x
            }
        }
    }
}

fn test1(){

    let func = |x| x;
    let str = func(String::from("value"));
    // let num = func(10);


    let x = 14;
    let bb = |ret| {
        ret == x
    };
    let y = 10;
    println!("y=={}---{}",x,bb(y));
    
}


