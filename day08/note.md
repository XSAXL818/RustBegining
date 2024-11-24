

# 标题：常用的集合



# 1、Vector

- Vec\<T>，即vector(C++中)
  - 由标准库提供
  - 数据存储在heap上
  - 可存储多个值
  - 只能存储同类型的数据
  - 值在内存中连续存放



## 创建vector

- Vec::new 函数

  ```rust
  let vec:Vec<i32> = Vec::new();// 通常指明类型
  ```

- 使用初始值创建Vec\<T>，使用vec！宏

  ```rust
  let v = vec![1,2,3]; // 不用显示指明类型
  ```



## 添加元素

- 向vector中添加元素：push 方法

  ```rust
      let mut v = Vec::new();
      v.push("hello");// 此时编译器可以推断出v的类型
  ```



## vector 内存释放

- 当vector离开作用域后，其就会释放内存
- 避免有元素引用已经释放内存的vector



## 获取元素

- 两种方式可以引用vector的值

  - 索引
  - get

  ```rust
      let v = vec![0,1,2,3,4,5,6];
      let index = 0;
      println!("index=0,elem={}",v[index]);
  
      let r = &v[2];
  
  
      match v.get(index) {
          Some(r) => println!("index=0,elem={}",r),
          None => println!("index=0,no elem")
      }
  
  // get 方法返回的是 Option<T>枚举
  pub fn get<I>(&self, index: I) -> Option<&I::Output>
      where
          I: SliceIndex<Self>,
      {
          index.get(self)
      }
  ```



## 所有权和借用规则

- 不能在同一作用域内同时拥有可变和不可变引用

  ```rust
      let mut v = vec![1,2,3,4,5];
      let elem1 = &v[0];// 不可变引用
  
      // v.push(1);// 可变引用
  
      println!("elem1={}",elem1);
  
  // push函数
  pub fn push(&mut self, value: T);
  ```

  注意：获取引用后，vector可能会改变，导致引用出错



## 遍历数据

- for循环

  ```rust
      let v = vec![1,2,3,6,6];
      // for i in v.iter() {
      //     print!("{} ",i);
      // }
      for i in &v {
          print!("{} ",i);
      }
      println!();
  
      let mut v = vec![1,2,3,6,6];
      for i in &mut v {
          *i += 10;
      }
      for i in &v {
          print!("{} ",i);
      }
      println!();
  
  // 直接使用 < i in 容器 > 即可，或者容器的迭代器
  ```



## 使用enum来存储多种数据类型

- enum的变体可以附加不同的数据

- enum的变体定义在同一个enum类型下

- 只知道有限的类型时可以使用enum，当类型存在不明确的情况下使用trait，后续再学

  ```rust
  #[derive(Debug)]
  enum Box {
      Int(i32),
      Float(f64),
      Str(String)
  }
  
  fn test5() {
      let v = vec![
          Box::Int(3),
          Box::Float(3.14),
          Box::Str(String::from("3.1415926")),
      ];
      for i in &v {
          match i {
              Box::Int(data) => println!("{}",data),
              Box::Float(data) => println!("{}",data),
              Box::Str(data) => println!("{}",data),
          }
      }
  }
  ```





# 2、String

## Rust开发者经常被字符串困扰的原因

- Rust倾向于暴露可能的错误
- 字符串数据结构复杂
- Rust使用UTF-8



## 字符串是什么

- Byte的集合
- 一些方法：将byte解析为文本
- Rust的核心语言层面，只有一个字符串类型：字符串切片 str 或 &str
- 字符串切片：对存储在其他地方、UTF-8编码的字符串的引用
  - 字符串字面值：存放在二进制文件中，也是字符串切片
- String类型
  - 来自标准库而不是核心语言
  - 可增长、可修改、可拥有



## 通常说的字符串

- String和&str：在标准库中频繁使用、且使用UTF-8编码



## 其他类型的字符串

- Rust标准库提供：OsString、OsStr、CString、CStr等
  - 以String结尾，即有所有权的类型
  - 以Str：拥有或借用的变体
  - 可存储不同编码的文本或在内存中以不同的形式展现
- Library crate 针对存储字符串可提供更多的选项



## 创建字符串（String）

- 由于字符串的特性，很多Vec\<T>的操作可用于String

- String::new 函数

  ```rust
  let s1 = String::new();
  ```

- 使用初始值来创建String

  - to_string()方法（不是String的函数），可用于实现Display trait 的类型，包括字符串字面值

    ```rust
        let s1 = "str ";
        let s1 = s1.to_string();
        let s1 = "Str".to_string();
    ```

  - String::from 函数，从字面值创建String

    ```rust
    let s1 = String::from("value");
    ```



## 修改String

- push_str()方法：吧一个字符串切片附加到String

  ```rust
      let mut s = String::from("hello");
      s.push_str(" ROBOT🤖");
      println!("{}",s);
  
      let s1 = String::from("value");
      s.push_str(&s1);
      println!("{}",s1);
  
  // 不会获取参数的所有权
  pub fn push_str(&mut self, string: &str);
  ```

