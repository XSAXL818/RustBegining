# 1、定义并实例化struct

## struct

### 定义struct

- 和C++一样咩，只是成员变量声明时，需要使用:冒号来指定类型

  ```rust
  struct User {
      name:String,
      age:i8
  }
  ```



### 实例化struct

- 实例化struct时需要注意：

  - 为每个字段指定具体值
  - 无需按声明的顺序进行指定

- ```rust
      let user1 = User{
          name:String::from("小张"),
          age:15
      };
  ```



### 访问struct的成员变量

- 使用点标记法

  ```rust
  println!("name={},age={}",user1.name,user1.age);
  ```

- 一旦struct的实例是可变的，那么实例中的所有字段都是可变的

  ```rust
      let user1 = User{
          name:String::from("小张"),
          age:15
      };
      // user1.age = 3;// 未声明为可变
      println!("name={},age={}",user1.name,user1.age);
  
      let mut user2 = User{
          name:String::from("小王"),
          age:49
      };
      user2.age=34;
      user2.name=String::from("喜喜");
      println!("name={},age={}",user2.name,user2.age);
  
  ```

- 可作为函数的返回值





### 字段初始化简写

- 字段名和字段值对应变量名相同时。即使用其他变量来初始化struct，且变量名和字段名相同。

  ```rust
  fn test2(){
      let user = User(String::from("小王"),18);
      println!("name={},age={}",user.name,user.age);
  }
  
  fn User(name:String, age:i8) -> User {
      User { name, age }
  }
  ```

  



### struct更新语法

- 当你想基于原有的实例，来初始化新实例时，可以使用更新语法

  老的方法：

  ```rust
  	// 如果使用u1中较多字段，书写繁琐
  	let u1 = User{
          name:String::from("小薇薇"),
          age:55
      };
      let u2 = User{
          name:u1.name,
          age:u1.age
      };
  ```

  更新语法：

  ```rust
      let u2 = User{
          name:String::from("小白"),
          ..u1 
      };
  ```

  - 如果struct的字段有如String这种类型，使用更新语法报错：String未实现Copy



## Tuple struct

- 可定义类似于tuple元组的struct，叫做tuple struct。

  - Tuple struct整体有名，但里面的元素没有名
  - 适用于想给tuple起个名，需要让该tuple不同于其他tuple，而且也不需要为每个元素起名

- 定义tuple struct：使用struct关键字，只需要填写变量的类型即可。

  ```rust
      struct Color(i32,i32,i32);
      struct Point3(i32,i32,i32);
  
      let black = Color(0,0,0);
      let center = Point3(0,0,0);
  ```



## Unit-Like Struct (没有任何字段)

- 可以定义没有任何字段的struct，与()空元组类似。
- 适用于需要在某个类型上实现某个trait接口，但里面又不想存储数据。





## struct数据的所有权

- 如含有String类型的字段，则该struct实例拥有其所有的所有权，只要struct实例有效，则字段数据有效。
- struct也可以存放引用，这需要使用生命周期（后续了解）
  - 生命周期保证只要struct实例有效，那么里面的引用也有效
  - struct里存储引用而不适用生命周期，就会报错。









# 2、struct例子

## 计算长方形面积

```rust
// 需要注意使用引用，不使用的话，main则不再具有r1的所有权
fn get_area( r:&Rectangle ) -> i32 {
    r.len * r.wid
}

// 自己写一下试试
struct Rectangle{
    len : i32,
    wid: i32
}
```



## 打印的格式

- std::fmt::Display，即println宏中使用{}来打印变量。
- std::fmt::Debug，即println宏中使用{:?}和{:#?}来打印标量。
- #[derive(Debug)]，写在struct上，即可使该struct实现Debug的格式打印
- {:?}，打印为一行
- {:#?}，有格式的打印





# 3、struct方法

- 方法和函数类似：fn关键字、函数名、形参列表、返回值。
- 方法和函数的不同：
  - 方法实在struct（或enum、strait对象）的上下文中定义
  - 方法的第一个参数时self，表示被调用的struct实例。



## 定义方法

- ```rust
  #[derive(Debug)]
  struct Rectangle{
      len:u32,
      wid:u32
  }
  
  impl Rectangle{
      fn eare(&self) -> u32 {
          return self.len * self.wid;
      }
  }
  ```

- 在impl 结构体名{}，块中定义。

- 方法的第一个参数可以是&self，也可获得其所有权，也可以可变借用&mut。



## 方法调用的运算符

- C++中：对象.方法和指针->方法，都是调用对象的方法。

- Rust没有 -> 运算符。

- Rust提供自动引用或解引用。

  - 在调用方法时发生

- 在调用方法时，Rust根据情况自动添加&、&mut 或 *，以便可以匹配方法的签名。

  ```rust
  println!("{},{}",rect.eare(),(&rect).eare());
  ```

  效果相同



## 方法参数

- 可以有多个参数

  ```rust
      fn is_contain(&self,other:&Rectangle) -> bool {
          if self.len > other.len && self.wid > other.wid{
              return true;
          }
          false
      }
  ```





## 关联函数

- 可以在impl块里定义不把self作为第一个参数的函数，他们叫关联函数。（不是方法，不在实例上调用）

  如String::from()

- 关联函数常用于构造器

  ```RUST
      fn square(size:u32) -> Rectangle{
          Rectangle{
              len:size,
              wid:size
          }
      }
  
  	// 调用时，类型名 :: 函数()，调用形式就如静态函数
      println!("关联函数：{:?}",Rectangle::square(5));
  ```

- :: 符号

  - 用于关联函数
  - 用于模块创建的命名空间



## 多个impl块

- 每个struct可以拥有多个impl块。







