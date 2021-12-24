## 使用包、单元包及模块管理
可以将代码拆分为不同的模块并使用 不同的文件来管理它们。一个包(package)可以拥有多个二进制单元 包及一个可选的库单元包。而随着包内代码规模的增长，你还可以将 部分代码拆分到独立的单元包(crate)中，并将它作为外部依赖进行引用。

另外一个与组织和封装密切相关的概念被称为作用域(scope): 在编写代码的嵌套上下文中有一系列被定义在“作用域内”的名字。

Rust提供了一系列的功能来帮助我们管理代码，包括决定哪些细 节是暴露的、哪些细节是私有的，以及不同的作用域内存在哪些名 称。这些功能有时被统称为模块系统(module system)，它们包括:
- 包package：一个用于构建、测试并分享单元包的Cargo功能
- 单元包crate：一个用于生成库或可执行文件的树形模块结构
- 模块module及use关键字：被用于控制文件结构，作用域及路径的私有性
- 路径path：一种用于命名条目的方法，包括结构体、函数和模块等

### 包和单元包
单元包可以被用于生成二进制程序或库。我们将Rust编译时所使用的入口文件称作这个单元包的根节点，它同时也是单元包的根模块。而包则由一个或多个提供相关功能的单元包集合而成，它所附带的配置文件Cargo.toml描述了如何构建这些单元包的信息。

有几条规则决定了包可以包含哪些东西。首先，一个包中只能拥有最多一个库单元包。其次，包可以拥有任意多个二进制单元包。最后，包内必须存在至少一个单元包(库单元包或二进制单元包)。

当我们执行这条命令时，Cargo会生成一个包并创建相应的 Cargo.toml文件。Cargo会默认将`src/main.rs`视作一个二进制单元包的根节点而无须指定，这个二进制单元包与包拥有相同的名称。同样地，假设包的目录中包含文件`src/lib.rs`，Cargo也 会自动将其视作与包同名的库单元包的根节点。Cargo会在构建库和二进制程序时将这些单元包的根节点文件作为参数传递给rustc。

单元包可以将相关的功能分组，并放到同一作用域下，这样便可以使这些功能轻松地在多个项目中共享。使用过的 rand包(rand crate)提供了生成随机数的功能。而为了使用这些功能，我们只需要将rand包引入当前项目的作用域中即可。所有由rand包提供的功能都可以通过单元包的名称rand来访问。

将单元包的功能保留在它们自己的作用域中有助于指明某个特定 功能来源于哪个单元包，并避免可能的命名冲突。例如，rand包提供 了一个名为Rng的trait，我们同样也可以在自己的单元包中定义一个 名为Rng的struct。正是因为这些功能被放置在了各自的作用域中，当 我们将rand添加为依赖时，编译器才不会为某个Rng的具体含义是什么 而困惑。在我们的单元包中，它指向刚刚定义的struct Rng。我们可 以通过rand::Rng来访问rand包中的Rng trait。

### 通过定义模块来控制作用域及私有性
条目命名的路径，可以将路径引入作用域的use关键字，以及能够将条目标记为公开的pub关键字。另外，我们还会学习如何使用as关键字、外部项目及通配符。

模块允许我们将单元包内的代码按照可读性与易用性来进行分组。与此同时，它还允许我们控制条目的私有性。换句话说，模块决定了一个条目是否可以被外部代码使用(公共)，或者仅仅只是一个内部的实现细节而不对外暴露(私有)。
```
- crate
  - front_of_house
    - hosting
      - add_to_waitlist
      - seat_at_table
    - serving
      - take_order
      - serve_order
      - take_payment
```

### 用来在模块树中指明条目的路径
类似于在文件系统中使用路径进行导航的方式，为了在Rust的模块树中找到某个条目，我们同样需要使用路径。比如，在调用某个函数时，我们必须要知晓它的路径。路径有两种形式：
- 使用单元包名或字面量crate从根节点开始的绝对路径
- 使用self、super或内部标识符从当前模块开始的相对路径
绝对路径与相对路径都由至少一个标识符组成，标识符之间使用双冒号`::`分隔