- push()方法：将单个字符附加到String

  ```rust
      let ch = 'a';
      s.push(ch);
      println!("{}",s);
  ```

- +：连接字符串

  ```rust
      let s1 = String::from("hello");
      let s2 = String::from("value");
      let s3 = s1 + " " + &s2;
      println!("s3={}",s3);
  	// s1不可再使用了
      // println!("{},{},{}",s2,s3,s1);
  ```

  - s1 + " " 的+运算符

    ```rust
    impl Add<&str> for String {
        type Output = String;
    
        #[inline]
        fn add(mut self, other: &str) -> String {
            self.push_str(other);
            self
        }
    }
    ```

    - 这个是具体实现的，不是泛型
    - 第一个参数是借用，第二个参数是应用

  - " "+s2的+运算符

    ```rust
    pub trait Add<Rhs = Self> {
        /// The resulting type after applying the `+` operator.
        #[stable(feature = "rust1", since = "1.0.0")]
        type Output;
    
        /// Performs the `+` operation.
        ///
        /// # Example
        ///
        /// ```
        /// assert_eq!(12 + 1, 13);
        /// ```
        #[must_use = "this returns the result of the operation, without modifying the original"]
        #[rustc_diagnostic_item = "add"]
        #[stable(feature = "rust1", since = "1.0.0")]
        fn add(self, rhs: Rhs) -> Self::Output;
    }
    ```

    - 实现+运算符的strait
    - 第一个是借用，第二个是引用
    - 所以第一个参数在使用 + 运算符后，所有权转移给了接受的变量

- format!:连接多个字符串

  ```rust
  // + 运算符，除第一个，后续String需要&引用
  	let s3 = s1 + "-" + &s2 + "-" + &s3;
      println!("s3={}",s3);
  
  //  不会获取参数的所有权
  	let s1 = String::from("1");
      let s2 = String::from("2");
      let s3 = String::from("3");
      println!("{}",format!("{}-{}-{}",s1,s2,s3));
      println!("{},{},{}",s1,s2,s3);
  ```



## 对String按索引的形式进行访问

- 按索引语法访问String的某部分，会报错

  ```rust
      let s1 = String::from("value");
      let ch = s1[0];
      println!("{}",ch);
  // the type `str` cannot be indexed by `{integer}`
  ```

- Rust的字符串不支持索引语法访问



## 内部表示

- String是对Vec\<u8>的包装

  - 成员函数：len()，返回字符串的字节数

    ```rust
        let s1 = "01234😶‍🌫️";
        println!("bytes={}",s1.len());
    // 19
    ```

    因为UTF-8编码方式，除ASCII外，一个字符占用的字节数多余1字节，所以单纯通过索引取出的字节没有实际意义。



## 字节、标量值、字形簇

- Bytes、Scalar Values、Grapheme Clusters

- <a id="target"> 遍历字符串</a>

- Rust有三种看待字符串的方式

  - 字节

  - 标量值

  - 字形簇（最接近人为理解的“字母”或单个字符）

    ```rust
    // 第一个表情是face in cloud，是一种组合字符，很多编辑器都不能适当处理，比如typora
    	let s1 = "😶‍🌫️😊🤖🎮🥇🌵🍵";
        // bytes
    	for b in s1.bytes() {
            print!("{} ",b);
        }
        println!();
    	// chars
        for ch in s1.chars() {
            print!("{} ",ch);
        }
        println!();
    	// 字形簇，Rust未提供。
    
    ```

- Rust不允许对String进行索引的最后一个原因：

  - 索引操作应消耗一个常量时间(O(1))
  - String无法保证：需要遍历所有内容，来确定有多少个合法的字符





## 切割String

- 可以使用[]和一个范围来创建字符串的切片

  - 切割的边界必须是字符的边界(单个字符的字节不一定是一个字节)

  - 如果切割不在边界上，则程序会运行错误，panic

    ```rust
        let str = "😊🥰🤣❤️😍";
        for ch in str.chars() {
            print!("{}",ch);
        }
        println!();
    
        // 正好第一个表情
        let spl = &str[0..4];
        // 取得的字节在第一个表情内，报错
        // let spl = &str[0..3];
        println!("切片={}",spl);   
    ```



## 遍历String的方法

- [对于字节：bytes方法](#target) 
- [对于标量值：chars方法](#target)
- 字形簇：标准库未提供





## String不简单

- Rust选择将正确处理String数据作为所有Rust程序的默认行为
  - 程序员必须在处理UTF-8数据之前投入更多精力
- 好处是可防止在开发后期处理涉及非ASCII字符的错误





# 3、HashMap

## HashMap<K,V>

- 键值对的形式存储数据，一个键K对应一个值V
- Hash函数，决定了如何在内存中存储K和V
- 使用场景：通过K（任何类型）来寻找数据，而不是通过索引



## 创建HashMap

- HashMap::new()函数，创建一个空的HashMap

  ```rust
  	// 创建空的时，需要显示书写类型
  	let mut scores:HashMap<String,u8> = HashMap::new();
  	// 或者让编译器推断
  	let mut scores = HashMap::new();
      scores.insert(String::from("数学"), 95);
  ```

- HashMap用的较少，不在prelude预导入中

- 标准库对其支持较少，没有内置的宏来创建HashMap

- 数据存在heap堆中

- 同构的，即一个HashMap，K和V的值的类型不可变



## collect方法创建HashMap

- 将两个长度相同的Vector，转变为ZIP，然后使用collect方法，可以组建一个HashMap

  - 第一个Vector的元素作为K，第二个作为V
  - collect方法可以将数据整合成多种集合类型，包括HashMap，需要显示指明返回值的类型

  ```rust
  	let course = vec!["数学","英语"];
      let scores  = vec![95,73];
      // 让编译器来推导
      let cs:HashMap<_,_> = course.iter().zip(scores.iter()).collect();
  ```





## HashMap和所有权

- 对于实现Copy trait 的类型，值会被复制到HashMap中

- 对于拥有所有权的值（如String），值会被移动，所有权会被转移给HashMap

  ```rust
      let key = String::from("key");
      let value = String::from("value");
  
      let mut map = HashMap::new();
      map.insert(key, value);
  
      // println!("{},{}",key,value);
  ```

- 如果将值的引用插入到HashMap中，值本身不会移动

  ```rust
  	let key = String::from("key");
      let value = String::from("value");
  
      let mut map = HashMap::new();
      map.insert(&key, &value);
      println!("{},{}",key,value);
  ```

  - 在HashMap有效的期间，被引用的值必须保持有效

    ```rust
    	let mut map = HashMap::new();
        {
            let key = String::from("key");
            let value = String::from("value"); 
            map.insert(&key, &value);
        }
    	// 报错
        // for (k,v) in &map {
        //     println!("{},{}",k,v);
        // }
    ```



## 访问HashMap的值

- get方法

  - 参数：K
  - 返回：Option<&V>

  ```rust
  	let mut map = HashMap::new();
      map.insert(String::from("数学"), 95);
      map.insert(String::from("英语"), 74);
  
      let k = String::from("数学");
  
      let ret = map.get(&k);
      match ret {
          Some(v) => println!("{}",v),
          None => println!("None")
      };
  
  // get源码，get传入的参数需时引用，返回是Option<T>
  pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
      where
          K: Borrow<Q>,
          Q: Hash + Eq,
      {
          self.base.get(k)
      }
  ```



## 遍历HashMap

- for循环

  ```rust
  //  (k,v)模式匹配  
  	for (k,v) in &map {
          println!("{}:{}",k,v);
      }
  ```



## 更新HashMap<K,V>

- HashMap大小可变

- K不能重复

- 更新时

  - K已经存在，对应一个V

    - 替换现有的V

      ```rust
          let v1 = vec![String::from("数学"),String::from("英语"),String::from("语文")];
          let v2 = vec![95,74,88];
          let mut map:HashMap<_,_> = v1.iter().zip(v2.iter()).collect();
          for (k,v) in &map {
              println!("{},{}",k,v);
          }
          println!();
      
          let key = String::from("数学");
          let new_score = 96;
          map.insert(&key, &new_score);
      
          for (k,v) in &map {
              println!("{},{}",k,v);
          }
      ```

    - 保留现有的V，忽略新的V

      ```rust
      // Entry 枚举，存在和不存在两个变体
      pub enum Entry<'a, K: 'a, V: 'a> {
          /// An occupied entry.
          Occupied( OccupiedEntry<'a, K, V>),
          /// A vacant entry.
          Vacant( VacantEntry<'a, K, V>),
      }
      // or_insert函数：如果不存在则插入，返回该值的可变引用
      pub fn or_insert(self, default: V) -> &'a mut V {
              match self {
                  Occupied(entry) => entry.into_mut(),
                  Vacant(entry) => entry.insert(default),
              }
          }
      // 
      	let mut map = HashMap::new();
          map.insert(String::from("test"), 99);
      
          let en = map.entry(String::from("test"));
          println!("entry={:?}",en);
          en.or_insert(10);
      
          let en = map.entry(String::from("run"));
          println!("entry={:?}",en);
          let x=  en.or_insert(100);
          *x = 999;
          for (k,v) in &map {
              println!("{},{}",k,v);
          }
      ```

      

    - 合并现有的V和新的V

      ```rust
          let text = "blue red blue green yellow";
      
          let mut map = HashMap::new();
          let default = 0;
          for word in text.split_whitespace() {
              // 返回key=word的那个实体entry，如果不存在默认插入<key,0>。返回值为该entry的V的可变引用
              let cnt = map.entry(word).or_insert(default);
              *cnt += 1;
          }
          for (k,v) in &map {
              println!("{},{}",k,v);
          }
      ```

      

  - K不存在

    - 添加一对(K,V)



## Hash函数

- 默认情况下，HashMap使用加密功能强大的Hash函数，可以抵抗拒绝服务（Dos攻击）
  - 不是可用的最快的算法
  - 提供更好的安全性
- 可以指定不同的hasher来切换到其他hash函数
  - hasher是实现BuildHasher trait的类型