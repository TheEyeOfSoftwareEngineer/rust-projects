## 通用集合类型

### 使用动态数组存储多个值
#### 创建动态数组
```rust
let v: Vec<i32> = Vec::new();
```
> 这段代码显式地增加了一个类型标记。因为我们还没有在 这个动态数组中插入任何值，所以Rust无法自动推导出我们想要存储的元素类型

Rust特意提供了一个用于简化代码的vec!宏。这个宏可以根据我们提供的值来创建一个新的动态数组。创建了一个持有初始值1、2、3的`Vec<i32>`。
```rust
let v = vec![1, 2, 3];
```

#### 更新动态数组
```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

#### 销毁动态数组时也会销毁其中的元素
和其他的struct一样，动态数组一旦离开作用域就会被立即销毁
```rust
{
  let v = vec![1, 2, 3];  
}
```
动态数组中的所有内容都会随着动态数组的销毁而销毁，其持有的整数将被自动清理干净

#### 读取动态数组中的元素
- 索引
- get
```rust
let v = vec![1, 2, 3, 4, 5];
let third: i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
  Some(third) => println!("The third element is {}", third),
  None => println!("There is no third element."),
}
```

首先，我们使用索引值2获得的是第 三个值:动态数组使用数字进行索引，索引值从零开始。其次，使用&与[]会直接返回元素的引用;而接收索引作为参数的get方法则会返回 一个Option<&T>。
```rust
let v = vec![1, 2, 3, 4, 5];
let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```
当我们运行这段代码时，[]方法会因为索引指向了不存在的元素 而导致程序触发panic。假如你希望在尝试越界访问元素时使程序直接崩溃，那么这个方法就再适合不过了。

get方法会在检测到索引越界时简单地返回None，而不是使程序直接崩溃。当偶尔越界访问动态数组中的元素是一个正常行为时，你就应该使用这个方法。
```rust
let mut v = vec![1, 2, 3, 4, 5];
let first = &v[0];
v.push(6);
println!("The first element is: {}", first); // error
```
此处的错误是由动态数 组的工作原理导致的:动态数组中的元素是连续存储的，插入新的元素后也许会没有足够多的空间将所有元素依次相邻地放下，这就需要分配新的内存空间，并将旧的元素移动到新的空间上。在本例中，第一个元素的引用可能会因为插入行为而指向被释放的内存。借用规则可以帮助我们规避这类问题。

#### 遍历动态数组中的值
```rust
let v = vec![100, 32, 57];
for i in &v {
  println!("{}", i);
}
```
我们同样也可以遍历可变的动态数组，获得元素的可变引用，并修改其中的值
```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
  *i += 50;
}
```

#### 使用枚举来存储多个类型的值
```rust
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

