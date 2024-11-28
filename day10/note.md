# 标题：泛型、Trait、生命周期



# 1、提取函数以消除重复代码

- 把常用的功能封装为函数



# 2、泛型



## 泛型

- 泛型可以提高代码复用能力，用于处理重复代码的问题

- 泛型是具体类型或其他属性的抽象代替

  - 编写的泛型代码不是最终的代码，而是一种模板，里面有一些占位符
  - 编译器编译时，会将“占位符”替换为具体的类型

- 例如：

  ```rust
  fn largest <T,U,D> (list:&[T]) -> T {...}
  ```

- 类型参数：

  - 通常用一个字母表示
  - 通常使用CamelCase驼峰命名法



## 函数定义中的泛型

- 泛型函数通常用于指定
  - 参数类型
  - 返回类型



## struct定义中的泛型

- 结构体名后使用\<T>



## enum 定义中的泛型

- 可以让枚举的变体持有泛型数据类型
  - 如Option\<T>，Result\<T,E>



## 方法定义中的泛型：结构体和枚举

- 将T放在impl关键字后，表示在类型T上实现方法

  ```rust
  impl<T> Point<T> {
      fn x(&self) -> &T {
          &self.x
      }
  }
  
  // 实现特定的T
  impl Point<i32> {
      fn y(&self) -> &i32 {
          &self.y
      }
  }
  ```

- struct里的泛型类型参数可以和方法的泛型类型参数不同

  ```rust
  #[derive(Debug)]
  struct PointTwo<T,U> {
      x: T,
      y: U,
  }
  
  impl<T,U> PointTwo<T,U> {
  
      fn mix<V,W>(self,other:PointTwo<V,W>) -> (PointTwo<T,W>,PointTwo<V,U>) {
          (
              PointTwo{
                  x: self.x,
                  y: other.y
              },
              PointTwo{
                  x: other.x,
                  y: self.y
              }
          )
      }
  }
  ```



## 泛型代码的性能

- 使用泛型的代码和使用具体类型的代码运行速度一样
- 单态化：在编译时将泛型替换为具体类型的过程
  - 类似于C++，在编译后直接将模板代码实例化为具体类型的代码



# 3、Trait

- Trait告诉Rust编译器：某种类型具有哪些并且可以与其他类型共享的功能

- Trait：抽象的定义共享行为

- Trait bounds(约束)：泛型类型参数指定为实现了特定行为的类型

- Trait 与 其他语言的接口类似，但有区别

- 使用Trait时，需要将其导入到作用域下

  ```rust
  use test3::{Car, Dog, ToString}; // ToString为Trait
  ```

  



## 定义一个Trait

- Trait的定义：把方法签名放在一起，来定义实现某种目的所必需的一组行为
  - 关键字：trait
  - 只有方法签名，没有具体实现
  - trait可以有多个方法：每个方法签名占一行，以分号；结尾
  - 实现该trait的类型必须提供具体的方法实现

- ```rust
  // 将类型转变为String字符串
  trait ToString {
      fn to_string(&self) -> String; 
  }
  ```



## 在类型上实现trait

- 与为类型实现方法类似

- 不同之处：

  - ```rust
    impl XTrait for SomeType { ... }
    ```

  - 在impl块里，需要对trait里的方法签名进行具体的实现

