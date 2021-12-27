## 泛型、trait和生命周期

所有的编程语言都会致力于高效地处理重复概念，并为此提供各种各样的工具。在Rust中，泛型 (generics)就是这样一种工具。泛型是具体类型或其他属性的抽象替代。

### 通过将代码提取为函数来减少重复工作
```rust
fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let mut largest = number_list[0];
  for number in number_list {
    if number > larger {
        let mut largest = number_list[0];
 = number;
    }
  }
  println!("The largest number is {}", largerst)
}
```
如果需要在两个不同的列表中搜索各自的最大值
```rust
fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let mut largest = number_list[0];
  for number in number_list {
    if number > largest {
      largest = number;
    }
  }
  println!("The largest number is {}", largest);
  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
  let mut largest = number_list[0];
  for number in number_list {
    if number > largest {
      largest = number;
    }
  }
  println!("The largest number is {}", largest);
}
```
为了消除这种重复代码，我们可以通过定义函数来创建抽象，它可以接收任意整数列表作为参数并进行求值
```rust
fn largest(list: &[i32]) -> i32 {
  let mut largest = list[0];
  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest(&number_list);
  println!("The largest number is {}", result);
  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
  let result = largest(&number_list);
  println!("The largest number is {}", result);
}
```

### 泛型数据类型
#### 在函数定义中
当使用泛型来定义一个函数时，我们需要将泛型放置在函数签名中通常用于指定参数和返回值类型的地方
```rust
fn largest<T>(list: &[T]) -> T {
  let mut largest = list[0];
  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest[&number_list];
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];
  let result = largest(&char_list);
  println!("The largest char is {}", result);
}
```

#### 在结构体定义中
```rust
struct Point<T> {
  x: T,
  y: T,
}

fn main() {
  let integer = Point {x: 5, y: 10};
  let float = Point {x: 1.0, y: 4.0};
}
```
为了在保持泛型状态的前提下，让Point结构体中的x和y能够被实例化为不同的类型，我们可以使用多个泛型参数
```rust
struct Point<T, U> {
  x: T,
  y: U,
}

fn main() {
  let both_integer = Point {x:5, y: 10};
  let both_float = Point {x: 1.0, y: 4.0};
  let integer_and_float = Point {x: 5, y: 4.0};
}
```

#### 在枚举定义中
```rust
enum Option<T> {
  Some(T),
  None,
}

enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

#### 在方法定义中
```rust
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

fn main() {
  let p = Point {x: 5, y: 10};
  println!("p.x = {}", p.x());
}
```
> 我们必须紧跟着impl关键字声明T，以便能够在实现方法时指定类型Point<T>。通过在impl之后将T声明为泛型，Rust能够识别出Point尖括号内的类型是泛型而不是具体类型
```rust
impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}
```

```rust
struct Point<T, U> {
  x: T,
  y: 0,
}

