# 所有权

- 是Rust最独特的特性，它能让Rust无需GC（垃圾回收机制）就可以保证内存安全。
  - GC:程序运行时，会自动寻找不再使用的内存，然后释放。
  - 其他语言，需要程序员显式地分配和释放内存。
- Rust采用第三种方式：
  - 内存通过所有权系统来管理，其中包含一组编译器在编译时检查的规则。
  - 当程序运行时，所有权特性不会减慢程序的运行速度。



## 1、栈内存和堆内存

- 在Rust中，一个值在stack和在heap堆程序来说有很大的影响。
- 代码运行时，stack和heap都是可用的内存，但结构不相同。



### 存储数据的区别

- stack即先进后出。
- 所有存在在stack上的数据必须拥有已知的固定大小。
- heap内存组织性较差。通常操作：申请一定数量的空间，操作系统在heap中找到，然后返回该空间的首地址，该过程通常称为分配。
- 数据压到stack上比在heap上分配快得多。



### 访问数据的区别

- 访问heap中的数据要比访问stack的数据满，因为需要通过指针。
- 数据存放的距离较近时，stack速度更快。



### 函数调用

- 代码调用函数时，值被传入到函数。函数本地的变量被压到stack上。函数结束后，值被弹出。



### 所有权存在的原因

- 所有权要解决的问题：
  - 跟踪代码的哪些部分正在使用heap的哪些数据
  - 最小化heap上的重复数据量
  - 清理heap上未使用的数据以避免空间不足
- 一旦使用所有权，不需要经常关注stack和heap
- 管理heap数据是所有权存在的原因



## 2、所有权的规则、内存和分配

### 所有权规则

- 每个值都有一个变量，这个变量是该值的所有者。

- 每个值同时只能有一个所有者

- 当所有者超出作用域时，值被删除（emmm，目前只看字面的话，这个域在C++使用{}就可以实现？），作用域（scope）就是程序中一个项目的有效范围。

  即变量只在{}内是可用的，{}外不可以访问{}内的元素

  ```rust
  let mut a = 3;
  {
      a = 5;// 可行，{}外的变量，在{}内可以访问
      let mut b = 4;
  }
  b = 4;// 不可行
  ```



### String类型

- 字符串字面量：即程序中手写的那些字符串值。它们是不可变的。
- Rust的第二种字符串类型：String。在heap上分配，能够存储在编译时未知数量的文本。

#### 创建String类型的值

- 可以是用from函数从字符串字面值创建出String类型

- let s = String::from(“hello”);

- 该字符串可以被改变。

- &str类型，切片字符串，即所有字符串字面量都是唯一存在的，如"world"和"hello"，只存在一份。&str类型的变量包含起始和结束地址，以此指向固定长度的字符串字面量。

  ```rust
  let mut ss = "hello";
  println!("{:p}",ss);// 
  ss = "world1";
  println!("{:p}",ss);// 地址变了，即指向"world"了
  ss = "hello";
  println!("{:p}",ss);// 变回最大时的地址
  
  
  
  let mut s = String::from("hello");
  println!("{:p}",&s);
  s.push_str(", xsaxl");
  println!("{:p}",&s);
  ```

  

#### 内存和分配

- 字符串字面值，在编译时就知道其内容，可直接硬编码到最终的可执行文件中。因此速度快、高效。
- String类型，为了支持可变性，需要在heap上分配内存来保存编译时未知的文本内容。运行时向操作系统请求内存，用完后，将内存返回。
- Rust采用的方式：对于某个值，当拥有它的变量走出作用范围时，内存会立即自动交换给操作系统。
- drop函数：变量内存释放时调用。



### 变量和数据交互的方式：移动（Move）

- 多个变量可以与同一个数据使用一种特殊的方式来交互。

- ```
  let x = 3;
  let y = x;
  println!("x = {}", x);// 整数这种类型，会自动将3压入stack中
  ```

