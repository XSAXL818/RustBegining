# 项目实例：命令行程序

- 功能
  - 简单版本的grep
  - 运行方式：cargo run [要查询的字符串] [文件名]

# 1、接受命令行参数

```rust
fn test1() {
    // 使用args获取参数，调用collect函数，返回指定的类型
    let args: Vec<String> = env::args().collect();
    // 当前函数不接受非法的UTF-8的字符
    // 如果出现，程序会恐慌。使用args_os接受非法的UTF-8
    println!("{:?}",args);

    if args.len() != 3 {
        println!("参数太少\n---打印帮助函数");
        return ;
    }
    let query = &args[1];
    let file = &args[2];
    println!("要查询的字符串={}\n指定的文件={}",query,file);

}
```





# 2、读取文件

```rust
fn test1() {
    // 使用args获取参数，调用collect函数，返回指定的类型
    let args: Vec<String> = env::args().collect();
    // 当前函数不接受非法的UTF-8的字符
    // 如果出现，程序会恐慌。使用args_os接受非法的UTF-8
    println!("{:?}",args);

    if args.len() != 3 {
        println!("参数太少\n---打印帮助函数");
        return ;
    }
    let query = &args[1];
    let file_path = &args[2];
    println!("要查询的字符串={}\n指定的文件={}",query,file_path);

    let contents = fs::read_to_string(file_path).expect("文件读取错误！");
    println!("文件内容如下：\n{}",contents);
}
```

- 代码越少，重构越简单



# 3、重构：改进模块和错误处理

## 二进制程序关注点分离的指导性规则

- 将程序拆分为main.rs和lib.rs，将业务逻辑放入lib.rs
- 当命令行解析逻辑较少时，将它放在main.rs也行
- 当命令行解析逻辑变复杂时，需要将它从main.rs提取到lib.rs



## 经过上述拆分，留在main的功能有：

- 使用参数值调用命令行解析逻辑
- 进行其他配置
- 调用lib.rs中的run函数
- 处理run函数可能出现的错误



```rust
// lib.rs，写业务逻辑
use std::{error::Error, fs};

// 返回一个实现了Error Trait的类型
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ?处理，如果是Ok，返回Ok携带的数据。如果是Err，返回Err
    let contents = fs::read_to_string(config.file_path)?;
    println!("文件内容如下：\n{}",contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    // 获取命令行参数中的待查询字串和文件路径
    pub fn new(args:& [String]) -> Result<Config,&'static str> {
        if args.len() != 3 {
            return Err("参数个数错误，请输入正确的参数");
        } 

        Ok( Config{
            query: args[1].clone(), 
            file_path: args[2].clone()
        } )
    }
}

// main.rs，获取配置、处理错误、使用run函数
// 使用env下的 args 函数，引入路径为其父级
use std::env;
use std::process;
use minigrep::Config;


fn main() {
    
    // 使用args获取参数，调用collect函数，返回指定的类型
    let args: Vec<String> = env::args().collect();
    // 当前函数不接受非法的UTF-8的字符
    // 如果出现，程序会恐慌。使用args_os接受非法的UTF-8
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("程序出错：{}",err);
        process::exit(1);
    });
    

    if let Err(e) = minigrep::run(config) {
        println!("程序出错:{}",e);
    }
}
```









# 4、使用TDD（测试驱动开发）开发库功能

- 编写一个会失败的测试、运行该测试，确保它是按照预期的原因事变
- 编写或修改刚好足够的代码，让新测试通过
- 重构刚刚添加或修改的代码，确保测试会始终通过
- 返回步骤1，继续



```rust
// 写根据功能写好测试
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn one_result() {
        let query = "GDragon";
        let contents = "\
BigBang
XSAXL
GDragon
DeSung
TaiYang
Top
GD&TOP
        ";
        assert_eq!(vec!["GDragon"],search(query,contents));
    }
}
// 实现测试里的功能
fn search<'a>(query:&str, contents:&'a str) ->Vec<&'a str>  {
    
    let mut ret = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            ret.push(line);
        }
    }

    ret
}
```





# 5、使用环境变量

```rust
	// var函数，检查是否出现 大小写不敏感 变量，返回结果是Result
    // is_err函数，如果返回的结果是Err则返回true，否则false
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
```



# 6、将错误消息写入标准错误而不是标准输出

## 标准输出vs标准错误

- 标准输出：stdout
  - println!
- 标准错误：stderr
  - eprintln!

```rust
// 打印的错误，输出到cmd命令行上
fn main() {
    
    // 使用args获取参数，调用collect函数，返回指定的类型
    let args: Vec<String> = env::args().collect();
    // 当前函数不接受非法的UTF-8的字符
    // 如果出现，程序会恐慌。使用args_os接受非法的UTF-8
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("程序出错：{}",err);
        process::exit(1);
    });
    

    if let Err(e) = minigrep::run(config) {
        eprintln!("程序出错:{}",e);
    }
}
// pringln! 打印的结果，输出到文件中
```

![image-20241201211926630](D:\Code\RustCode\RustBegining\day12\assets\image-20241201211926630.png)

![image-20241201211940915](D:\Code\RustCode\RustBegining\day12\assets\image-20241201211940915.png)
