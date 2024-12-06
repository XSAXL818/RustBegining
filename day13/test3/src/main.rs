// 使用env下的 args 函数，引入路径为其父级
use std::env;
use std::process;
use test3::Config;


fn main() {
    
    // 使用args获取参数，调用collect函数，返回指定的类型
    let args = env::args();
    // 当前函数不接受非法的UTF-8的字符
    // 如果出现，程序会恐慌。使用args_os接受非法的UTF-8
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("程序出错：{}",err);
        process::exit(1);
    });
    

    if let Err(e) = test3::run(config) {
        eprintln!("程序出错:{}",e);
    }
}