- ```rust
  let x = String::from("heihei");
  let y = x;
  println!("x = {}", x);// x的语义已经移动给y了，x失效了。原因：String没有实现copy
  ```

  - String的内存分布，一个String对象是一个保存在stack上的结构，里面有三个基础变量（指针，长度，容量），指针指向堆中的一段内存，该内存即使String的字符串。
  - 当字符串对象被赋值给另一个对象，前者会失效，后者复制一份前者在stack上的数据，前者离开作用域后，Rust释放指针指向的内存。
  - ![image-20241117210613139](D:\Code\RustCode\RustBegining\day04\assets\image-20241117210613139.png)

- 浅拷贝：只拷贝对象自身地址指向的结构的内存。

- 深拷贝：浅拷贝的基础上，将指针指向的内容也进行拷贝。

- 个人理解：浅拷贝是直接寻址，深拷贝是间接寻址。

- ```
      let x = String::from("heihei");
      let y = x.clone();// 将heap上的内存也拷贝了
      println!("x = {} , y = {}",x,y);
  ```

  

###   Stack上的数据：复制

- Copy trait，实现了这个接口，就可以像使用整数一样，就变量赋值给其他变量后，就变量依然可用。
- 如果一个类型或该类型的一部分实现了 Drop trait，那么Rust不允许它再去实现Copy trait。即防止两次释放内存。



### 一些拥有Copy trait的类型

- 任何简单标量(整数、浮点数、布尔、char)，可是Copy的。
- 任何需要分配内存或者一些资源的，都不是Copy的。
- 元组，如果所有字段都是Copy，则该元组是Copy的类型。



## 3、所有权与函数

- 在语义上，将值传递给函数和把值赋给变量是类似的，将值传递给函数将发生移动或复制。

  ```rust
  fn move_to_me(str:String){
      println!("当前字符串被移动了：{}",str);
  }
  
  fn test5(){
      let x = String::from("value");
      move_to_me(x);
      // println!("{}",x);// x已经失效了
  }
  ```

- 函数在返回值的过程中也会发生所有权的转移。

- 一个变量的所有权遵循以下模式：

  - 一个值被赋值给其他变量时，就会发生移动。
  - 一个包含heap数据的变量离开作用域后，变量自己会使用drop函数清理heap上的数据，除非变量的所有权移动到另一个变量上了。

- 将String变量作为实参传给函数，函数返回值再带上该实参，即可实现变量所有权不消失（即所有权最终回到调用函数的作用域）。Rust提供引用来方便实现该操作。







# 引用和借用

## 不可变引用

```rust
fn get_len(str:&String) -> usize{
    str.len()
}
fn test1(){
    let s1 = String::from("value");
    let len = get_len(&s1);
    println!("所有权未移动:{},{}",s1,len);
}
```

形参的类型是&引用，实参的类型也是&引用。C++中，实参只需为变量，编译器会根据形参来自动判断使用拷贝构造还是引用。

```
// str.push_str("string");// 默认是不可变的
```

## 可变引用

```rust
fn get_len1(str: &mut String) -> usize{
    str.push_str("string");
    str.len()
}

fn test2(){
    let mut s = String::from("hello");
    let len = get_len1(&mut s);
    println!("可变引用:{},{}",s,len);
}
```

- 可变引用的特殊情况：一个作用域内，一个对象只能有一个引用进行操作。

```rust
    let mut x = String::from("value");
    let x1 = &mut x;
    let x2 = &mut x;
    // println!("{},{}",x1,x2);
```

声明多个引用，会报warning错误，显示不安全的变量。如果代码中出现使用引用，则会编译失败。

- 以下三种行为会发生数据竞争：
  - 两个或多个指针同时访问一个数据
  - 至少有一个指针用于写入数据
  - 没有使用任何机制来同步对数据的访问





## 借用

- 借用：把引用作为函数参数





## 特殊情况

- 可以使用{}来创建多个可变引用。

  ```rust
      let mut x = String::from("value");
  
      {
          let y = &mut x;
          y.push_str("string");
      }
      println!("{}",x);
  
      {
          let yy = &mut x;
          yy.push_str("string");
      }
      println!("{}",x);
  ```

