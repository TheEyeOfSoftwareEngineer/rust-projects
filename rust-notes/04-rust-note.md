## 枚举与模式匹配
枚举类型，通常也被简称为枚举，它允许我们列举所有可能的值来定义一个类型。

### 定义枚举
我们可以通过定义枚举IpAddrKind来表达这样的概念，声明该枚 举需要列举出所有可能的IP地址种类—V4和V6，这也就是所谓的枚举 变体(variant):
```rust
enum IpAddrKind {
  V4,
  V6
}
```

### 枚举值
```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```
> 枚举的变体全都位于其标识符的命名空间中，并 使用两个冒号来将标识符和变体分隔开来。
由于IpAddrKind::V4和 IpAddrKind::V6拥有相同的类型IpAddrKind，所以我们可以定义一个 接收IpAddrKind类型参数的函数来统一处理它们:
```rust
fn route(ip_type: IpAddrKind) {} 
```
我们只能知道IP地址的种类，却还没有办法去存 储实际的IP地址数据。
```rust
enum IpAddrKind {
  V4,
  V6
}

struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

let home = IpAddr {
  kind: IpAddrKind,
  address: String::from("127.0.0.1"),
}

let loopback = IpAddr {
  kind: IpAddrKind::V6,
  address: String::from("::1),
}
```
枚举允许我们直接将其关联的数据嵌入枚举变体内。我 们可以使用枚举来更简捷地表达出上述概念，而不用将枚举集成至结 构体中。在新的IpAddr枚举定义中，V4和V6两个变体都被关联上了一 个String值:
```rust
enum IpAddr {
  V4(String),
  V6(String),
}
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```
我们直接将数据附加到了枚举的每个变体中，这样便不需要额外地使用结构体。

另外一个使用枚举代替结构体的优势在于:每个变体可以拥有不 同类型和数量的关联数据。还是以IP地址为例，IPv4地址总是由4个0 ~255之间的整数部分组成。假如我们希望使用4个u8值来代表V4地址，并依然使用String值来代表V6地址，那么结构体就无法轻易实现这一目的了，而枚举则可以轻松地处理此类情形:
```rust
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```
实际上，由于存储和编码IP地址的工作实在太常 见了，因此标准库为我们内置了一套可以开箱即用的定义
```rust
struct Ipv4Addr {
  // ...
}

struct Ipv6Addr {
  // ...
}

enum IpAddr {
  V4(Ipv4Addr),
  V6(Ipv6Addr),
}
```
你可以在枚举的变体中嵌入任意类型的数据，无论是字符串、数值，还是结构体，甚至可以嵌入另外一个枚举
> 虽然标准库中包含了一份IpAddr的定义，但由于 我们没有把它引入当前的作用域，所以可以无冲突地继续创建和使用自己定义的版本。
```rust
enum Message {
  Quit,
  Move {x: i32, y: i32},
  Write(String),
  ChangeColor(i32, i32, i32),
}
```
- Quit没有任何关联数据
- Move包含了一个匿名结构体
- Write包含了一个String
- ChangeColor包含了3个i32值
枚举有些类似于定义多个不同类型的结构体。但 枚举除了不会使用struct关键字，还将变体们组合到了同一个Message类型中。下面代码中的结构体可以存储与这些变体完全一样的数据:
```rust
struct QuitMessage; // 空结构体 
struct MoveMessage {
  x: i32,
  y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体
```
两种实现方式之间的差别在于，假如我们使用了不同的结构体， 那么每个结构体都会拥有自己的类型，我们无法轻易定义一个能够统一处理这些类型数据的函数，而我们定义的Message枚举则不同，因为它是单独的一个类型。


枚举和结构体还有一点相似的地方在于:正如我们可以使用impl关键字定义结构体的方法一样，我们同样可以定义枚举的方法
```rust
impl Message {
  fn call(&self) {
    // 
  }
}

let m = Message::Write(String::from("hello"));
m.call();
```

### Option枚举及其在空值处理方面的优势
Option类型描述了一种值可能不存在的情形，所有被广泛应用。将这一概念使用类型系统描述出来意味着，编译器可 以自动检查我们是否妥善地处理了所有应该被处理的情况。使用这一 功能可以避免某些在其他语言中极其常见的错误。

Rust并没有像许多其他语言一样支持空值。空值 (Null)本身是一个值，但它的含义却是没有值。在设计有空值的语言中，一个变量往往处于这两种状态:空值或非空值。这个枚举就是`Option<T>`，它在标准库中被定义为如下：
```rust
enum Option<T> {
  Some(T),
  None,
}
```
由于`Option<T>`枚举非常常见且很有用，所以它也被包含在了预导入模块中，这意味着我们不需要显式地将它引入作用域。另外，它的 变体也是这样的:我们可以在不加`Option::`前缀的情况下直接使用`Some`或`None`。但`Option<T>`枚举依然只是一个普通的枚举类型，`Some(T)`和`None`也依然只是`Option<T>`类型的变体。
```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```
假如我们使用了None而不是Some变体来进行赋值，那么我们需要明确地告知Rust这个`Option<T>`的具体类型。这是因为单独的None变体值与持有数据的Some变体不一样，编译器无法根据这些信息来正确推导出值的完整类型。

