fn main() {
    

    test1();
    println!("--------");


}

struct GuessNum {
    number: i32
}

impl GuessNum {
    pub fn new (number:i32) -> GuessNum {
        if number < 1 || number > 100 {
            panic!("输入的数字必须在1-100之间，当前数字为:{}",number);
        }
        // 变量名和结构体的成员变量名称相同
        GuessNum { number }
    }
    pub fn number(&self) -> i32 {
        self.number
    }
}


fn test1(){ 
    loop {
        // 获取随机数 .........
        // 验证用户输入，如果输入有误关闭程序
        let guess_number = "13";
        let guess:i32 = match guess_number.trim().parse(){
            Ok(num) => num,
            Err(e) => continue
        };

        let guess = GuessNum::new(guess);

        // 判断是否猜中
    }
}
