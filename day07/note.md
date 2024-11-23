# 标题: Package（包），Crate（单元包），Module（模块）

# 1、Package、Crate、定义Module

## Rust的代码组织

- 代码组织主要包括：

  - 那些细节可以暴露，那些细节是私有的

  - 作用域内哪些名称有效

  - 。。。。

- 模块系统：

  - Package(包)：Cargo的特性，让你构建、测试、共享crate
  - Crate(单元包)：一个模块树，它可以产生一个library或可执行文件
  - Module(模块)、use：让你控制代码的组织、作用域、私有路径
  - Path(路径)：为struct、function或module等项命名的方式



## Package和Crate

- Crate的类型：

  - binary：二进制文件
  - library：库文件

- Crate Root：

  - Crate的根
  - 是源代码文件
  - Rust编译器从这里开始，组成你的Crate的根Module
  - Crate的上面是Package

- 一个Package：

  - 包含1个Cargo.toml，它描述了如何构建这些Crates

  - 只能包含0-1个library crate

  - 可以包含任意数量的binary crate

  - 但必须至少包含一个crate（library或binary  ）

  - 使用cargo创建项目(即一个Package)![](D:\Code\RustCode\RustBegining\day07\assets\屏幕截图 2024-11-21 203645.png)

  - 项目结构：

    ![image-20241121203853051](D:\Code\RustCode\RustBegining\day07\assets\image-20241121203853051.png)

    - 其中target是VSCode中插件自动生成的

    - main.rs是项目的入口文件

    - 但在配置文件中没有提及（如同其他语言main函数是入口一样）

      ![image-20241121204407413](D:\Code\RustCode\RustBegining\day07\assets\image-20241121204407413.png)

  - Cargo.toml是配置文件



## Cargo的惯例

- src/main.rs：
  - binary crate 的 crate root
  - crate名与package名相同
- src/lib.rs：
  - package包含一个library crate
  - library crate的crate root
  - crate的名与package名相同
- Cargo把crate root 文件交给rustc来构建library或binary
- 一个Package可以同时包含src/main.rs和src/lib.rs
  - 一个binary crate，一个library crate
  - 名称与package名相同
- 一个Package可以有多个binary crate：
  - 文件放在src/bin
  - 每个文件是单独的binary crate
- 不要慌......





## Crate的作用

- 将相关功能组合到一个作用域内，便于项目间分享
  - 防止冲突
- 例如 rand crate，访问它的功能需要通过它的名字：rand



## 定义module来控制作用域和私有性

- Module：

  - 在一个crate内，将代码进行分组
  - 控制项目的私有行。public、private
  - 增加可读性、易于复用

- 建立module：

  - mod关键字
  - 可嵌套
  - 可包含其他项（struct、enum、常量、trait、函数等）的定义

  ```rust
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
  ```

  



## Module

- crate(man.rs)

  - Shcool(module)
    - Student(子module)
      - add_to(包含的函数)
      - del(包含的函数)
    - Teacher(子module)
      - 包含的函数..
      - ...
    - StuInfo结构体（包含的结构体）

- src/main.rs和src/lib.rs叫做crate roots

  ![image-20241121214249056](D:\Code\RustCode\RustBegining\day07\assets\image-20241121214249056.png)

  - 这两个文件这两个文件的内容各自形成了名为crate的模板，位于整个模块树的根部





# 2、路径Path

## 路径

- 为了在Rust的模块中找到某个条目(struct、enum、函数...)，需要使用路径。

- 路径的两种形式

  - 绝对路径：从crate root开始，使用crate名或字面值crate
  - 相对路径：从当前模块开始，使用self，super或当前模块的标识符

- 路径至少由一个标识符组成，标识符之间使用 ：：分隔 ( 类似于C++的命名空间呗 )

- ```rust
  mod student {
      mod at_school {
          fn walk_to_room( addr:&str ) {
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
  ```

  - info函数使用绝对路径和相对路径调用同一个crate的函数

  - 当定义部分和使用部分同时移动时，可以使用相对路径。

  - 当前代码的错误：

    ![image-20241121220110707](D:\Code\RustCode\RustBegining\day07\assets\image-20241121220110707.png)



## 私有边界

- 模块不仅可以组织代码，还可以定义私有边界
- 如果想把函数或struct等设为私有，可以将它放在某个模块中
- Rust中的所有条目（函数、方法、struct、enum、常量、模块）默认为私有的
- 父级模块无法访问子模块中的私有条目（额。。。。，没必要理解）[ok，就是类的private而已...]
- 子模块中可以使用所有祖先模块中的条目





## pub关键字

- 使用pub关键字来将某些条目标记为公共的

- ```rust
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
  ```

  - info和student同级，所有info可以看到student
  - 如果没有pub at_school，则info看不到student的at_school
  - 如果没有pub walk_to_room，则info看不到at_school下的walk_to_room
  - 简单理解为类的私有成员即可。



## super关键字

- 对于类似树形结构的组织(理解为文件结构、类的继承)，super表示上一级。

- super用来访问父级模块路径中的内容

  ```rust
  // 理解为crate文件夹下有car文件和Animal文件夹，Animal文件下有dog文件
  fn car() {}
  
  mod Animal {
      fn dog() {
          super::car();
      }
  }
  ```





## pub struct

- 使用pub修饰struct，则这个struct为公共的，但其字段是私有的

  ```rust
  mod School {
      pub struct Stu {
          name:String,
          age:u8
      }
  
  }
  
  fn get_pri() {
      let x = School::Stu{
          name:String::from("stu1"),
          age:13
      };
  }
  ```

  ![image-20241121222153942](D:\Code\RustCode\RustBegining\day07\assets\image-20241121222153942.png)