```rust
mod front_of_house {
  mod hosting {
    fn add_to_waitlist() {}
    fn seat_at_table() {}
  }

  mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
  }
}

pub fn eat_at_restaurant() {
  // absolute path
  crate::front_of_house::hosting::add_to_waitlist();
  // relative path
  front_of_house::hosting::add_to_waitlist();
}
```
模块hosting是私有的。换句话说，虽然我们拥有指向hosting模块及add_to_waitlist函数的正确路径，但由 于缺少访问私有域的权限，所以Rust依然不允许我们访问它们。

模块不仅仅被用于组织代码，同时还定义了Rust中的私有边界 (privacy boundary):外部代码无法知晓、调用或依赖那些由私有 边界封装了的实现细节。因此，当你想要将一个条目(比如函数或结 构体)声明为私有时，你可以将它放置到某个模块中。

Rust中的所有条目(函数、方法、结构体、枚举、模块及常量) 默认都是私有的。处于父级模块中的条目无法使用子模块中的私有条 目，但子模块中的条目可以使用它所有祖先模块中的条目。虽然子模 块包装并隐藏了自身的实现细节，但它却依然能够感知当前定义环境 中的上下文。

Rust之所以选择让模块系统这样运作，是因为我们希望默认隐藏 内部的实现细节。这样，你就能够明确地知道修改哪些内部实现不会 破坏外部代码。同时，你也可以使用pub关键字来将某些条目标记为公 共的，从而使子模块中的这些部分被暴露到祖先模块中。

### 使用pub关键字来暴露路径
```rust
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}    
  }
}

pub fn eat_at_restaurant() {
  // absolute path
  crate::front_of_house::hosting::add_to_waitlist();
  // relative path
  front_of_house::hosting::add_to_waitlist();
}
```

### 使用super关键字开始构造相对路径
```rust
fn serve_order() {}

mod back_of_house {
  fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
  }

  fn cook_order() {}
}
```

### 将结构体或枚举声明为公共的
结构体与枚举都可以使用pub来声明为公共的，但需要注意其中存 在一些细微差别。当我们在结构体定义前使用pub时，结构体本身就成为了公共结构体，但它的字段依旧保持了私有状态。我们可以逐一决定是否将某个字段公开。
```rust
mod back_of_house {
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }
}

let mut meal = back_of_house::Breakfast::summer("Rye");

meal.toast = String::from("Wheat");
println!("I'd like {} toast please", meal.toast);
```
```rust
mod back_of_house {
  pub enum Appetizer {
    Soup,
    Salad,
  }
}

pub fn eat_at_restaurant() {
  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;
}
```

因为Appetizer枚举具有公共属性，所以我们能够在 eat_at_restaurant中使用Soup与Salad变体。枚举与结构体之所以不同，是由于枚举只有在所有变体都公共可用时才能实现最大的功效， 而必须为所有枚举变体添加pub则显得烦琐了一些，因此所有的枚举变体默认都是公共的。对于结构体而言，即便部分字段是私有的也不会影响到它自身的使用，所以结构体字段遵循了默认的私有性规则，除非被标记为pub，否则默认是私有的。

### 用use关键字将路径导入作用域
我们可以借助use关键字来将路径引入作用域，并像使 用本地条目一样来调用路径中的条目。

使用use来指定相对路径稍有一些不同。我们必须在传递给use的 路径的开始处使用关键字self，而不是从当前作用域中可用的名称开始。
```rust
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}    
  }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
}
```

### 创建use路径时的惯用模式
使用use将函数的父模块引入作用域意味着，我们必须在调用函数时指定这个父模块，从而更清晰地表明当前函数没有被定义在当前作用域中。当然，这一 方式同样也尽可能地避免了重复完整路径。另一方面，当使用use将结构体、枚举和其他条目引入作用域时，我们习惯于通过指定完整路径的方式引入。
```rust
use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert(1, 2);
}
```
假如我们需要将两个拥有相同名称的条目引入作用域，那么就应该避免使用上述模式，因为Rust并不支持这样的情形。
```rust
use std::fmt;
use std::io;

fn functiona1() -> fmt::Result {
  // ...
}

fn functiona2() -> io::Result {
  // ...
}
```
我们可以使用父模块来区分两个不同的 Result类型。但是，假设我们直接指定了use std::fmt::Result与use std::io::Result，那么同一作用域内就会出现两个Result类型，这时 Rust便无法在我们使用Result时确定使用的是哪一个Result。

