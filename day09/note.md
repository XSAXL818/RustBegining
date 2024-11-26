# 标题：错误处理

# 1、panic!宏和不可恢复的错误

## Rust的错误处理概述

- Rust的可靠性：错误处理
  - 大部分情况下：在编译时提示错误，并处理
- 错误的分类：
  - 可恢复：如文件未找到，可再次尝试
  - 不可恢复：访问索引超出范围
- Rust没有类似异常的处理机制
  - 可恢复错误：Result<T,E>
  - 不可恢复：panic!宏



## 不可恢复的错误与panic!

- 当panic!宏执行时的动作（默认情况下）：

  1. 程序打印错误信息
  2. 展开、清理调用栈
  3. 退出程序

- 第二的两种情况

  - 默认：程序展开调用栈（工作量大）
    - Rust沿着调用栈往回走
    - 清理每个遇到的函数中的数据
  - 或立即中止调用栈：
    - 不进行清理，直接停止程序
    - 内存需要OS进行清理

- 如果想让程序最终的二进制文件更小，把设置从“展开”改为“中止”：

  - 在Cargo.toml中[profile]范围下设置：

    - panic="abort"

      ![image-20241125213131210](D:\Code\RustCode\RustBegining\day09\assets\image-20241125213131210.png)

- 案例

  - 调用panic!宏

    ```rust
    fn test1(){
        panic!("调用panic!宏");
    }
    ```

    ![image-20241125213457425](D:\Code\RustCode\RustBegining\day09\assets\image-20241125213457425.png)

  - 越界访问

    ```rust
    fn test1(){
        panic!("调用panic!宏");
    }
    ```

    ![image-20241125213636818](D:\Code\RustCode\RustBegining\day09\assets\image-20241125213636818.png)

    当前代码在main.rs中16：6下发生panic，但代码本身只是函数调用，没有panic!宏调用



## 使用panic!产生的回溯信息

- panic!可能发生在使用的函数中

- 可通过调用！的函数的回溯信息来定位出现问题的代码

- 可通过设置环境变量RUST_BACKTRACE得到回溯信息

  ![image-20241125214949451](D:\Code\RustCode\RustBegining\day09\assets\image-20241125214949451.png)

  VSCode应该自动设置了该环境变量

- 为了获取带有调试信息的回溯，必须启动调试符号（命令cargo run --release去除release）



# 2、Result枚举与可恢复的错误

## Result枚举

- 源码

  ```rust
  pub enum Result<T, E> {
      /// Contains the success value
      #[lang = "Ok"]
      #[stable(feature = "rust1", since = "1.0.0")]
      Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
  
      /// Contains the error value
      #[lang = "Err"]
      #[stable(feature = "rust1", since = "1.0.0")]
      Err(#[stable(feature = "rust1", since = "1.0.0")] E),
  }
  ```

- T：操作成功情况下，返回Ok变体时的数据类型

- E：操作失败，返回Err变体里的数据类型

- 例子：打开不存在文件

  ```rust
      // 成功返回的数据是File类型，失败的是Error类型
      let ret:Result<File, std::io::Error> = File::open("xsaxl.txt");
  ```



## 处理Result的一种方式：match表达式

- 和Option枚举相同，Result及其变体也是prelude（预导入）的

- 例子：

  ```rust
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
  ```

  match设计的真牛🐂





## 匹配不同的错误

- 打开文件，未找到则默认创建文件

  ```rust
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
  
  // ErrorKind枚举源码
  pub enum ErrorKind {
      NotFound,
      PermissionDenied,
      ConnectionRefused,
      ConnectionReset,
      HostUnreachable,
      NetworkUnreachable,
      ConnectionAborted,
      .....
  ```

- match很有用，但使用很原始

- 闭包(closure)。Result<T,E>提供了很多方法：

  - 它们接受闭包作为参数
  - 使用match实现

- 改进后

  ```rust
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
  ```



## unwrap方法

