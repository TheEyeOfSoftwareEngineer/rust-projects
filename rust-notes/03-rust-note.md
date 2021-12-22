## 结构体
结构，或者说结构体，是一种自定义数据类型，它允许我们命名 多个相关的值并将它们组成一个有机的结合体。

### 定义并实例化结构体
结构体和元素有些相似。和元组一样，结构 体中的数据可以拥有不同的类型。而和元组不一样的是，结构体需要 给每个数据赋予名字以便清楚地表明它们的意义。正是由于有了这些 名字，结构体的使用要比元组更加灵活:你不再需要依赖顺序索引来 指定或访问实例中的值。


关键字struct被用来定义并命名结构体，一个良好的结构体名称 应当能够反映出自身数据组合的意义。除此之外，我们还需要在随后 的花括号中声明所有数据的名字及类型，这些数据也被称作字段。
```rust
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool
}
```

为了使用定义好的结构体，我们需要为每个字段赋予具体的值来 创建结构体实例 。可以通过声明结构体名称，并使用一对花括号包含 键值对来创建实例。其中的键对应字段的名字，而值则对应我们想要 在这些字段中存储的数据。这里的赋值顺序并不需要严格对应我们在 结构体中声明它们的顺序。换句话说，结构体的定义就像类型的通用 模板一样，当我们将具体的数据填入模板时就创建出了新的实例。
```rust
let user1 = User {
  email: String::from("someone@example.com"),
  username: String::from("someusername123"),
  active: true,
  sign_in_count: 1,
}
```
在获得了结构体实例后，我们可以通过点号来访问实例中的特定 字段。
```rust
user1.email = String::from("anotheremail@example.com")
```
> 一旦实例可变，那么实例中的所有字段都将是可变的。Rust不允许我们单独声明某一部分字段的可变性。如同其他表达式一样，我们可以在函数体的最后一个表达式中构造结构体实例，来隐式地将这个实例作为结果返回。
```rust
fn build_user(email: String, username: String) -> User {
  User {
    email: email,
    username: username,
    active: true,
    sign_in_count: 1,
  }
}
```
在函数中使用与结构体字段名相同的参数名可以让代码更加易于 阅读，但分别两次书写email和username作为字段名与变量名则显得有 些烦琐了，特别是当结构体拥有较多字段时。Rust为此提供了一个简便的写法(和JS一样)。
### 在变量名与字段名相同时使用简化版的字段初始值
```rust
fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
```
### 使用结构体更新语法根据其他实例创建新实例
在许多情形下，在新创建的实例中，除了需要修改的小部分字段，其余字段的值与旧实例中的完全相同。我们可以使用结构体更新语法来快速实现此类新实例的创建。
```rust
let user2 = User {
  email: String::from("another@example.com"),
  username: String::from("anotherusername567"),
  ..user1
}
```
通过结构体更新语法，我们可以使用更少的代码来实现完全相同的效果. 这里的双点号..表明剩下的那些还未被显式赋值的字段都与给定实例拥有相同的值。

### 使用不需要对字段命名的元组结构体来创建不同的类型
还可以使用另外一种类似于元组的方式定义结构体，这种结构体也被称作元组结构体。元组结构体同样拥有用于表明自身含义的名称，但你无须在声明它时对其字段进行命名，仅保留字段的类型即可。一般来说，当你想要给元组赋予名字，并使其区别于其他拥有同样定义的元组时，你就可以使用元组结构体。在这种情况下，像常规结构体那样为每个字段命名反而显得有些烦琐和形式化了。

定义元组结构体时依然使用struct关键字开头，并由结构体名称及元组中的类型定义组成。
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```
> black和origin是不同的类型，因为它们两个分别是不同元组结构体的实例。定义的每一个结构体都拥有自己的类型，即便结构体中的字段拥有完全相同的类型。

### 没有任何字段的空结构体
Rust允许我们创建没有任何字段的结构 体!因为这种结构体与空元组()十分相似，所以它们也被称为空结构 体。当你想要在某些类型上实现一个trait，却不需要在该类型中存储 任何数据时，空结构体就可以发挥相应的作用。

### 结构体数据的所有权
我们使用了自持所有权的 String类型而不是&str字符串切片类型。这是一个有意为之的选 择，因为我们希望这个结构体的实例拥有自身全部数据的所有权。 在这种情形下，只要结构体是有效的，那么它携带的全部数据也就 是有效的。

我们也可以在结构体中存储指向其他数据的引用，不过 这需要用到Rust中独有的生命周期功能。生命周期保证了结构体实例中引用数据的有效期不短于 实例本身。

### 通过派生trait增加实用功能
Rust提供了许多可以通过derive注解来派生的trait，它 们可以为自定义的类型增加许多有用的功能。

### 方法
方法与函数十分相似:它们都使用fn关键字及一个名称来进行声 明;它们都可以拥有参数和返回值;另外，它们都包含了一段在调用时执行的代码。但是，方法与函数依然是两个不同的概念，因为方法总是被定义在某个结构体(或者枚举类型、trait对象)的上下文中，并且它们的第一个参数永远都是self，用于指代调用该方法的结构体实例。
#### 定义方法
```rust
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}
```
选择`&self`签名的原因和之前选择使用`&Rectangle`的原因差不多:**我们既不用获得数据的所有权也不需要写入数据，而只需要读取数据即可**。假如我们想要在调用方法时改变实例的某些数据，那么就需要将第一个参数改写为`&mut self`。通常来说，将第一个参数标记为self并在调用过程中取得实例的所有权的方法并不常见。这种技术有可能会被用于那些需要将self转换为其他类型，且在转换后想要阻止调用者访问原始实例的场景。

使用方法替代函数不仅能够避免在每个方法的签名中重复编写self的类型，还有助于我们组织代码的结构。我们可以将某个类型的实例需要的功能放置在同一个impl块中，从而避免用户在代码库中盲目地自行搜索它们。

自动引用和解引用的功能。方法调用是Rust中少数几个拥有这种行为的地方之一。第一种调用看上去要简捷得多。这种自动引用行为之所以能够行得通，是因为方法有一个明确的作用对象:self的类型。在给出调用者和方法名的前提下，Rust可以准确地推导出方法是否是只读的(&self)，是否需要修改数据(&mut self)，是否会获取数据的所有权(self)。这种针对方法调用者的隐式借用在实践中可以让所有权系统更加友好且易于使用。

### 带有更多参数的方法
```rust
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}
```

### 关联函数
除了方法，impl块还允许我们定义不用接收self作为参数的函数。由于这类函数与结构体相互关联，所以它们也被称为关联函数(associated function)。我们将其命名为函数而不是方法，是因为它们不会作用于某个具体的结构体实例。String::from就是关联函数的一种。关联函数常常被用作构造器来返回一个结构体的新实例。

### 多个impl块
每个结构体可以拥有多个impl块
```rust
impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }  
  fn square(size: u32) -> Rectangle {
    Rectangle { width: size, height: size }
  }
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}
```
