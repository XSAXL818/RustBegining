# 标题：函数式语言特性：迭代器和闭包

# 1-1、闭包

## 什么是闭包

- 闭包：可以捕获其所在环境的匿名函数
  - 是匿名函数
  - 保存为变量，作为参数
  - 可在一个地方创建闭包，然后在另一个上下文中调用闭包来完成运算
  - 可以从其定义的作用域捕获值



## 例子：生成自定义运动计划的程序

- 算法的逻辑不是重点，重点是算法中的计算过程需要几秒钟事件等待
- 目标：不让用户发生不必要的等待
  - 仅在必要时调用该算法
  - 只调用一次

```rust
// 一个长时间的函数
fn long_time() -> type {}
// 待优化的情况
if * { // 优化一：每个分支都使用了long_time函数，可以在if外先保存该结果
    long_time();
    long_time();
} else {
    if * {
        
    } else {
        long_time();
    }
}
// 优化一
let ret = long_time();
if * { 

} else {
    if * {
        
    } else { // 分支2.2
        
    }
}
// 由于分支2.2本身不执行long_time函数，但当前函数肯定执行，所以将ret赋值为一个函数
// 优化一
let long_time = |p1,p2...|{ 函数体 };
if * { // 此处后续优化....
	long_time();
    long_time();
    
} else {
    if * {
        long_time();
    } else { // 分支2.2
        
    }
}
```



# 1-2、类型推断和标注

## 闭包的类型推断

- 闭包不要求标注参数和返回值的类型

- 闭包通常很小，只在狭小的作用域内工作，编译器一般可以i推断出类型

  ```rust
  let func = |num:i32| -> i32 {}
  ```

  ![image-20241203200435002](D:\Code\RustCode\RustBegining\day13\assets\image-20241203200435002.png)

- 闭包的定义最终只会为参数/返回值推断出唯一具体的类型

  ```rust
      let func = |x| x;
      let str = func(String::from("value"));
      let num = func(10);
  ```

  ![image-20241203200713253](D:\Code\RustCode\RustBegining\day13\assets\image-20241203200713253.png)



## 函数和闭包的定义语法

- 上次运动计划程序的优化

  ```rust
  let long_time = |p1,p2...|{ 函数体 };
  if * { //分支1： 此处后续优化....
  	long_time();
      long_time();
      
  } else {
      if * {
          long_time();
      } else { 
          
      }
  }
  // 优化方案1：将返回结果保存，然后使用
  // 视频上说可能代码代码冗余，目前不知道。。。，不过这在Java、Python中应该
  // 可以使用，毕竟开发效率第一
  ```

- 另一种优化方法：创建一个struct，持有闭包和其调用结果

  - 只会在需要结果时才执行闭包
  - 可缓存结果

- 这个模式通常叫做记忆化或延迟计算



## 如何让struct持有闭包

- struct的定义需要知道所有字段的类型
  - 所以需要指定闭包的类型
- 每个闭包实例都有自己唯一的匿名类型，即使两个闭包签名相同
- 需要使用：泛型和Trait Bound来实现



# 1-3、使用泛型和Fn Trait存储闭包

## Fn Trait

- Fn Trait 由标准库提供

- 所有的闭包都至少实现了一下trait之一：

  - Fn
  - FnMut
  - FnOnce

  ```rust
  struct Load<T>
  where T: Fn(i32) -> i32 
  {
      pub calcu: T,
      pub value: Option<i32>
  }
  
  impl<T> Load<T> 
  where T : Fn(i32) -> i32 
  {
      fn new(calcu:T) -> Load<T> {
          Load{
              calcu,
              value:None
          }
      } 
  
      fn value(&mut self, arg: i32) -> i32 {
          match self.value {
              Some(v) => v,
              None => {
                  let x = (self.calcu)(arg);
                  self.value = Some(x);
                  x
              }
          }
      }
  }
  ```

  就是自己封装一个结构体，类似于单例模式，只让所得结果存在一份，属于懒人法的一种了。

  ```rust
  // 优化后
  let load = Load::new(|p1,p2...|{ 函数体 });
  if * { //分支1： 此处后续优化....
  	load.value();
      load.value();
  } else {
      if * {
          load.value();
      } else { 
          
      }
  }
  ```

  确实比用一个变量接受要来得方便和整洁



## 使用缓存器Cacher实现的限制

- Cacher实例假定针对不同的参数arg，value方法总会得到相同的结果
  - 可以使用HashMap代替单个值：
    - key：arg参数
    - value：执行闭包的结果
- 上个例子只接受固定的参数类型和返回固定的类型
  - 可以引入多个泛型参数



# 1-4、使用闭包捕获上下文

## 闭包可以捕获他们所在的环境