- 不可以同时拥有一个可变引用和一个不可变引用，如果出现此种情况，代码中如果使用则编译失败。

  ```rust
      let mut x = String::from("value");
      let y = &x;
      let yy = &mut x;
      yy.push_str("string");
  
      // println!("{},{}",y,yy); // 出现可变和不可变后不能使用
  ```

- 多个不可变的引用可以

- 悬空指针

  - 一个指针引用了内存中的某个地址，而这块内存已经释放并分配给其他人使用。
  - 在Rust里，编译器可保证引用永远不会悬空引用。

  ```rust
  fn return_ref() -> &String{ 
      let x = String::from("value");
      &x
  }
  ```

  ![image-20241118212738322](D:\Code\RustCode\RustBegining\day04\assets\image-20241118212738322.png)



## 引用的规则

- 只满足下面的一个条件：
  - 一个可变的引用
  - 任意数量不可变的引用
- 引用必须一直有效





# 切片

- Rust的另外一种不持有所有权的数据类型：切片

- 例题：函数接收一个字符串，返回该单词的第一个单词，如果没有任何空格，就将整个字符串返回。

  ```rust
  fn get_first_world_index(str : &String) -> usize {
      
      let bytes = str.as_bytes();
      println!("len={},bytes={}",str.len(),bytes.len());
      for (i , &item) in bytes.iter().enumerate(){
          if item == b' '{
              return i;
          }
      }
      return str.len()
  }
  ```

  扩展：String类的成员函数

  - len()方法返回的是字节长度。
  - bytes返回的是String的Bytes迭代器，适合处理单个字符的情况。
  - as_bytes返回的是字节切片，速度更快，适合处理单个字节的情况。
  - Bytes迭代器调用enumerate方法，返回元组：( 索引值 , 字节值 )

  

  上述存在的问题：函数返回的数值后，一旦对字符串进行数值改变的操作，返回的值将失效。Rust提供字符串切片解决此问题。



## 字符串切片

- 字符串切片是指向字符串中一部分内容的引用

- 形式：[开始索引..结束索引)

  ```rust
      let x = "01234 6789";
      let pre = &x[0..5];
      let last = &x[6..10];
  ```

  小技巧：

  - [ start.. end ]，start和end如果是两端端点，可以省略。。
  
- 解决上述问题：

  ```rust
  	let s = String::from("0123 567");
      let sli = get_plus(&s);
  
      // s.push_str("string");// 前面出现了不可变引用，
      // fn push_str(&mut self)
      println!("切片={}",sli);// sli和字符串绑定了
  ```

  ![image-20241119004532409](D:\Code\RustCode\RustBegining\day04\assets\image-20241119004532409.png)

​	String很多函数的第一个参数是可变引用的自身，所以，当涉及使用这些函数时，你就不能编写需要和自身绑定的变量。



## 字符串字面值

- 字符串字面值被直接存储在二进制程序中

- 字符串切片就是一个含有指针和长度的结构体。

  原先想简单了，所以即使指向同一个字符串的字符串切片，其对象的内存地址也不同。

- &str是不可变应用

- 开发技巧：一般使用&str作为参数类型，既可以接受&str也可以接受String类型的参数。

  - 使用字符串切片，直接作为参数。

  - 使用String，可以创建一个完整的String切片来作为参数。

  - 功能不变，且接受参数范围更广。

    ```rust
    fn test4(){
        let x = String::from("value");
        println!("len={}",get_len(&x[..]));
    
        let x = "12314";
        println!("len={}",get_len(&x[..]));
    }
    
    fn get_len(s : &str) -> usize{
        s.len()
    }
    
    ```



## 其他类型的切片

```rust
    let x = [0,1,2,3,4,5];
    let sli = &x[..4];
    for i in sli {
        print!("{},",i);
    }
    println!();
```





# 总结

### 感觉Rust的特性，C++都可以实现，但是为了实现这些特性的代价较大，需要手动实现这些功能。