impl<T, U> Point<T, U> {
  fn mixup(V, W)(self, other: Point<V, W>) -> Point(T, W) {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

fn main() {
  let p1 = Point {x: 5, y: 10.4};
  let p2 = Point {x: "Hello", y: 'c'};
  let p3 = p1.mixup(p2);
  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

#### 泛型代码的性能问题
单态化：在编译的时候把具体的泛型替换成确定的数据类型

### trait定义共享行为
> trait与其他语言中常被称为接口(interface)的功能类似，但也不尽相同

#### 定义trait
类型的行为由该类型本身可供调用的方法组成。当我们可以在不同的类型上调用相同的方法时，我们就称这些类型共享了相同的行为。trait提供了一种将特定方法签名组合起来的途径，它定义了为达成某种目的所必需的行为集合
```rust
pub trait Summary() {
  fn summrize(&self) -> String
}
```
使用了trait关键字来声明trait，紧随关键字的是该 trait的名字，在本例中也就是Summary。在其后的花括号中，我们声明了用于定义类型行为的方法签名，也就是本例中的`fn summarize(&self) -> String`

一个trait可以包含多个方法:每个方法签名占据单独一行并以分号结尾

#### 为类型实现trait
```rust
pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location);
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content);
  }
}
```
为类型实现trait与实现普通方法的步骤十分类似。它们的区别在于我们必须在impl关键字后提供我们想要实现的trait名，并紧接for关键字及当前的类型名。在impl代码块中，我们同样需要填入trait中的方法签名。但在每个签名的结尾不再使用分号，而是使用花括号并在其中编写函数体来为这个特定类型实现该trait的方法所应具有的行为

```rust
let tweet = Tweet {
  username: String::from("horse_ebooks"),
  content: String::from("of couse, as you probably already know, people"),
  reply: false,
  retweet: false,
}

println!("1 new tweet: {}", tweet.summarize());
```
#### 默认实现
```rust
pub trait Summary {
  fn summarize(&self) -> String {
    String::from("(Read more...)");
  }
}
```

```rust
let article = NewsArticle {
  headline: String::from("Penguins win the Stanley Cup Championship!"),
  location: String::from("Pittsburgh, PA, USA"),
  author: String::from("Iceburgh"),
  content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
};

println!("New article available! {}" , article.summarize());
```
可以在默认实现中调用相同trait中的其他方法，哪怕这些 方法没有默认实现。基于这一规则，trait可以在只需要实现一小部分方法的前提下，提供许多有用的功能
```rust
pub trait Summary {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}

impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}

let tweet = Tweet {
  username: String::from("horse_ebooks"),
  content: String::from("of couse, as you probably already know, people"),
  reply: false,
  retweet: false,
}

println!("1 new tweet: {}", tweet.summarize());
```

#### trait作为参数
可以定义一个notify函数来调用其item参数的 summarize方法，这里的参数item可以是任何实现了Summary trait的类型。为了达到这一目的，我们需要像下面一样使用impl Trait语法:
```rust
pub fn notify(item: impl Summary) {
  println!("Breaking news! {}"m item.summarize());
}
```
- trait约束
`impl Trait`常被用在一些较短的示例中，但它其实只是 trait约束 (trait bound)的一种语法糖。它的完整形式如下所示:
```rust
pub fn notify<T: Summary>(itme: T) {
  println!("Breaking news! {}", item.summarize());
}
```
- 通过+语法来指定多个trait约束
```rust
pub fn notify(item: impl Summary + Display) {
  // ...
}

pub fn notify<T: Summary + Display>(item; T) {
  // ...
}
```
- 使用where从句来简化trait约束
```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
  // ...
}

fn some_function<T, U> -> i32 
  where T: Display + Clone,
        U: Clone + Debug {
  // ...
}
```
#### 返回实现了trait的类型
```rust
fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  }
}
```

#### 使用trait约束来修复largest函数
```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];
  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }
  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];
  let result = largest(&char_list);
  println!("The largest char is {}", result);
}
```

#### 使用trait约束来有条件地实现方法
```rust
use std::fmt::Display;

struct Pari<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self {
      x,
      y,
    }
  }
}

impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}
```

### 使用生命周期保证引用的有效性
#### 使用生命周期来避免悬垂引用
生命周期最主要的目标在于避免悬垂引用，进而避免程序引用到非预期的数据
```rust
{
  let r;
  {
    let x = 5;
    r = &x;
  }
  println!("r: {}", r);
}
```
在外部作用域中声明了一个名为r的未初始化变量，而内部作用域则声明了一个初始值为5的变量x。在内部作用域中，我们尝试将r的值设置为指向x的引用。接着，当内部作用域结束时，尝试去打印出r所指向的值。这段代码将无法通过编译，因为在我们使用r时，它所指向的值已经离开了作用域

#### 借用检查器
Rust编译器拥有一个借用检查器 (borrow checker)，它被用于比较不同的作用域并确定所有借用的合法性

#### 函数中的泛型生命周期
```rust
fn main() {
  let string1 = String::from("abcd");
  let string2 = "xyz";
  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
```

#### 生命周期标注语法
```rust
&i32 // 引用
&'a i32 // 拥有显式生命周期的引用
&'a mut i32 // 拥有显式生命周期的可变引用
```
单个生命周期的标注本身并没有太多意义，标注之所以存在是为了向Rust描述多个泛型生命周期参数之间的关系

#### 函数签名中的生命周期标注
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
```

#### 深入理解生命周期

#### 结构体定义中的生命周期标注
```rust
struct ImportantExcerpt<'a> {
  part: &'a str,
}

fn main() {
  let novel = String::from("Call me I shamael. Some years ago...");
  let first_sentence = novel.split('.')
    .next()
    .expect("Could not find a '.'");
  let i = ImportantExcerpt { part: first_sentence };
}
```

#### 生命周期省略
任何引用都有一个生命周期，并且需要为使用引用的函数或结构体指定生命周期参数
```rust
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..1];
    }
  }
  &s[..]
}
```
函数参数或方法参数中的生命周期被称为输入生命周期，而返回值的生命周期则被称为输出生命周期

在没有显式标注的情况下，编译器目前使用了3种规则来计算引用的生命周期。第一条规则作用于输入生命周期，第二条和第三条规则作用于输出生命周期。当编译器检查完这3条规则后仍有无法计算出生命周期的引用时，编译器就会停止运行并抛出错误。这些规则不但对 fn定义生效，也对impl代码块生效
- 每一个引用参数都会拥有自己的生命周期参数。换句话说，单参数函数拥有一个生命周期参数
- 当只存在一个输入生命周期参数时，这个生命周期会被赋予给所有输出生命周期参数
- 当拥有多个输入生命周期参数，而其中一个是`&self`或`&mut self`时，self的生命周期会被赋予给所有的输出生命周 期参数

#### 方法定义中的生命周期标注
声明和使用生命周期参数的位置取决于它们是与结构体字段相关，还是与方法参数、返回值相关

结构体字段中的生命周期名字总是需要被声明在impl关键字之后，并被用于结构体名称之后，因为这些生命周期是结构体类型的一部分

在impl代码块的方法签名中，引用可能是独立的，也可能会与结构体字段中的引用的生命周期相关联。另外，生命周期省略规则在大部分情况下都可以帮我们免去方法签名中的生命周期标注
```rust
impl<'a> ImportantExcerpt<'a> {
  fn announce_and_return_part(&self, annoucement: &str) -> &str {
    println!("Attention please: {}", annoucement);
    self.part
  }
}
```
这里有两个输入生命周期，所以Rust通过应用第一条生命周期省略规则给了&self和announcement各自的生命周期。接着，由于其中一个参数是&self，返回类型被赋予了&self的生命周期，因此所有的生命周期就都被计算出来了

#### 静态生命周期
Rust中还存在一种特殊的生命周期`'static`，它表示整个程序的执行期。所有的字符串字面量都拥有'static生命周期：
```rust
let s: &'static str = "I have a static lifetime";
```

#### 同时使用泛型参数、trait约束和生命周期
```rust
use std::fmt::Display;

fn longest_with_an_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
  println!("Annoucement! {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
```



