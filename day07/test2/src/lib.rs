
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


mod School {
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