let row = vec![
  SpreadsheetCell::Int(3),
  SpreadsheetCell::Text(String::from("blue")),
  SpreadsheetCell::Float(10.12),
];
```

### 使用字符串存储UTF-8编码的文本
字符串本身就是基于字节的集合，并通过功能性的方法将字节解析为文本

#### 字符串是什么
Rust在语言核心部分只有一种字符串类型，那就是字符串切片str，它通常以借用的形式 (&str)出现。

String类型被定义在了Rust标准库中而没有被内置在语言的核心部分。当Rust开发者们提到“字符串”时，他们通常指的是String与 字符串切片&str这两种类型，而不仅仅只是其中的一种

#### 创建一个新的字符串
```rust
let mut s = String::new();
```
创建了一个叫作s的空字符串，之后我们可以将数据填入 该字符串。但是一般而言，字符串在创建的时候都会有一些初始数据。对于这种情况，我们可以对那些实现了Display trait的类型调用to_string方法，如同字符串字面量一样
```rust
let data = "initial contents";
let s = data.to_string();
let s = "initial contents".to_string();
```
我们同样也可以使用函数String::from来基于字符串字面量生成String。
```rust
let s = String::from("initial contents");
```

#### 更新字符串
String的大小可以增减，其中的内容也可以修改，正如我们将数 据推入其中时`Vec<T>`内部数据所发生的变化一样。此外，我们还可以方便地使用+运算符或format! 宏来拼接String。

使用`push_str`或`push`向字符串中添加内容
```rust
let mut s = String::from("foo");
s.push_str("bar");
```
s中的字符串会被更新为foobar。由于我们 并不需要取得参数的所有权，所以这里的push_str方法只需要接收一个字符串切片作为参数
```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);
```
假如push_str方法取得了s2的所有权，那么我们就无法在最后一行打印出它的值了

push方法接收单个字符作为参数，并将它添加到String中
```rust
let mut s = String::from("lo");
s.push('l'); // lol
```
使用+运算符或format! 宏来拼接字符串
```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1 被当做字符串字面量，而s2被当做字符串切片
```
> 注意这里s1已经被移动且再也不能被使用了
值得注意的是，我们在加法操作中仅对s2采用了引用，而s1在加法操作 之后则不再有效。产生这一现象的原因与使用+运算符时所调用的方法 签名有关。这里的+运算符会调用一个add方法，它的签名看起来像下面一样:
```rust
fn add(self, s: &str) -> String {
    ...
}
```
这里的self是String，而s是&str，所以我们可以将s转换为String，然后调用add方法。我们能够使用&s2来调用add函数的原因在于:编译器可以自动将 &String类型的参数强制 转换为&str类型。当我们调用add函数时， Rust使用了一种被称作解引用强制转换的技术，将&s2转换为了`&s2[..]`。
其次，我们可以看到add函数签名中的self并没有&标记，所以add函数会取得self的所有权。
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = s1 + "-" + &s2 + "-" + &s3;

let s = format!("{}-{}-{}", s1, s2, s3); // 代码要更加易读，并且不会夺取任何参数的所有权
```

#### 字符串索引
```rust
let s1 = String::from("hello");
let h = s1[0];
```
Rust中的字符串并不支持索引
- 内部布局
String实际上是一个基于`Vec<u8>`的封装类型
```rust
let len = String:from("Hola").len();
```
len方法将会返回4，这意味着动态数组所存储的 字符串Hola占用了4字节。在编码为UTF-8时，每个字符都分别占用1字节
Rust中提供了不同的方式来解析存储在计算机中的字符串数据，以便于程序员们自行选择所需的解释方式，而不用关心具体的语言类型。
Rust不允许我们通过索引来获得String中的字符还有最后一个原因，那就是索引操作的复杂度往往会被预期为常数时间,但是在String中无法保障这个做法的性能；因为Rust必须要遍历从 头至索引位置的整个内容来确定究竟有多少合法的字符存在

#### 字符串切片
```rust
let hello = String::from("Hello, world!");
let s = &hello[0..5];
```

#### 遍历字符串的方法
```rust
for c in "नमस्ते".chars() {
  println!("{}", c); // prints न, म, स, त, ए
}
```
而bytes方法则会依次返回每个原始字节，这在某些场景下可能会有用:
```rust
for b in "नमस्ते".bytes() {
  println!("{}", b); // prints 228, 224, 226, 232, 224
}
```

### 在哈希映射中存储键值对
`HashMap<K, V>`
它存储了从K类型键到V类型值之间的映射关系。哈希映射在内部 实现中使用了哈希函数 ，这同时决定了它在内存中存储键值对的方 式。许多编程语言都支持这种类型的数据结构，只是使用了不同的名 字，例如:哈希(hash)、映射(map)、对象(object)、哈希表 (hash table)、字典(dictionary)或关联数组(associative array)等，这只是其中的一部分而已

