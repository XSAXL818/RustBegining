use std::io;
use rand::Rng; // trait
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("秘密数字是: {}", secret_number);
    
    println!("猜数游戏!");

    loop{
        println!("请输入你猜的数:");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
        .expect("无法读取行");
        
        // 老版本：当输入字母时，程序会崩溃
        // let guess:u32 = guess.trim().parse().expect("请输入数字");
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("输入错误，请输入数字！！！");
                continue;
            },
        };
    
        println!("你猜的数是: {}", guess);
    
        match guess.cmp(&secret_number){
            Ordering::Less=> println!("太小了"),
            Ordering::Greater=> println!("太大了"),
            Ordering::Equal=> {
                println!("猜对了");
                break;
            } 
        }
    }

    
}