- 例子：让类型实现to_string方法

  ```rust
  // 将类型转变为String字符串
  pub trait ToString {
      fn to_string(&self) -> String; 
  }
  
  
  pub struct Car {
      pub version: String,
      pub name: String,
      pub price: f32,
      pub brand: String
  }
  
  pub struct Dog {
      pub name: String,
      pub host: String,
      pub age: u8
  }
  
  
  impl ToString for Car {
      fn to_string(&self) -> String {
          format!("Car:{{version={},\n\tname={},\n\tprice={},\n\tbrand={}}}",self.version,self.name,self.price,self.brand)
      }
  }
  
  impl ToString for Dog {
      fn to_string(&self) -> String {
          format!("Dog:{{name={},\n\thost={},\n\tage={}}}",self.name,self.host,self.age).to_string()
      }
  }
  
  // 调用
  fn test1(){
      let car = Car {
          version: String::from("1.0"),
          name: String::from("苏7"),
          price: 13.40,
          brand: String::from("小米"),
      };
  
      let dog = Dog {
          name: String::from("小苟"),
          host: String::from("LoongTory"),
          age: 6
      };
  
      println!("{}",car.to_string());
      println!("{}",dog.to_string());
  }
  ```

  

## 实现trait的约束

- 可以在某个类型上实现某个trait的前提条件是：
  - 这个类型 或 这个 trait实在本地crate里定义的
  
    人话：自己定义的类型，可以实现别人的trait或者自己本地写的trait
  
- 无法为外部类型来实现外部的trait：
  - 这个限制是程序属性的一部分（一致性）
  - 就是不能修改别人提供的库的类型（如std标准库）





## 默认实现

- 在trait中实现，在实现中不重写即可

  ```rust
  // trait
  pub trait ToStringByDefault {
      fn to_string_default(&self) -> String {
          String::from("默认实现")
      }
  }
  // 实现
  impl ToStringByDefault for Dog {}
  impl ToStringByDefault for Car {
      fn to_string_default(&self) -> String {
          format!("Car: {{ {},{},{},{} }}",self.version,self.name,self.price,self.brand)
      }
  }
  ```

- 默认实现的函数可以调用没有默认实现的函数，但类型实现该tarit时，必须实现被默认实现的函数调用的函数

  ```rust
  
  pub trait CatTrait {
      fn test2(&self);
  
      fn test1(&self) {
          println!("默认实现中调用：未实现的函数");
          self.test2();// self.test2调用的函数是 test2(&self)
      }   
  }
  pub struct Cat {
      pub name:String,
      pub age: u8
  }
  
  impl CatTrait for Cat {
      fn test2(&self) {
          println!("test2");
      }
  }
  ```

- 注意：无法从方法的重写实现里面调用默认的实现。



## Trait作为参数

- impl Trait 语法：impl trait 作为参数形象，但适用于简单情况

  ```rust
  pub struct HomeLand {
      pub name: String,
      pub age: u8
  }
  
  pub struct DouBao {
      pub name: String,
      pub model: String
  }
  
  pub trait MyDisplay {
      fn to_string(&self) -> String;
  }
  
  impl MyDisplay for HomeLand {
      fn to_string(&self) -> String {
          format!("name={},age={}",self.name,self.age)
      }
  }
  
  impl MyDisplay for DouBao {
      fn to_string(&self) -> String {
          format!("name={},model={}",self.name,self.model)
      }
  }
  
  pub fn MyPrintln(data: impl MyDisplay) {
      println!("{}",data.to_string());
  }
  
  fn test4() {
      let hl = HomeLand {
          name: String::from("祖国人"),
          age: 43
      };
  
      let db = DouBao {
          name: String::from("豆包"),
          model: String::from("模型")
      };
  
      MyPrintln(hl);
      MyPrintln(db);
  }
  ```

- Trait bound 语法：适用于于复杂情况

  上面的优化

  ```rust
  pub fn my_println<T:MyDisplay>(data: T) {
      println!("{}",data.to_string());
  }
  ```

  - 多参数时，简洁一点

    ```rust
    pub fn good1_def(data1: impl MyDisplay, data2: impl MyDisplay) {
        println!("{}",data1.to_string());
        println!("{}",data2.to_string());
    }
    pub fn good1<T:MyDisplay>(data1: T, data2: T) {
        println!("{}",data1.to_string());
        println!("{}",data2.to_string());
    }
    ```

