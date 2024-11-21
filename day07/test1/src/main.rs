fn main() {
    

    test1();
    println!("--------");


}

fn test1(){
    
}


mod School {

    #[derive(Debug)]
    struct StuInfo{
        year:u32,
        name:String,
        gender:char
    }

    mod Student {
        use super::StuInfo;

        fn add_to(stu : StuInfo) {
            println!("录取学生:{:?}",stu);
        }
        fn del() {
            println!("开除学生");
        }
    }

    mod Teacher {
        fn teach() {
            println!("教课");
        }

        fn search(){
            println!("做研究");
        }
    }

}