简单来讲，因为`Option<T>`和T(这里的T可以是任意类型)是不同的类型，所以编译器不会允许我们像使用普通值一样去直接使用`Option<T>`的值。
```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y;
```
提示错误：Rust无法理解i8和Option<T> 相加的行为，因为它们拥有不同的类型。当我们在Rust中拥有一个i8 类型的值时，编译器就可以确保我们所持有的值是有效的。我们可以 充满信心地去使用它而无须在使用前进行空值检查。而只有当我们持 有的类型是`Option<i8>`(或者任何可能用到的值)时，我们才必须要考虑值不存在的情况，同时编译器会迫使我们在使用值之前正确地做出处理操作。换句话说，为了使用`Option<T>`中可能存在的T，我们必须要将它转换为T。一般而言，这能帮助我们避免使用空值时最常见的一个问题:假设某个值存在，实际上却为空。

无论在什么地方，只要一个值的类型不是Option<T>的，我们就可以安全地假设这个值不是非空的。这是Rust为了限制空值泛滥以增加Rust代码安全性而做出的一个有意为之的设计决策

为了使用一个`Option<T>`值，你必须要编写处理每个变 体的代码。某些代码只会在持有Some(T)值时运行，它们可以使用变体中存储的T。而另外一些代码则只会在持有None值时运行，这些代码将 没有可用的T值。match表达式就是这么一个可以用来处理枚举的控制 流结构:它允许我们基于枚举拥有的变体来决定运行的代码分支，并 允许代码通过匹配值来获取变体内的数据。

### 控制流运算符match
它允许将一个值与一系列的模式相比较，并根据匹配的模式执行相应代码。模式可由字面量、变量名、通配符和许多其他东西组成
```rust
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}
```
如果分支代码足够短，就像仅返回一个值的话，那么通 常不需要使用花括号。但是，假如我们想要在一个匹配分支中包含多行代码，那么就可以使用花括号将它们包裹起来。
```rust
fn value_in_cents(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => {
      println!("Lucky penny!");
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}
```

### 绑定值的模式
匹配分支另外一个有趣的地方在于它们可以绑定被匹配对象的部分值，而这也正是我们用于从枚举变体中提取值的方法。
```rust
#[derive[Debug]]
enum UsState {
  Alabama,
  Alaska,
  // ...
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState)
}
```
```rust
fn value_in_cents(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => 1,
    Coin::Nicker => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println1("State quarter from {}:?}!", state);
      25
    }
  }
}
```

### 匹配`Option<T>`
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i+1),
  }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### 匹配必须穷举所有的可能
match表达式中还有另外一个需要注意的特性
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    Some(i) => Some(i + 1),
  }
}
```
此段代码的问题在于我们忘记了处理值是None的情形。幸运的是，这是一个Rust可以轻松捕获的问题

### _通配符
```rust
let some_u8_value = 0u8;
match some_u8_value {
  1 => println!("one"),
  3 => println!("three"),
  5 => println!("five"),
  7 => println!("seven"),
  _ => (),
}
```
这里的_模式可以匹配任何值。通过将它放置于其他分支后，可以 使其帮我们匹配所有没有被显式指定出来的可能的情形。与它对应的 代码块里只有一个()空元组，所以在_匹配下什么都不会发生。使用它 也就暗示了，我们并不关心那些在_通配符前没有显式列出的情形，且 不想为这些情形执行任何操作。

不过，在只关心某一种特定可能的情形下，使用match仍然会显得有些烦琐。为此，Rust提供了if let语句。

### 简单控制流if let
if let能让我们通过一种不那么烦琐的语法结合使用if与let，并处理那些只用关心某一种匹配而忽略其他匹配的情况。
```rust
let some_u8_value = Some(0u8);
match some_u8_value {
  Some(3) => println!("three"),
  _ => (),
}
```
不过，我们可以使用if let以一种更加简短的方式实现这段代码。
```rust
if let Some(3) = some_u8_value {
  println!("three");
}
```
这里的if let语法使用一对以=隔开的模式与表达式。它们所起的作用与match中的完全相同，表达式对应match中的输入，而模式则对应第一个分支。

使用if let意味着你可以编写更少的代码，使用更少的缩进，使用更少的模板代码。但是，你也放弃了match所附带的穷尽性检查。究竟应该使用match还是if let取决于你当时所处的环境，这是一个在代码简捷性与穷尽性检查之间取舍的过程。
```rust
let mut count = 0;
match coin {
  Coin::Quarter(state) => println!("State quarter from {:?}!", state),
  _ => count += 1,
}
```
或者
```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
  println!("State quarter from {:?}!", state);
} else {
  count += 1;
}
```




