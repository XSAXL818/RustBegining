# 标题：编写自动化测试

# 1、编写和运行测试

## 测试（函数）

- 测试：
  - 就是一个函数
  - 用于验证非测试代码的功能是否和预期一致

- 测试函数体（通常）执行的3个操作：

  - 准备数据
  - 运行被测试的代码
  - 断言结果

  

## 解剖测试函数

- 测试函数需要使用test属性进行标注
  - Attribute就是一段Rust代码的元数据
  - 在函数上#[test]，可将函数变为测试函数



## 运行测试

- 使用cargo test 命令运行所有测试函数

  - Rust会构建一个Test Runner 可执行文件
    - 它会运行标注了test的函数，并报告其运行是否成功

- 当使用cargo 创建 library 项目的适合，会生成一个test module，里面有一个test函数

  - 可以添加任意数量的test module 或函数

- 例子：使用 cargo new adder --lib 创建library库项目

  - 提供的函数示例

    ```rust
    pub fn add(left: u64, right: u64) -> u64 {
        left + right
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn it_works() {
            let result = add(2, 2);
            assert_eq!(result, 4);
        }
    }
    
    ```

  - 使用 cargo test运行测试

    ![image-20241129164459277](D:\Code\RustCode\RustBegining\day11\assets\image-20241129164459277.png)



## 测试失败

- 测试函数panic就表示失败

- 每个测试运行在一个新线程上

- 当主线程检查到测试线程挂掉（此例子使用panic!），就将该测试标记为失败

  ```rust
   	#[test]
      fn panic_func() {
          panic!("测试：调用panic!");
      }
  ```

  结果：

  ![image-20241129165247358](D:\Code\RustCode\RustBegining\day11\assets\image-20241129165247358.png)





















# 2、编写测试：断言(Assert)

- 相比较panic，建议使用assert!宏



## 使用assert!宏检查测试结果

- assert!宏，来自标准库，用来确定某个状态是否为true

  - true：测试通过
  - false：调用panic!，测试失败

  ```rust
  pub fn return_bool(num: i32) -> bool {
      if num >= 0 {
          return true
      }
      false
  }
  
  #[cfg(test)]
  mod tests {
      use super::*;
  
      #[test]
      fn test_for_return_bool_true() {
          assert!(return_bool(22));
      }
      #[test]
      fn test_for_return_bool_false() {
          assert!(!return_bool(-3));
      }
  }
  ```

  针对函数的功能，第一个返回true，第二个返回false，但使用了！取反，所以如果函数代码没有bug，则两个断言都会成功。

  ![image-20241129172348889](D:\Code\RustCode\RustBegining\day11\assets\image-20241129172348889.png)



## 使用assert_eq!和assert_ne!测试相等性

- equal和not equal

- 都来自标准库

- 判断两个参数是否相等或不等

- 实际上，他们使用的就是 == 和 != 运算符

- 额外的：断言失败后，会自动打印两个参数的值

  - 使用debug格式打印参数
    - 要求参数实现了PartialEq和Debug Trait特性（所有基本类型和标准库大部分类型都实现了）
  - assert_eq!(p1,p2)，p1和p2分别表示期待值和函数的返回值，p1、p2无位置关系

  ```rust
  pub fn add_two(x:i32) -> i32 {
      x+2
  }
  
  	#[test]
      fn test_for_add_two() {
          // 相等则测试通过
          assert_eq!(4,add_two(2));
      }
  ```

  失败后打印左值和右值，即参数的左右位置

  ![image-20241129173224293](D:\Code\RustCode\RustBegining\day11\assets\image-20241129173224293.png)

  ```rust
  	#[test]
      fn test_for_add_two_ne() {
          // 不相等则测试通过
          assert_ne!(5,add_two(2));
      }
  ```

  ![image-20241129173357324](D:\Code\RustCode\RustBegining\day11\assets\image-20241129173357324.png)

















# 3、编写测试：自定义错误信息

## 添加自定义错误信息

