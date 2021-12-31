## Cargo和crates.io
```rust
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```
选项opt-level决定了Rust在编译时会对代码执行何种程度的优化，从0到3都是合法的配置值。越高级的优化需要消耗越多的编译时间，当你处于开发阶段并常常需要编译代码时，你也许宁可牺牲编译 产出物的运行速度，也想要尽可能地缩短编译时间。这就是dev配置下的默认opt-level值为0的原因。而当你准备好最终发布产品时，则最好花费更多的时间来编译程序。因为你只需要在发布时编译一次，但却会多次运行编译后的程序，所以发布模式会使用更长的编译时间来交换更佳的运行时性能。这就是release配置下的默认opt-level值为3的原因。

### 编写有用的文档注释
可以使用三斜线(///)而不是双斜线来编写文档注释，并且 可以在文档注释中使用Markdown语法来格式化内容
```rust
/// 将传入的数字加
/// 
/// # Example
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
  x + 1
}
```
为整个包添加描述性文档
```rust
//! # my_crate
//! 
//! my_crate是一系列工具的集合
//! 这些工具被用来简化特定的计算操作
```

### 使用pub use来导出合适的公共API
代码模块可能是一个包 含多个层次的树状结构，但当用户想要使用某个较深层次中的类型时 就会在查找过程中遇到麻烦。另外，在引入数据时需要输入use my_crate::some_module::another_module::UsefulType;，这比输入简单的use my_crate:: UsefulType;要烦人得多
```rust
//! # Art
//! 
//! 一个用来建模艺术概念的代码库
pub mod kinds {
  /// RYB颜色模型的三原色
  pub enum PrimaryColor {
    Red,
    Yellow,
    Blue,
  }

  /// RYB模型的调和色
  pub enum SecondaryColor {
    Orange,
    Green,
    Purple,
  }
}

pub mod utils {
  use crate::kinds::*;
  /// 将两种等量的原色混合生成调和色
  pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
    // ...
  }
}
```
```rust
use art::kinds::PrimaryColor:
use art::utils::mix;

fn main() {
  let red = PrimaryColor::Red;
  let yellow = PrimaryColor::Yellow;
  mix(red, yellow);
}
```
为了从公共API中移除内部结构，我们可以修改art包代码，使用pub use语句将需要公开的条目重新导出到顶层结构中
```rust
//! # Art
//! 
//! A library for modeling artistic concepts
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
  // ...
}

pub mod utils {
  // ...
}
```
```rust
use art::PrimaryColor;
use art::mix;

fn main() {
  // ...
}
```
当存在较多嵌套模块时，使用pub use将类型重新导出到顶层模块可以显著地改善用户体验

### Cargo工作空间
#### 创建工作空间
工作空间是由共用同一个Cargo.lock和输出目录的一系列包所组成的