#### 创建一个新的哈希映射
可以使用new来创建一个空哈希映射，并通过insert方法来添加元素
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```
> 我们首先需要使用use将HashMap从标准库的集合部分引入当前作用域
和动态数组一样，哈希映射也将其数据存储在堆上;依然和动态数组一样，哈希映射也是同质的:它要求所有的键必须拥有相同的类型，所有的值也必须拥有相同的类型。

另外一个构建哈希映射的方法是，在一个由键值对组成的元组动态数组上使用collect方法。这里的collect方法可以将数据收集到很多数据结构中，这些数据结构也包HashMap。
```rust
use std::collections::HashMap;
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```
这里的类型标记HashMap<_, _>不能被省略，因为collect可以作用于许多不同的数据结构，如果不指明类型的话，Rust就无法知道我们具体想要的类型。但是对于键值的类型参数，我们则使用了下画线占位，因为Rust能够根据动态数组中的数据类型来推导出哈希映射所包含的类型。

#### 哈希映射与所有权
对于那些实现了Copy trait的类型，例如i32，它们的值会被简单地复制到哈希映射中。而对于String这种持有所有权的值，其值将会转移且所有权会转移给哈希映射
```rust
use std::collections::HashMap;
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");
let mut map = HashMap::new();
// 一旦键值对被插入，其所有权就会转移给哈希映射
map.insert(field_name, field_value);
```
在调用insert方法后，field_name和field_value变量被移动到哈希映射中，我们再也没有办法使用这两个变量了。

假如我们只是将值的引用插入哈希映射，那么这些值是不会被移动到哈希映射中的。这些引用所指向的值必须要保证，在哈希映射有效时自己也是有效的

#### 访问哈希映射中的值
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
score.insert(String::from("Blue"), 10);
score.insert(String::from("Yellow"), 50);
let team_name = String::from("Blue");
let score = scores.get(&team_name);
```
上面这段代码中的score将会是与蓝队相关联的值，也就是 Some(&10)。因为get返回的是一个Option<&V>，所以这里的结果被封 装到了Some中;假如这个哈希映射中没有键所对应的值，那么get就会返回None。

类似于动态数组，我们同样可以使用一个for循环来遍历哈希映射中所有的键值对:
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

#### 更新哈希映射
- 覆盖旧值
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);
println!("{:?}", scores);
```
- 只有键没有对应值时插入数据
在实际工作中，我们常常需要检测一个键是否存在对应值，如果 不存在，则为它插入一个值。哈希映射中提供了一个被称为entry的专 用API来处理这种情形，它接收我们想要检测的键作为参数，并返回一个叫作Entry的枚举作为结果。这个枚举指明了键所对应的值是否存在
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);
println!("{:?}", scores);
```
Entry的or_insert方法被定义为返回一个Entry键所指向值的可变引用，假如这个值不存在，就将参数作为新值插入哈希映射中，并把这个新值的可变引用返回。使用这个功能要比我们自己编写逻辑代码更加简单，使代码更加整洁，另外也可以与借用检查器结合得更好
- 基于旧值来更新值
哈希映射的另外一个常见用法是查找某个键所对应的值，并基于这个值来进行更新
```rust
use std::collections::HashMap;
let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
println!("{:?}", map);
```
代码中的方法or_insert实际上为我们传入的键返回了一个指向关联值的可变引用(&mut V)。这个可变引用进而被存储到变量count 上，为了对这个值进行赋值操作，我们必须首先使用星号(*)来对 count进行解引用。由于这个可变引用会在for循环的结尾处离开作用域，所以我们在代码中的所有修改都是安全且满足借用规则的

#### 哈希函数
为了提供抵御拒绝服务攻击(DoS，Denial of Service)的能力，HashMap默认使用了一个在密码学上安全的哈希函数。这确实不是最快的哈希算法，不过为了更高的安全性付出一些性能代价通常是值得的。假如你在对代码进行性能分析的过程中，发现默认哈希函数成为了你的性能热点并导致性能受损，你也可以通过指定不同的哈希计算工具 来使用其他函数。这里的哈希计算工具特指实现了BuildHasher trait的类型