- 闭包可以访问定义它的作用域内的变量，普通函数不能

  ```rust
  	let x = 14;
      let bb = |ret| {
          ret == x
      };
      let y = 10;
      println!("y=={}---{}",x,bb(y));
  ```

- 会产生内存开销



## 闭包从所在环境捕获值的方式

- 与函数获得参数的方式相同
  - 获得所有权：FnOnce
  - 可变借用：FnMut
  - 不可变借用：Fn
- 创建闭包时，通过闭包对环境值的使用，Rust会推断出使用哪个trait
  - 所有的闭包都是先实现了FnOnce
  - 没有移动捕获变量的是实现了FnMut
  - 没有可变访问捕变量的闭包实现了Fn
  - Fn > FnMut > FnOnce



## move关键字

- 在参数列表前使用move关键字，可以强制闭包取得它所使用的环境值的所有权

  - 适用于将闭包传递给新线程以移动数据使其归新线程所有的

  ```rust
      let x = String::from("value");
      
      // 假设线程一
      {
          let cacher = move |src:String|  src.contains(&x);
          let src = String::from("new value");
          println!("contains:{}",cacher(src));
      }
  	// 所有权已经转移了
  	// println!("{}",x);
  ```



## 使用准则

- 当指定Fn trait bound之一时，首先用Fn，基于闭包体的情况，编译器会告知是否需要FnOnce或FnMut





# 2-1、迭代器

## 什么是迭代器

- 迭代器模式：对一系列项执行某些任务
- 迭代器负责：
  - 遍历每个项
  - 确定序列何时完成
- Rust的迭代器
  - 懒惰的：除非调用消费迭代器的方法，否则迭代器本身没有任何效果

```rust
    let v = vec![1,2,3,4,5];
    let it = v.iter();
    for elem in it {
        println!("{}",elem);
    }
```



# 2-2、Iterator trait 和 next 方法

## Iterator trait

- 所有迭代器都实现了Iterator trait

- Iterator trait 定义于标准库，定义大致如下：

  ```rust
  
  pub trait Iterator {
      /// The type of the elements being iterated over.
      #[rustc_diagnostic_item = "IteratorItem"]
      #[stable(feature = "rust1", since = "1.0.0")]
      type Item;
      #[lang = "next"]
      #[stable(feature = "rust1", since = "1.0.0")]
      fn next(&mut self) -> Option<Self::Item>;
  ```

- type Item 和 Self：：Item 定义了与该trait关联的类型

  - 目前只需了解：实现Iterator trait 需要你定义一个Item类型，它用于next方法的返回类型（迭代器返回的类型）



## Iterator trait

- Iterator trait 仅要求实现一个方法：next

- next：

  - 每次调用返回迭代器中的一项
  - 返回结果包括在Some中
  - 迭代结束，返回None

- 可直接在迭代器上调用next方法

  ```rust
      let v = vec![1,2,3,4,5];
      let mut it = v.iter();// 使用mut修饰
      loop {
          match it.next() {
              Some(ret) => println!("ret={}",ret),
              None => break
          }
      }
  ```

  、



## 几个迭代方法

- 使用iter方法，获取的是不可变引用，不能通过该迭代器修改元数据

  ![image-20241204024720554](D:\Code\RustCode\RustBegining\day13\assets\image-20241204024720554.png)

- into_iter：创建的迭代器会获取所有权

  ![image-20241204025013361](D:\Code\RustCode\RustBegining\day13\assets\image-20241204025013361.png)

- iter_mut方法：迭代可变引用

  ![image-20241204025205033](D:\Code\RustCode\RustBegining\day13\assets\image-20241204025205033.png)

  



# 2-3、消耗和产生迭代器的方法

## 消耗迭代器的方法

- 在标准库中，Iterator trait 有一些带默认实现的方法
- 其中一些方法会调用next方法
  - 实现Iterator trait 时必须实现next方法的原因之一
- 调用next的方法叫做"消耗型适配器"
  - 因为不断调用，会将迭代器的元素耗尽，无法恢复。玉米棒，掰掉就没有啦
- 如：sum方法（耗尽迭代器）
  - 取得迭代器的所有权
  - 通过反复调用next，遍历所有元素
  - 每次迭代，把当前元素添加到一个总和里，迭代结束，返回总和

```rust
    let v = vec![1,2,3,4];
    let it = v.iter();
    let mut sum:i32 = it.sum();
    println!("sum={}",sum);

    // sum = it.sum();// 上一次sum调用，已经耗尽该iter了
```



## 产生其他迭代器的方法

- 定义在Iterator trait 上的另外一些方法叫做"迭代器适配器"

  - 把迭代器转换为不同种类的迭代器