### 使用as关键字来提供新的名称
使用use将同名类型引入作用域时所产生的问题还有另外一种解决办法:我们可以在路径后使用as关键字为类型指定一个新的本地名称，也就是别名。
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn functiona1() -> Result {
  // ...
}

fn functiona2() -> IoResult<()> {
  // ...
}
```

### 使用pub use重导出名称
当我们使用use关键字将名称引入作用域时，这个名称会以私有的 方式在新的作用域中生效。为了让外部代码能够访问到这些名称，我 们可以通过组合使用pub与use实现。这项技术也被称作重导出(re- exporting)，因为我们不仅将条目引入了作用域，而且使该条目可以 被外部代码从新的作用域引入自己的作用域。
```rust
pub use crate::front_of_house::hosting;
```
通过使用pub use，外部代码现在也能够借助路径 hosting::add_to_ waitlist来调用add_to_waitlist函数了。假设我 们没有指定pub use，那么虽然eat_at_restaurant函数能够在自己的 作用域中调用hosting:: add_to_waitlist，但外部代码则无法访问这 一新路径。

当代码的内部结构与外部所期望的访问结构不同时，重导出技术 会显得非常有用。例如，在这个餐厅的比喻中，餐厅的员工会以“前厅”和“后厨”来区分工作区域，但访问餐厅的顾客则不会以这样的术语来考虑餐厅的结构。通过使用pub use，我们可以在编写代码时使 用一种结构，而在对外部暴露时使用另外一种不同的结构。这一方法 可以让我们的代码库对编写者与调用者同时保持良好的组织结构。

### 使用外部包
在`Cargo.toml`中添加rand作为依赖会指派Cargo从crates.io上下载rand及相关的依赖包，并使rand对当前的项目可用。
```rust
[dependencies]
rand = "0.5.5"
```
接着，为了将rand定义引入当前包的作用域，我们以包名rand开始添加了一行use语句，并在包名后列出了我们想要引入作用域的条目。
```rust
use rand::Rng;
fn main() {
  let secret_number = rand::thread_rng().gen_range(1, 101);
}
```
注意，标准库(std)实际上也同样被视作当前项目的外部包。由于标准库已经被内置到了Rust语言中，所以我们不需要特意修改`Cargo.toml`来包含std。但是，我们同样需要使用use来将标准库中特定的条目引入当前项目的作用域。
```rust
use std::collections::HashMap;
```

### 使用嵌套的路径来清理众多use语句
当我们想要使用同一个包或同一个模块内的多个条目时，将它们逐行列出会占据较多的纵向空间。

我们还可以在同一行内使用嵌套路径来将上述条目引入作用域。这一方法需要我们首先指定路径的相同部分，再在后面跟上两个冒号，接着用一对花括号包裹路径差异部分的列表
```rust
use std::{cmp::Ordering, io};
```
在一些复杂的项目里，使用嵌套路径来将众多条目从同一个包或同一个模块引入作用域可以节省大量的独立use语句。

我们可以在路径的任意层级使用嵌套路径，这一特性对于合并两行共享子路径的use语句十分有用。
```rust
use std::io;
use std::io::Write;
// 可以将两者合并起来并使用self
use std::io::{self, Write};
```

### 通配符
假如你想要将所有定义在某个路径中的公共条目都导入作用域，那么可以在指定路径时在后面使用*通配符:
```rust
use std::collections::*;
```
use语句会将定义在std::collections内的所有公共条目 都导入当前作用域。请小心谨慎地使用这一特性!通配符会使你难以确定作用域中存在哪些名称，以及某个名称的具体定义位置。

### 将模块拆分为不同的文件