- 可以向assert!、assert_eq!、assert_ne!添加可选的自定义信息

  - 这些自定义信息和失败信息都会被打印出来
  - assert!：第一个参数必填，第二个参数为自定义信息，选填
  - assert_eq!和ne!，第三个参数为自定义信息
  - 自定义信息参数会被传递给format!宏，可以使用{}占位符

  ```rust
  fn return_bool(x:i32) -> bool {
      if x > 0 {
          true
      } else {
          false
      }
  }
  
  #[cfg(test)]
  mod tests {
      use super::*;
  
     #[test]
     fn for_return_bool() {
          let x = -3;
          assert!(return_bool(x));
     }
  }
  ```

  失败信息

  ![image-20241129174019547](D:\Code\RustCode\RustBegining\day11\assets\image-20241129174019547.png)

  加上自定义信息

  ```rust
  #[test]
     fn for_return_bool() {
          let x = -3;
          assert!(return_bool(x),
          "该值小于等于0,value={}",x);
     }
  ```

  结果如下：

  ![image-20241129174208999](D:\Code\RustCode\RustBegining\day11\assets\image-20241129174208999.png)

- 







# 4、使用should_panic属性检查是否恐慌

## 验证错误处理的情况

- 测试处理验证代码的返回值是否正确，还需验证代码是否如预期的处理发生错误的情况

- 有时需要验证代码在特定情况下是否发生了panic

- should_panic属性

  - 函数发生panic：测试通过
  - 函数没有发生panic：测试失败

  ```rust
  fn return_bool(x:i32) -> bool {
      if x > 0 {
          true
      } else {
          panic!("传入的值小于等于0,value={}",x);
      }
  }
  
  #[cfg(test)]
  mod tests {
      use super::*;
  
      #[test]
      #[should_panic]
      fn it_return_bool() {
          let x = -33;
          return_bool(x);
      }
  }
  ```

  测试会发生panic，测试通过

  ![image-20241129174755447](D:\Need\屏幕截图\image-20241129174755447.png)

  如果没有发生panic，则测试失败，参数改为大于0的

  ![image-20241129174828895](D:\Code\RustCode\RustBegining\day11\assets\image-20241129174828895.png)

  

## 让should_panic更精确

- 为should_panic属性添加一个可选的expected参数：
  - 将检查失败信息中是否包含所指定的文字
- 例子：发生恐慌，且匹配到相应的错误字段，测试才通过

```rust
fn return_bool(x:i32) -> bool {
    if x > 0 {
        true
    } else if x == 0 {
        panic!("传入的值等于0,value={}",x);
    } else {
        panic!("传入的值小于0,value={}",x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 将检查错误信息中是否有该字段
    #[should_panic(expected="传入的值等于0")]
    fn it_return_bool() {
        let x = 0;
        return_bool(x);
    }
}
```

![image-20241129175310025](D:\Code\RustCode\RustBegining\day11\assets\image-20241129175310025.png)

如传入的是小于0的数，测试不通过

![image-20241129175336203](D:\Code\RustCode\RustBegining\day11\assets\image-20241129175336203.png)





# 5、编写测试：在测试用中使用Result<T,E>

- 无需panic，可使用Result<T,E>作为返回类型编写测试

  - 返回Ok：测试通过
  - 返回Err：测试失败

  ```rust
      fn it_works_true() -> Result<(), String> {
          if 2+2 == 4 {
              Ok(())
          } else {
              Err(String::from("错误啦"))
          }
      }
      #[test]
      fn it_works_false() -> Result<(), String> {
          if 2+1 == 4 {
              Ok(())
          } else {
              Err(String::from("错误啦"))
          }
      }
  ```

  ![image-20241129212245884](D:\Code\RustCode\RustBegining\day11\assets\image-20241129212245884.png)

- 注意：返回Result<T,E>就不要使用#[should_panic]了



# 6、控制测试如何运行

- 改变cargo test的行为：通过添加命令行参数

- 默认行为：

  - 并行运行
  - 所有测试
  - 捕获（不显示）所有输出（如print!宏），使读取与测试结果相关的输出更容易

- 命令行参数：

  - 针对cargo test的参数：紧跟cargo test后
  - 针对 测试可执行程序：放在 -- 之后