- 可以通过链式调用使用多个迭代器适配器来执行复杂的操作，这种调用可读性较高

- 例如map：

  - 接受一个闭包，闭包作用于每个元素
  - 返回一个新的迭代器

  ```rust
      let v1 = vec![1,2,3,4];
      let it = v1.iter();
      // 使用map获取一个新的迭代器，使用collect并显式指定类型，来获取指定类型的Vec
      let v2:Vec<String> = it.map(|x| format!("{}",x) ).collect();
      for elem in v2.iter(){
          println!("{:?}",elem);
      }
  ```

- collect方法：消耗型适配器，把结果收集到一个集合中，常用于链式调用



# 2-4、使用闭包捕获环境

- filter方法：

  - 接受一个闭包
  - 这个闭包在遍历迭代器的每个元素时，返回bool类型
  - 如果闭包返回true：当前元素将会包含在filter产生的迭代器中
  - false：当前元素不在新产生的迭代器中
  - 一个筛选函数，满足要求留下

  ```rust
  #[derive(Debug,PartialEq)]
  struct NiuMa {
      age: u8,
      school: String
  }
  
  impl NiuMa {
      fn new(age:u8,school:String) -> NiuMa {
          NiuMa {
              age,
              school
          }
      }
  }
  // 会将vec的所有权拿走，建议传入迭代器
  fn get_niu_ma(niu_ma:Vec<NiuMa>) -> Vec<NiuMa> {
      niu_ma.into_iter().filter(|x| {
          x.age < 35 && x.school != "孀妃"
      }).collect()
  }
  
  fn get_niu_ma_it<I:Iterator<Item = NiuMa>>(it:I) -> Vec<NiuMa> {
      it.filter(|x| {
          x.age < 35 && x.school != "孀妃"
      }).collect()
  }	
  
  	let v = vec![NiuMa::new(22, String::from("孀妃")),
          NiuMa::new(23, String::from("二妖妖")),
          NiuMa::new(36, String::from("酒吧舞")),
      ];
  
      let niu_ma:Vec<&NiuMa> = v.iter().filter(|x| {
          x.age < 35 && x.school != "孀妃"
      }).collect();
  
      let it = v.iter().filter(|x| {
          x.age < 35 && x.school != "孀妃"
      });
  
  
      println!("符合我公司的人财：");
      for i in niu_ma.iter() {
          println!("{:?}",i);
      }
  
      println!("通过迭代器：");
      for i in it {
          println!("{:?}",i);
      }
  
      println!("通过函数(迭代器传参)：");
      // 上面有引用，不能在上面调用
      let niu_ma3 = get_niu_ma_it(v.into_iter());
      for i in niu_ma3.iter() {
          println!("{:?}",i);
      }
  ```

- filter根据迭代器的类型(iter、into_iter、iter_mut)，返回不同元素类型的迭代器



# 2-5、使用Iterator trait 创建自定义迭代器

- 实现next方法

  ```rust
  struct Point{
      point: [i32;3],
      index: u8
  }
  impl Point {
      fn new() -> Point {
          Point {
              point: [1,100,200],
              index: 0,
          }
      }
  }
  
  impl Iterator for Point {
      type Item = i32;
      
      fn next(&mut self) -> Option<Self::Item> {
  
          let x:usize = (self.index).into();
          if x < 3 {
              self.index += 1;
              Some(self.point[x])
          } else {
              self.index = 0;
              None
          }
          
      }
  }
  
  // 可通过for循环遍历迭代器
      println!("遍历坐标系：");
      let pp = Point::new();
  // 实现next后，默认提供into_iter方法，交出所有权
      for i in pp.into_iter() {
          println!("{}",i);
      }
  ```

  





# 3、使用迭代器和闭包改进第12章的项目

## 优化点：

- 阶段1获取的值是从迭代器转化的vec，阶段二再使用vec的固定索引位置的值。

  即整个过程只需要args的迭代器即可实现，阶段二使用next方法获取值。

- ```rust
  let args: Vec<String> = env::args().collect();
  ```

  不适用collect函数

  ```rust
   let args = env::args();
      // 当前函数不接受非法的UTF-8的字符
      // 如果出现，程序会恐慌。使用args_os接受非法的UTF-8
      let config = Config::new(args).unwrap_or_else(|err| {
          eprintln!("程序出错：{}",err);
          process::exit(1);
      });
  ```

- ```rust
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
  ```

  通过迭代器实例化结构体，而不是索引值的方式

- ```rust
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
  ```

  如果和fillter功能相同，可以使用fillter来简化代码









# 4、性能比较：循环vs迭代器

- 迭代器更快

## 零开销抽象

- 使用抽象时，不会引入额外的运行时开销
- 







