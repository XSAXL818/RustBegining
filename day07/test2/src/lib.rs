use Produce::get_pri_log;


mod student {
    pub mod at_school {
        pub fn walk_to_room( addr:&str ) {
            println!("前往{}学习",addr);
        }
    }

  
}

// 通知学生地点
pub fn info(new_addr:&str) {
    let default = "10A318";
    crate::student::at_school::walk_to_room(default);

    student::at_school::walk_to_room(default);
}




// 理解为crate文件夹下有car文件和Animal文件夹，Animal文件下有dog文件
fn car() {}

mod Animal {
    fn dog() {
        super::car();
    }
}


pub mod School {
    pub struct Stu {
        pub name:String,
        pub age:u8
    }

}

fn get_pri() {
    let x = School::Stu{
        name:String::from("stu1"),
        age:13
    };
}


pub mod Produce {

    #[derive(Debug)]
    pub struct Car{
        pub version:String,
        log:String
    }
    // 只能获取标准化的车，不提供定制
    pub fn Car() ->Car {
        Car{
            version:String::from("1.1.1"),
            log:String::from("new")
        }
    }
    // 只提供部分信息的访问权，不可以修改
    pub fn get_pri_log (car:Car) -> String {
        car.log
    }
}

pub fn for_test() {
    let car = Produce::Car();
    println!("version={}",car.version);
    // println!("version={}",car.log);//无法直接访问
    println!("log={}",Produce::get_pri_log(car));
}


