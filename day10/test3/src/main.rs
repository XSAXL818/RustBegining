use test3::{my_println, Car, Cat, CatTrait, Dog, DouBao, HomeLand, MyToString, ToString, ToStringByDefault};


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
    let  x = String::from("小白");
    println!("{}",x.my_to_string());
}


fn test4() {
    let hl = HomeLand {
        name: String::from("祖国人"),
        age: 43
    };

    let db = DouBao {
        name: String::from("豆包"),
        model: String::from("模型")
    };

    my_println(hl);
    my_println(db);
}


fn test3() {
    let cat = Cat {
        name: String::from("小秘密"),
        age: 4
    };
    cat.test1();
   
}

fn test2() {
    let dog = Dog {
        name: String::from("小白"),
        host: String::from("小飒"),
        age: 5
    };

    let car = Car {
        version: String::from("1.0"),
        name: String::from("苏7"),
        price: 13.40,
        brand: String::from("小米"),
    };
    
    println!("{}",dog.to_string_default());
    println!("{}",car.to_string_default());
}

fn test1(){
    let car = Car {
        version: String::from("1.0"),
        name: String::from("苏7"),
        price: 13.40,
        brand: String::from("小米"),
    };

    let dog = Dog {
        name: String::from("小苟"),
        host: String::from("LoongTory"),
        age: 6
    };

    println!("{}",car.to_string());
    println!("{}",dog.to_string());
}