- 使用 + 指定多个 Trait bound 

  ```rust
  pub fn good2_def(data1: impl MyDisplay + Display, data2: impl MyDisplay + Display) {
      println!("{}",data1.to_string());
      println!("{}",data2.to_string());
  }
  pub fn good2<T:MyDisplay + Display>(data1: T, data2: T) {
      println!("{}",data1.to_string());
      println!("{}",data2.to_string());
  }
  ```

- Trait bound 使用 where 子句：优化类型指定太多，函数签名更清楚了

  ```rust
  pub fn notify1<T: MyDisplay + Display, U: Clone + Debug> ( d1: T, d2: U ) -> String {
      format!("{},{:?}",d1,d2)
  }
  
  //where 优化
  pub fn notify1_plus<T,U>( d1: T, d2: U ) -> String 
  where
      T: MyDisplay + Display,
      U: Clone + Debug
  {
      format!("{},{:?}",d1,d2)
  }
  ```

  哈哈，听说从C#学过来的



## 实现Trait作为返回类型

- impl Trait 语法

  ```rust
  pub fn return_trait(num : i32) -> impl Display {
      if num >= 10 {
          return String::from("XXX")
      }
      
      808.to_string()
  }
  ```

- 注意：impl Trait 只能返回确定的同一类型，返回的类型不同，代码报错



## 使用 Trait Bound

- 使用Trait Bound 修复 largest 函数

  ```rust
  // 修复前
  fn largest<T>(list: &[T]) -> T {
      let mut largest = list[0];
      for &item in list.iter() {
          if item > item {
              largest = item
          }
      }
      largest
  }
  
  // 修复后
  fn largest<T: PartialOrd>(list: &[T]) -> T {
      let mut largest = list[0];
  
      for &item in list.iter() {
          if item > largest { //  > 是std::cmp::PartialOrd这个trait
              largest = item
          }
      }
  
      largest
  }
  // 让T实现Copy，但如果数组元素类型是String，还是有问题
  fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
      let mut largest = list[0];// 此处T类型没有实现Copy，报错
  
      for &item in list.iter() {
          if item > largest { //  > 是std::cmp::PartialOrd这个trait
              largest = item
          }
      }
  
      largest
  }
  // 使用Clone
  fn largest1<T: PartialOrd + Clone>(list: &[T]) -> T {
      let mut largest = list[0].clone();// 此处T类型没有实现Copy，报错
  
      for item in list.iter() {
          if item > &largest { //  > 是std::cmp::PartialOrd这个trait
              largest = item.clone()
          }
      }
  
      largest
  }
  // 也可以直接返回引用
  fn largest2<T: PartialOrd + Clone>(list: &[T]) -> &T {
      let mut largest = &list[0];// 此处T类型没有实现Copy，报错
  
      for item in list.iter() {
          if item > &largest { //  > 是std::cmp::PartialOrd这个trait
              largest = item
          }
      }
  
      largest
  }
  ```
  
  

## 使用Trait Bound 有条件的实现方法

- 在使用泛型类型参数的impl块上使用 Trait bound，我们可以有条件的为实现了特定Trait的类型来实现方法

  ```rust
  pub struct Mouse<T> {
      x: T,
      y: T
  }
  // 所有Mouse泛型都有
  impl<T> Mouse<T> {
      fn new( x: T, y: T ) -> Self {
          Self {x,y}
      }
  }
  // T实现Clone和Display才有
  impl<T: Clone + Display> Mouse<T> {
      fn in_clone_display() {
          println!("实现Clone和Display的类型才有")
      }
  }
  ```

- 也可以为实现了其他Trait的任意类型有条件的实现某个Trait

