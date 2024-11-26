use std::{fmt::Error, fs::File, io::{self, ErrorKind, Read}};

fn main() {
    

    // test1();
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

fn read_username_link() -> Result<String,io::Error> {
    let path = r"D:\Code\RustCode\RustBegining\res\username.txt";
    let mut name = String::new();

    // let mut file = File::open(path)?;
    // file.read_to_string(&mut name)?;
    File::open(path)?.read_to_string(&mut name)?;



    Ok(name)
}

fn read_username_plus() -> Result<String,io::Error> {
    let path = r"D:\Code\RustCode\RustBegining\res\username.txt";
    let mut file = File::open(path)?;

    let mut name = String::new();
    file.read_to_string(&mut name)?;
    Ok(name)
}


fn read_username() -> Result<String,io::Error> {
    let path = r"D:\Code\RustCode\RustBegining\res\username.txt";
    let file = File::open(path);

    let mut file = match file {
        Ok(f) => f,
        Err(e) => return Err(e)
    };

    let mut name = String::new();
    match file.read_to_string(&mut name) {
        Ok(_) => Ok(name),
        Err(e) => Err(e)
    }
}

fn test5() {
    print!("")?;

}


fn test4() {
    let path = r"D:\Code\RustCode\RustBegining\res\test1111.txt";
    // let file = File::open(path).unwrap();

    // let file = match File::open(path) {
    //     Result::Ok(data) => data,
    //     Result::Err(err) => panic!("{}",err)
    // };


    let file = File::open(path).expect("expect可以自定义错误信息");
}


fn test3() {

    let path = r"D:\Code\RustCode\RustBegining\res\test1.txt";
    let file = File::open(path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(path).unwrap_or_else(|error | {
                panic!("{:?}",error)
            })
        } else {
            panic!("文件打开错误:{:?}",error);
        }
    });
}


fn test2() {
    let path = r"D:\Code\RustCode\RustBegining\res\test1.txt";
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(new_file) => new_file,
                Err(err) => panic!("文件未找到，且创建失败：{:?}",err)
            },
            other_error => panic!("文件打开失败：{:?}",other_error)
        
        }
    };
    println!("文件信息: {:?}",file);
}

fn test1(){
    // 成功返回的数据是File类型，失败的是Error类型
    let ret:Result<File, std::io::Error> = File::open(r"D:\Code\RustCode\RustBegining\res\test1.txt");
    let file = match ret {
        Ok(file) => file,
        Err(error)=> panic!("{:?}",error)
    };
    println!("文件信息：{:?}",file);
    
    let ret:Result<File, std::io::Error> = File::open(r"D:\Code\RustCode\RustBegining\res\test2.txt");
    match ret {
        Ok(file) => println!("{:?}",file),
        Err(error)=> println!("{:?}",error)
    };
}