- cargo test --help

  - 显示cargo test 之后可使用的参数

- cargo test -- --help

  - 用于显示cargo test --之后可以使用的参数，即两个-（--）后面的

  



# 7、控制测试运行：并行和连续执行测试

## 并行运行测试

- 运行多个测试：默认使用多个线程并行运行
  - 运行快
- 确保测试之间：
  - 不会互相依赖
  - 不依赖于某些共享资源（环境、工作目录、环境变量），且这些资源测试可能会修改



## --test-threads 参数

- 传递给二进制文件
- 使用--test-threads参数，后边跟线程的数量
  - 如cargo test -- --test-threads=1



## 显示函数输出

- 默认下，如果测试通过，Rust的test库会捕获打印在标准输出的内容

  - 如println!宏

  ```rust
  
  #[cfg(test)]
  mod tests {
      use super::*;
  
      #[test]
      fn it_works_good() {
          println!("该信息不会输出到标准输出中");
      }
      #[test]
      fn it_works_bad() {
          println!("该信息会输出到标准输出中");
          panic!("函数发生panic")
      }
  }
  ```

  ![image-20241129214315084](D:\Code\RustCode\RustBegining\day11\assets\image-20241129214315084.png)

- 使用cargo test -- --show-output显示测试成功的输出

  ![image-20241129214510832](D:\Code\RustCode\RustBegining\day11\assets\image-20241129214510832.png)





# 8、控制测试运行：按名称运行测试

- cargo test 运行所有测试

  ```rust
      #[test]
      fn it_works1() {}
      #[test]
      fn it_works2() {}
      #[test]
      fn it_works3() {}
  ```

  ![image-20241129215827777](D:\Code\RustCode\RustBegining\day11\assets\image-20241129215827777.png)

- 运行单个测试：将测试的名称作为 cargo test 的参数

  使用cargo test it_works1

  ![image-20241129215907457](D:\Code\RustCode\RustBegining\day11\assets\image-20241129215907457.png)

  但是参数只能传一个

  ![image-20241129215948485](D:\Code\RustCode\RustBegining\day11\assets\image-20241129215948485.png)

- 运行多个测试：指定测试名的一部分（模块名也可）

  - 测试名的一部分：cargo test it_works

    ![image-20241129220222415](D:\Code\RustCode\RustBegining\day11\assets\image-20241129220222415.png)

  - 模块名：cargo test tests

    ![image-20241129220252647](D:\Code\RustCode\RustBegining\day11\assets\image-20241129220252647.png)

    

    

# 9、控制测试运行：忽略测试

## 忽略某些测试，运行剩余测试

- #[ignore]属性

  ![image-20241129221907406](D:\Code\RustCode\RustBegining\day11\assets\image-20241129221907406.png)

- 单独运行忽略测试

  - cargo test -- --ignored

  ![image-20241129222010998](D:\Code\RustCode\RustBegining\day11\assets\image-20241129222010998.png)

- 













# 10、测试组织

## 测试的分类

- 单元测试
  - 小
  - 一次对一个模块进行隔离的测试
  - 可测试private接口
- 集成测试
  - 在库外部。和其他外部代码一样使用你的代码
  - 只能使用public接口
  - 可能在每个测试中使用到多个模块
- 区别：
  - 对于一个项目，单元测试可以具体测试到所有函数，而集成测试则是测试这项目的主要业务。增加用户功能由集成测试来测试，增加用户功能所使用到的相关检查函数则由单元测试来测试。




# 11、单元测试

## #[cfg(test)]标注

- 在模块名上使用 #[cfg(test)]标注

  - 只有运行cargo test 才编译和运行代码
  - 运行cargo build 则不会

- 集成测试在不同目录，不需要使用该标注

- cfg：configuration（配置）

  - 告诉Rsut下面的条目只有在指定的配置选项下才被包含
  - 配置选项test：由Rust提供，用来编译和运行测试
  - 只有cargo test 才会编译代码，包括模块中的helper函数和#[test]标注的函数

