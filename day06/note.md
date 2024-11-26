# Title: 枚举与模式匹配

# 1、枚举

## 定义枚举

- 枚举允许我们列举所有可能的值来定义一个类型

- 使用 enum 枚举类型名 { 枚举值,... } 来定义枚举

  ```RUST
  enum IpAddrKind{
      V4,
      V6
  }
  ```



## 使用枚举值

- 使用 枚举类型名 :: 枚举值 来获取

  ```rust
      let ipv4 = IpAddrKind::V4;
      let ipv6 = IpAddrKind::V6;
      println!("{:?},{:?}",ipv4,ipv6);
  ```



## 将数据附加到枚举的变体中

- 即可以为每个枚举值定义它的类型，或者称绑定数据

  ```rust
  enum EnumIP{
      V4(u8,u8,u8,u8),
      V6(String)
  }
  
  	
  	let ip2 = EnumIP::V4(127, 0, 0, 1);
      let ip3 =   EnumIP::V6(String::from("::1"));
  ```

  



## 标准库中的IpAddr

- IpAddr:

  ```rust
  pub enum IpAddr {
      /// An IPv4 address.
      #[stable(feature = "ip_addr", since = "1.7.0")]
      V4(#[stable(feature = "ip_addr", since = "1.7.0")] Ipv4Addr),
      /// An IPv6 address.
      #[stable(feature = "ip_addr", since = "1.7.0")]
      V6(#[stable(feature = "ip_addr", since = "1.7.0")] Ipv6Addr),
  }
  ```

- V4和V6是两个不同类型的结构体

- 即枚举值绑定的值的类型可以是任何类型

- 例子：

  ```rust
  enum Msg{
      Quit,
      Move {x:i32, y:i32}, // 匿名结构体
      Write(String),
      ChangeColor(i32,i32,i32)
  }
  ```



## 为枚举定义方法

- 和struct一样，使用impl关键字

  ```rust
  impl Msg{
      fn call(&self){
          println!("{:?}",self);
      }
  }
  ```

  



# 2、Option枚举

- 定义于标准库中
- 在Prelude(预导入模块)中
- 描述了：某个值可能存在（某种类型）或不存在的情况



## Rust没有Null

- 其他语言中：
  - Null是一个值，表示没有值
  - 一个变量处于两种状态：空值(null)、非空
- Null引用：Billion Dollar Mistake，百万级别的错误
- Null的问题在于：当Null与非Null进行某些运算时，就会引起错误，如字符串连接。
- Null的概念仍是有用的：因某种原因而变为无效或缺失的值。



## Rust中类似Null概念的枚举-Option\<T>

- 标准库的定义：

  ```rust
  pub enum Option<T> {
      /// No value.
      #[lang = "None"]
      #[stable(feature = "rust1", since = "1.0.0")]
      None,
      /// Some value of type `T`.
      #[lang = "Some"]
      #[stable(feature = "rust1", since = "1.0.0")]
      Some(#[stable(feature = "rust1", since = "1.0.0")] T),
  }
  ```

  - Some：关联一个T类型，T是泛型参数

- 可以直接使用,即不用 枚举类型::枚举值 。

  - Option\<T>
  - Some(T)
  - None

  ```rust
      let some_num = Some(5);
      let some_str = Some(String::from("value"));
  
      let none_num:Option<u32> = None;
  ```

  - 使用None时，需要显示指定类型，因为无法推断出其类型。



## Option\<T>比Null好在哪？

- Option\<T>和T是不同类型，不可以将Option\<T>直接当作T

  ```rust
  // let sum = 4 + some_num;
  ```

- 若想使用Option\<T>中的T，必须将它转换为T

  - 即其他语言同类型变量进行运算时，变量可能为Null，但任何编译成功。
  - 但在Rust中，只要两个变量不是Option\<T>，那么永远不会出现Null。

- 可以使用unwrap()函数来获取Option\<T>的值

- 综上机制：类似String类型，本身不存在Null值，所有只要两个变量都是String，就不可能出现Null值参与运算。