- 源码

  ```rust
      pub fn unwrap(self) -> T
      where
          E: fmt::Debug,
      {
          match self {
              Ok(t) => t,
              Err(e) => unwrap_failed("called `Result::unwrap()` on an `Err` value", &e),
          }
      }
  // 案例
      let path = r"D:\Code\RustCode\RustBegining\res\test1111.txt";
      let file = File::open(path).unwrap();
  	// 下面处理等价于上面一行
      // let file = match File::open(path) {
      //     Result::Ok(data) => data,
      //     Result::Err(err) => panic!("{}",err)
      // };
  ```

  - 如果Result是Ok，就会返回Ok里面的值
  - 是Err，就会调用panic!宏。不可以自定义Err的行为。



## expect

- expect：和unwrap类似，但可以指定错误信息

  ```rust
      let file = File::open(path).expect("expect可以自定义错误信息");
  ```

  



## 传播错误

- 在出错的代码处，处理错误

- 将错误通过函数返回到调用者

  ```rust
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
  ```

  

## ？运算符

- 传播错误的一种快捷方式

  ```rust
  fn read_username_plus() -> Result<String,io::Error> {
      let path = r"D:\Code\RustCode\RustBegining\res\username.txt";
      let mut file = File::open(path)?;
  
      let mut name = String::new();
      file.read_to_string(&mut name)?;
      Ok(name)
  }
  ```

  ？的意思：对于返回类型是Result枚举类型，如果值为Ok(data)则将data返回；如果值是Err(e)则将整体返回

- 链式调用

  ```rust
  File::open(path)?.read_to_string(&mut name)?;
  ```

- 只用用于返回类型为Result或者Option或者实现了Try的类型的函数

  ![image-20241126160354875](D:\Code\RustCode\RustBegining\day09\assets\image-20241126160354875.png)

  print!()返回值为()，所以不可以使用

- 如果当前当前函数的返回类型不是Result，函数内不可以使用？



## ？与from函数

- Trait std::convert::From 上的from函数：
  - 用于错误之间的转换
- 被？所应用的错误，会隐式的被from函数处理
- 当?调用from函数时：
  - 它所接受的错误类型会被转化为当前函数返回类型所定义的错误类型
  - 错误EA必须实现了转换为EB的from函数
- 用于：针对不同错误原因，返回同一种错误类型
  - 只要每个错误类型实现了转换为所返回的错误类型的from函数







# 3、何时使用panic

## 总体原则

- 定义一个可能失败的函数时，优先考虑返回Result，否则就panic!



## 编写示例、原型代码、测试

- 可以使用panic!
  - 演示某些概念：unwrap
  - 原型代码：unwrap、expect
  - unwrap、expect





## 有时你比编译器掌握更多的信息





## 错误处理的指导性建议

- 当代码最终可能处于损坏状态时，最好使用panic!
- 损坏状态：某些假设、保证、约定或不可变性被打破
  - 例如非法的值、矛盾的值或空缺的值被传入代码
  - 以及下列的一条：
    - 这种损坏状态并不是预期会偶尔发生的
    - 再次之后，代码处于损坏状态就无法运行
    - 在使用的类型中没有一个好的方法处理这些信息进行编码



## 场景建议

- 调用你的代码，传入无意义的参数值：panic!
- 调用外部不可控代码，返回非法状态，你无法恢复：panic！
- 失败时可预期的：Result
- 代码对值进行操作，首先应该验证这些值时：panic!



## 为验证创建自定义类型

- 创建新的类型，把验证逻辑放在构造实例的函数里

  ```rust
  struct GuessNum {
      number: i32
  }
  
  impl GuessNum {
      pub fn new (number:i32) -> GuessNum {
          if number < 1 || number > 100 {
              panic!("输入的数字必须在1-100之间，当前数字为:{}",number);
          }
          // 变量名和结构体的成员变量名称相同
          GuessNum { number }
      }
      pub fn number(&self) -> i32 {
          self.number
      }
  }
  
  
  fn test1(){ 
      loop {
          // 获取随机数 .........
          // 验证用户输入，如果输入有误关闭程序
          let guess_number = "13";
          let guess:i32 = match guess_number.trim().parse(){
              Ok(num) => num,
              Err(e) => continue
          };
  
          let guess = GuessNum::new(guess);
  
          // 判断是否猜中
      }
  }
  ```

- getter：返回字段数据