- 例子1：#[cfg(test)]模块下，未使用#[test]标注的函数也会被编译

  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;
  
      #[test]
      fn it_works() {
          let result = add(2, 2);
          assert_eq!(result, 4);
      }
  
      // 也会被编译
      fn other() {
          print!({})
      }
  }
  ```

  ![image-20241129223025664](D:\Code\RustCode\RustBegining\day11\assets\image-20241129223025664.png)



## 测试私有函数

- Rust允许测试私有函数

  ```rust
  fn other() {
      println!("私有函数");
  }
  
  #[cfg(test)]
  mod tests {
      use super::*;
  
      #[test]
      fn it_works() {
          let result = add(2, 2);
          assert_eq!(result, 4);
          other();
      }
  }
  ```

  

# 12、集成测试

- Rust中，集成测试完全位于被测试库的外部
- 目的：用于测试 被测试库 的多个部分是否能正常一起工作
- 集成测试的覆盖率很重要



## tests目录

- 创建集成测试：tests目录

- tests目录下的每个测试文件都是单独的一个crate

  - 需要将被测试的库导入

- 无需标注#[cfg(test)]，tests目录被特殊对待

  - 只有cargo test，才会编译tests目录下的文件

- 例子1：

  ```rust
  // lib.rs
  pub fn add(left: u64, right: u64) -> u64 {
      left + right
  }
  //
  ```

  ![image-20241130021249362](D:\Code\RustCode\RustBegining\day11\assets\image-20241130021249362.png)

  Running后面可以看出集成测试的程序



## 运行指定的集成测试

- 运行一个特定的集成测试：cargo test  函数名

  ```rust
  // tests目录下的一个函数
  use test12::*;
  
  
  #[test]
  fn it_works() {
      assert_eq!(add(2, 2),4);
  }
  // 另一个文件中的一个函数
  #[test]
  fn new_it_works() {
      assert_eq!(add(2, 2),4);
  }
  ```

  运行结果：

  ![image-20241130022010489](D:\Code\RustCode\RustBegining\day11\assets\image-20241130022010489.png)

  会模糊匹配所有函数的名称

- 运行某个测试文件内的所有测试：cargo test --test 文件名

  tests目录下的两个集成测试

  ![image-20241130022131020](D:\Code\RustCode\RustBegining\day11\assets\image-20241130022131020.png)

  运行结果

  ![image-20241130022150732](D:\Code\RustCode\RustBegining\day11\assets\image-20241130022150732.png)



## 集成测试中的子模块

- tests目录下每个文件被编译成单独的crate

  - 这些文件不共享行为（与src下的文件规则不同）
  - 如果想要写一个工具库供其他使用？

  ```rust
  //tests目录下的commom.rs文件
  pub fn setup() {
      println!("启动函数.....");
  }
  ```

  运行测试效果：

  ![image-20241130022735475](D:\Code\RustCode\RustBegining\day11\assets\image-20241130022735475.png)

  会将common.rs也编译运行

- 在tests目录下创建common文件夹，cargo test不会将该目录下的文件当作测试程序

  ![image-20241130023131160](D:\Code\RustCode\RustBegining\day11\assets\image-20241130023131160.png)

  运行结果

  ![image-20241130023158073](D:\Code\RustCode\RustBegining\day11\assets\image-20241130023158073.png)

- 复习：使用 mod 模块名； 即可将该模块导入到作用域，模块名对应文件名.

```rust
use test12::*;

// 也可用 use 导入具体的模块里的item
mod common;

#[test]
fn it_works() {
    common::setup();
    assert_eq!(add(2, 2),4);
}
```

运行结果

![image-20241130024045514](D:\Code\RustCode\RustBegining\day11\assets\image-20241130024045514.png)

 

## 针对binary crate 的集成测试

- 如果项目是binary crate,只含有src/main.rs没有src/lib.rs
  - 不能再tests目录下创建集成测试
  - 无法把main.rs的函数导入作用域
- 只有library crate 才能暴露函数给其他crate用
- binary crate 意味着独立运行
- 所以业务功能基本都放在lib.rs中,main.rs只做函数的调用