# 3、控制流运算符-match

## match

- 允许是一个值与一系列模式进行匹配，并根据匹配到的模式，执行对应的代码

- 模式可以是字面值、变量名、通配符

- match 表达式 { 模式n => 需要执行的代码 , ... } ，不同分支使用，逗号分隔

  ```rust
  enum Coin{
      One,
      Five,
      Ten,
      Twenty
  }
  
  fn value_in_cents(coin:Coin) -> u8 {
      match coin {
          Coin::One => 1, // 执行的代码不是一行，使用{}括起
          Coin::Five => 5,
          Coin::Ten => 10,
          Coin::Twenty => 20
      }
  }
  ```

  函数分析：功能为将枚举值转变为对应的数值。match{}中，每个分支都会返回值，该值作为match{}的值，由于match{}后无分号；，所以分支中返回的值就是match{}返回的值，即函数返回的值。
  
  ```rust
  	let ret = match one {
          Coin::One => {
              println!("一元");
              1
          }
          Coin::Five => 5,
          Coin::Ten => 10,
          Coin::Twenty => 20
      };
  ```
  
  使用{}后不需要使用,逗号分隔不同的分支





## 绑定值的模式

- 匹配的分支可以绑定到被匹配对象的部分值。

  - 因此可以从enum变体中提取值

  ```rust
  fn test2(){
      let coin = NewCoin::Ten(Province::Henan);
      let ret = match coin {
          NewCoin::One => 1,
          NewCoin::Two => 2,
          NewCoin::Five => 5,
          NewCoin::Ten(province) => { // province即coin绑定的数据
              println!("from {:?}",province);
              10
          }
      };
      println!("ret={}",ret);
  }
  
  #[derive(Debug)]
  enum Province{
      Henan,
      Shaanxi
  }
  
  enum NewCoin{
      One, 
      Two,
      Five,
      Ten(Province)
  }
  ```

  即枚举里有些枚举值可以绑定一些其他类型的数据，那么匹配时，可以给这些数据起名，然后访问这些数据。





## 匹配Option\<T>

- ```rust
  fn test3(){
      let x = Some(32);
      println!("{:?}",add_one(x));
  
      let x : Option<i32> = None;
      println!("{:?}",add_one(x));
  }
  
  // 传入一个option，如果不为None则自增1
  fn add_one(x : Option<i32> ) -> Option<i32>{
      match x {
          Option::None => None, // 可以使用Option::获取，也可以直接写
          Some(num) => Some(num+1)
      }
  }
  ```

  

## match匹配必须穷举所有的可能<a id="question1"></a>

- _通配符：替代其余没有列出的值

  ```rust
  fn test4(){
      match 5 {
          _ => 1
      };
  }
  ```

- 变量名取代其余值

  ```rust
      let x = MyEnum::HeiHei;
      let x = match x {
          MyEnum::Ok => 1,
          MyEnum::Err => 0,
          other => -1
      };
      println!("{}",x);
  ```

  





# 4、if-let

- 处理只关心一种匹配而忽略其他匹配的情况

- if let 匹配值 = 表达式 { 执行代码 }

  ```rust
      let num = 16;
      // 只想知道是不是4的倍数
      match num%4 {
          0 => println!("Yes"),
          _ => ()
      };
  
      // 只针对一种模式匹配，使用if let 简化
      if let 0 = num%4 {
          println!("Yes");
      }
  ```

- 更少的代码，更少的缩进，更少的模板代码

- 放弃了穷举的可能性

- 也可以返回值，但必须配合else使用，以此保证必须返回值。<a id="question2"></a>

  ```rust
  let ret = if let 0 = num%4 {
          0
      } else {
          1
      };
      println!("ret={}",ret);
  ```

  



# 遇到的小问题

- [x] [使用match匹配时，如果表达式的结果为枚举类型，必须将枚举的所有值匹配?](#question1)
- [x] [if-let 和match一样返回值吗？](#question2)



