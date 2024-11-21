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
- 父级模块无法访问子模块中的私有条目（额。。。。，没必要理解）
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
  记得写个例子：
  一个struct类，部分字段是pub，一个pub函数返回该struct的实例。
  在其他的mod中获取该struct，然后试着访问其字段。
  
  ```

  





## pub enum