- 字段使用pub修饰

  ```rust
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
  ```
  
  



## pub enum

- pub修饰enum
  - enum是公共的
  - enum的变体也都是公共的，变体就是枚举类型具体的值





# 3、Use关键字

## use关键字

- 可以使用use关键字将路径导入到作用域中

  - 导入的内容遵守私有性规则

  ```rust
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
  ```

- 使用use时，可使用绝对路径和相对路径来引用

  ```rust
  use school::student::add_stu;
  use crate::school::student::add_stu;
  ```

  - 目前新手个人推荐使用绝对路径
  - 当使用其他模块的函数时，推荐引用该函数的上级模块
    - 如果使用函数多，需要多次引用，麻烦
    - 直接引用，无法区分该函数是本地定义还是引用来的



## use的习惯用法

- 函数：将函数的父级模块引入作用域（指定到父级）

- struct，enum，其他：指定完整路径（指定到本身）

  ```rust
  use std::collections::HashMap;
  
  fn test1(){
      let mut map = HashMap::new();
      map.insert(1, 3);
  }
  ```

- 不同模块存在同名条目：指定到父级，防止冲突。可使用as解决。



## as关键字

- 为引入的路径指定本地的别名

  ```rust
  use std::collections::HashSet as Set;
  
  fn test1(){
      let mut set = Set::new();
      set.insert("value");
  }
  ```



## 使用 pub use 重新导入名称

- 使用use将路径导入到作用域内后，该名称在此作用域是私有的

  ```rust
      // 假设xxx是一个自己常用的工具库
      mod xxx {
          use test3;
          // xxx里含有test3所有的 pub 修饰的结构
          // 同时将这些条目修改为private，不供外部访问
          
      }
      // xxx::test3::nwnu();// 报错
  ```

- 使用 pub use 来重新导出

  - 将条目引用作用域
  - 该条目可以被外部代码再次引用

  ```rust
      mod xxx_pub {
          pub use test3;
          // 将test3的所有pub结构引入，
          // 并修饰为pub，外部可以访问
      }
      xxx_pub::test3::nwnu();
  ```



## 使用外部包( package )

- 常规流程

  1. Cargo.toml添加依赖的包( package )

     - 自动在https://crates.io/网站上下载（内网访问🆗的，但下载可能有点慢）

     - Cargo.toml下导包

       ![](D:\Code\RustCode\RustBegining\day07\assets\屏幕截图 2024-11-23 174212.png)

     - 包下载的位置

       ![image-20241123175026072](D:\Code\RustCode\RustBegining\day07\assets\image-20241123175026072.png)

  2. use 将特定条目引用到作用域中使用

     ![image-20241123180411919](D:\Code\RustCode\RustBegining\day07\assets\image-20241123180411919.png)

- 标准库(std)也被当做外部包

  - 但是其已经被内置了，不需要修改Cargo.toml来包含std
  - 但需要使用use来引入std中的具体条目



## 查询cargo的package目录

![](D:\Code\RustCode\RustBegining\day07\assets\屏幕截图 2024-11-23 175131.png)

![](D:\Code\RustCode\RustBegining\day07\assets\屏幕截图 2024-11-23 175156.png)

注意：如果.cargo目录的package-cache有大小，说明导包时为下载未完成



## 设置清华镜像

...目前不设置感觉也挺快的，先不搞了



## 使用嵌套路径清晰大量的use语句

- 如果使用同一个包或者模块下的多个条目

  ![image-20241123184119937](D:\Code\RustCode\RustBegining\day07\assets\image-20241123184119937.png)

- 可使用嵌套路径在同一行内将上述条目进行引入：

  - 路径相同的部分::{路径差异的部分}

    ![image-20241123184657968](D:\Code\RustCode\RustBegining\day07\assets\image-20241123184657968.png)

    

- 一个引用是另外一个引用的子路径：使用self

  ```rust
  use std::io;
  use std::io::Write;
  ```

  优化

  ```rust
  use std::io::{self,Write};
  ```



## 通配符 *

- 使用*可以把路径下所有的公共条目都引入到作用域中

  ```rust
      use std::collections::*;
      let mut x = LinkedList::new();
      x.push_back("elt");
  ```

- 通常不使用

- 应用场景

  - 测试。将所有北侧代码引入tests模块
  - 被用于预导入(prelude)模块





# 4、将模块拆分为不同的文件



## 将模块内容移动到其他文件

- 模块定义时，如果模块名后面使用；分号结束时（注意是mod 模块名;）

  - Rust会从模块同名的文件中加载内容
  - 模块树的结构不会变化，即模块的树结构转换为文件目录结构

- 例子1：使用  **use 模块名；** 导入模块

  ```rust
  // school.rs 文件
  pub mod student {
      pub fn add_stu() {
          println!("add_stu");
         
      }
  }
  
  // lib.rs 文件
  mod school;
  
  fn work() {
      school::student::add_stu();
  }
  ```

- 例子2：模块的树结构不变

  ```rust
  // level1.rs 文件
  // level1文件会在level1文件夹下查找level2.rs文件
  pub mod level2;
  
  // level2.rs 文件，不在level1文件夹下，就是普通的一级（lib.rs同级）
  pub fn fn_in_lel2() {
      println!("level2");
      
  }
  
  // lib.rs 文件，对于lib，会在同级目录下在level1.rs文件
  mod level1;
  
  // main.rs 文件通过项目名(package)test4引用lib.rs的文件
  test4::level();
  ```

  
