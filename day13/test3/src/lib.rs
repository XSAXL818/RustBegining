use std::{env, error::Error, fs};

// 返回一个实现了Error Trait的类型
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ?处理，如果是Ok，返回Ok携带的数据。如果是Err，返回Err
    let contents = fs::read_to_string(config.file_path)?;

    println!("查询结果：");
    let ret = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in ret {
        println!("{}",line);
    }

    println!("\n文件内容如下：\n{}",contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    // 获取命令行参数中的待查询字串和文件路径
    pub fn new(mut args:std::env::Args) -> Result<Config,&'static str> {
        if args.len() != 3 {
            return Err("参数个数错误，请输入正确的参数");
        } 
        // var函数，检查是否出现 大小写不敏感 变量，返回结果是Result
        // is_err函数，如果返回的结果是Err则返回true，否则false
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok( Config{
            query: args.next().unwrap(), 
            file_path: args.next().unwrap(),
            case_sensitive
        } )
    }
}

fn search<'a>(query:&str, contents:&'a str) ->Vec<&'a str>  {
    
    // let mut ret = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         ret.push(line);
    //     }
    // }

    // ret

    contents.lines()
        .filter(|line| line.contains(query))
        .collect()

    
}

fn search_case_insensitive<'a>(query:&str,contents:&'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();

    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            ret.push(line);
        }
    }

    ret
}