- 为满足Trait Bound 的所有类型实现Trait叫做覆盖实现

  标准库：对于所有实现Display Trait的类型，都为其实现ToString Trait，即提供to_string方法

  ```rust
  pub trait  MyToString {
      fn my_to_string(&self) -> String;
  }
  
  // 对所有实现了MyDisplay1的类型，都实现MyToString
  impl <T: Display> MyToString for T {
      fn my_to_string(&self) -> String {
          format!("my_to_string={}",self)
      }
  }
  
  //
  fn test5() {
      let  x = String::from("小白");
      println!("{}",x.my_to_string());
  }
  ```

  





# 4、生命周期

- Rust最与众不同的特征
- Rust的每个引用都有自己的生命周期
- 生命周期：引用保持有效的作用域
- 大多数情况：生命周期是隐式的、可被推断的
- 当引用的生命周期可能以不同的方式互相关联时：手动标注生命周期



## 生命周期-避免悬垂引用

- 生命周期的主要目标

  ```rust
  fn test1(){
      let x;
      {
          let a = 3;
          x = &a;
      }
      println!("{}",x);// x引用的对象已经释放了
  }
  ```



## 借用检查器

- 比较作用域来判断所有的借用是否合法
- 会计算各个变量的生命周期，如上一个中的x和a，检查器会检查x和a的生命周期，并且判断出x在a的生命周期结束后使用了a，所以出现错误



## 函数中的泛型生命周期

- 例子：

  ```rust
  fn test1(){
      let str1 = String::from("US");
      let str2 = "China";
      println!("longest={}",longest(str1.as_str(), str2));
  }
  
  fn longest(str1: &str, str2: &str) -> &str {
      if str1.len() > str2.len() {
          return str1
      }
      str2
  }
  ```

  报错为： this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `str1` or `str2`，即返回的类型是引用，但无法判断返回的值的生命周期是str1还是str2

  ```rust
  // 标注生命周期
  fn longest<'a> (str1: &'a str, str2: &'a str) -> &'a str {
      if str1.len() > str2.len() {
          return str1
      }
      str2
  }
  ```

  



## 生命周期标注

- 生命周期的标注不会改变引用的生命周期长度
- 当指定了泛型生命周期函数，函数可接受带有任何生命周期的引用
- 生命周期的标注：描述了多个引用的生命周期间的关系，但不影响生命周期



## 生命周期标注-语法

- 生命周期参数名：

  - 以' 开头
  - 通常使用全小写字母，且非常短
  - 惯例使用 'a

- 生命周期标注的位置：

  - 在引用的&符号后
  - 使用空格将标注和引用类型分开

- 例子：

  ```rust
  &i32
  &'a i32
  &'a mut i32
  ```



## 函数签名中的生命周期标注

- 在函数名和参数列表之间的<>里

- 生命周期'a的实际生命周期就是变量中生命周期较小的哪个

  ```rust
  fn test2() {
      let str1 = String::from("China");
      let ret;
      {
          let str2 = String::from("Us");
          ret = longest(str1.as_str(), str2.as_str());
          //^^^^ borrowed value does not live long enough
      }
      println!("{}",ret);
  }
  
  fn longest<'a> (str1: &'a str, str2: &'a str) -> &'a str {
      if str1.len() > str2.len() {
          return str1
      }
      str2
  }
  ```



## 深入理解生命周期

- 指定生命周期参数的方式依赖于函数

  ```rust
  fn longest1<'a> (str1: &'a str, str2: & str) -> &'a str {
      if str1.len() > str2.len() {
          return str1
      }
      str2
  }
  // 如果不依赖
  ```

- 从函数返回引用时，返回类型的生命周期参数需要与其中一个参数的生命周期匹配

- 如果返回的引用没有指向任何参数，那么它只能引用函数内创建的值：

  - 这就是悬垂引用：该值在函数结束时就走出了作用域

  ```rust
  fn longest2() -> &str {
      let x = String::from("123124");
      x.as_str()
  }
  ```

- 如果想返回函数内的变量，直接返回所有权

  ```rust
  fn return_pri() -> String {
      let x = String::from("value");
      x
  }
  ```



## Struct定义中的生命周期标注

