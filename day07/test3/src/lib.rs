
mod school {
    pub mod student {
        pub fn add_stu(){
            println!("add_stu");
        }
        fn add_teacher(){
            println!("add_teacher");
        }
    }
}

// 当前作用域引入add_stu
use school::student::add_stu;


pub fn nwnu() {
    add_stu();
    add_stu();
    // add_teacher();// 遵循私有性原则
}