- Struct里可包括

  - 自持有的类型
  - 引用：需要在每个引用上添加生命周期标注

  ```rust
  fn test3() {
      let baby = "小花猫";
      let mother = Cat{baby};
      
  }
  
  struct Cat<'a> {
      baby: &'a str
  }
  ```

  

## 生命周期的省略

- 每个引用都有生命周期

- 需要为使用生命周期的函数或struct指定生命周期参数

- 早些版本需要标注所有引用的生命周期，但可以有部分情况可以省略

  ```rust
  fn first_word<'a>(s:& str) -> &str {
      let bytes = s.as_bytes();
      for (i,item) in bytes.iter().enumerate() {
          if item == b' ' {
              return &s[0..i];
          }
      }
      &s[..]
  }
  ```

  

## 生命周期省略规则

- 在Rust引用分析中所编入的模式称为生命周期省略规则
  - 这些规则无需开发者来遵守
  - 是一些特殊情况，由编译器来考虑
  - 如果你的代码符合这些特殊情况，就无需显示标注生命周期
- 生命周期省略规则不会提供完整的推断：
  - 如果应用规则后，引用的生命周期仍然模糊不清，则会报错
  - 此时手动添加生命周期标注，来表明引用间的相互关系



## 输入、输出生命周期

- 输入：函数的参数
- 输出：函数的返回值



## 生命周期省略的三个规则

- 编译器使用3个规则来在没有显式标注生命周期的情况下，来确定引用的生命周期

  - 规则1用于输入生命周期
  - 规则2、3用于输出生命周期
  - 如果编译器应用完3个规则之后，仍然无法确定生命周期的引用-》报错
  - 这些规则适用于fn定义和impl块

- 规则1：每个引用类型的参数都有自己的生命周期

- 规则2：如果只有一个输入生命周期参数，那么该生命周期被赋给所有的输出生命周期参数

- 规则3：如果有多个输入生命周期参数，但其中一个是&self 或 &mut self，那么self的生命周期会被赋给所有的输出生命周期参数

- 例子1：

  ```rust
  fn first_word(s: &str) -> &str{}
  // 应用规则1后
  fn fw<'a>( s: &'a str ) -> &str {}
  // 应用规则2后
  fn fw<'a>( s: &'a str ) -> &'a str {}
  // 所有引用都有生命周期了
  ```

- 例子2：

  ```rust
  fn longest(x: &str, y: &str) -> &str {}
  // 应用规则1
  fn longest<'a,'b>( x: &str, y: &str ) -> &str {}
  // 规则2不适用
  // 应用规则3，没有self,没有变化
  ```

  

## 方法（结构体中的函数）定义中的生命周期标注

- 在struct上使用生命周期实现方法，语法和泛型参数的语法一样

- 在哪生命和使用生命周期参数，依赖于：

  - 生命周期参数是否和字段、方法的参数或返回值有关

- struct字段的生命周期名：

  - 在impl后声明
  - 在struct名后使用
  - 这些生命周期是struct类型的一部分

- impl块内的方法签名中：

  - 引用必须绑定与struct字段引用的生命周期，或引用是独立的也可以
  - 生命周期省略规则经常使得方法中的生命周期标注不是必须的

  ```rust
  struct Stu<'a> {
      name: &'a str,
  }
  
  impl<'a> Stu<'a> {
      fn name(&self) -> &str {
          &self.name
      }
  
      fn level(&self, age: &str ) -> &str {
          println!("{}",age);
          &self.name
      }
  }
  ```

  

## 静态生命周期

- 'static是一个特殊的生命周期：整个程序的运行时间
  - 例如：所有字符串字面量都拥有'static生命周期
- 为了性能：尽量考虑避免悬垂引用，而不是使用静态生命周期



## 泛型参数类型、Trait Bound、生命周期

- <'a,T>，可以同时存在泛型参数类型和生命周期标注...
