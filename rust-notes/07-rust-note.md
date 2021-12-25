## 错误处理
在Rust中，我们将错误分为两大类:可恢复错误与不可恢复错误。对于可恢复错误，比如文件未找到等，一般需要将它们报告给用户并再次尝试进行操作。而不可恢复错误往往就是bug的另一种说法，比如尝试访问超出数组结尾的位置等。

### 不可恢复错误和panic!
```rust
fn main() {
    panic!("crash and burn");
}
```

### 使用panic！产生的回溯信息
```rust
fn main() {
  let v = vec![1, 2, 3];
  v[99];
}
```
在类似于C这样的语言中，程序在这种情况下依然会尝试返回你所请求的值，即便这可能会与你所期望的并不相符:你会得到动态数组中对应这个索引位置的内存，而这个内存可能存储了其他数据，甚至都不属于动态数组本身。这种情形也被称为缓冲区溢出 (buffer overread)，并可能导致严重的安全性问题。攻击者可以通过操纵索引来访问存储在数组后面的、那些不被允许读取的数据。

为了保护我们的程序，避免出现类似的漏洞，当你尝试读取一个非法索引指向的元素时，Rust会拒绝继续执行代码，并终止程序

### 可恢复错误与Result
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```rust
use std::fs::File;
fn main() {
  let f = File::open("Hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => {
      panic!("Problem opening the file: {:?}", error);
    }
  }
}
```
> 与Option枚举一样，Result枚举及其变体已经通过预导入模块被自动地引入当前作用域中，所以我们不需要在使用Ok变体与Err变体之前在match分支中显式地声明Result::

### 匹配不同的错误
```rust
use std::fs::File;
use std::io::ErrorKind;
fn main() {
  let f = File::open("Hello.txt");
  let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("Hello.txt") {
        Ok(fc) => fc;
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      other_error => panic!("Problem opening the file: {:?}", other_error),
    },
  };
}
```
File::open返回的Err变体中的错误值类型，是定义在某个标准库中的结构体类型io::Error。这个结构体拥有一个被称作kind的方 法，我们可以通过调用它来获得io::ErrorKind值。这个io::ErrorKind枚举是由标准库提供的，它的变体被用于描述io操作所 可能导致的不同错误。这里使用的变体是ErrorKind::NotFound，它用 于说明我们尝试打开的文件不存在。所以，我们不但对变量f使用了match表达式，还在内部对error.kind()使用了match表达式。
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let f = File::open("Hello.txt").map_err(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("Hello.txt").unwrap_or_else(|error| {
        panic!("Problem creating the file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file: {:?}", error);
    }
  });
}
```

### 失败时触发panic的快捷方式：unwrap和except
当Result 的返回值是Ok变体时，unwrap就会返回Ok内部的值。而当Result的返回值是Err变体时，unwrap则会替我们调用panic!宏
```rust
use std::fs::File;
fn main() {
  let f = File::open("Hello.txt").unwrap();
}
```
使用expect所实现的功能与unwrap完全一样:要么返回指定文件句柄，要么触发panic!宏调用。唯一的区别在于，expect触发panic!时会将传入的参数字符串作为错误提示信息输出，而unwrap触发的panic!则只会携带一段简短的默认信息

### 传播错误
当你编写的函数中包含了一些可能会执行失败的调用时，除了可以在函数中处理这个错误，还可以将这个错误返回给调用者，让他们决定应该如何做进一步处理。这个过程也被称作传播错误，在调用代 码时它给了用户更多的控制能力。与编写代码时的上下文环境相比，调用者可能会拥有更多的信息和逻辑来决定应该如何处理错误
```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}
```
- `Result<String, io::Error>`: 意味着这个函数的返回值的类型为`Result<T, E>`，其中的泛型参数T被替换为具体的String类型，而泛型E则被替换为具体的`io::Error`类型
- 当这个函数顺利运行时，调用这 个函数的代码将会获得一个包裹在Ok中的String值，也就是这个函数从文件中读取的用户名。而假如这个函数碰到了某个问题，函数的调用者就会获得一个包含了io::Error实例的Err值
- 之所以选择io::Error作为函数的返回类型， 是因为函数中另外两个可能会失败的操作，File::open函数及 read_to_string方法，恰好同样使用了io::Error作为错误类型
传播错误的模式在Rust编程中非常常见，所以Rust专门提供了一个问号运算符(?)来简化它的语法
- 传播错误的快捷方式:?运算符
```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
  let mut f = File::open("hello.txt");
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}
```
通过将?放置于Result值之后，我们实现了与使用match表达式来处理Result时一样的功能。假如这个Result的值是Ok，那么包含在Ok中的值就会作为这个表达式的结果返回并继续执行程序。假如值是Err，那么这个值就会作为整个程序的结果返回，如同使用了return一样将错误传播给调用者。

match表达式与?运算符的一个区别:被?运算符所接收的错误值会隐式地被from函数处理，这个函数定义于标准库的From trait中，用于在错误类型之间进行转换。当?运算符调用from函数时，它就开始尝试将传入的错误类型转 换为当前函数的返回错误类型。当一个函数拥有不同的失败原因，却使用了统一的错误返回类型来同时进行表达时，这个功能会十分有用。只要每个错误类型都实现了转换为返回错误类型的from函数，?运算符就会自动帮我们处理所有的转换过程。

?运算符帮助我们消除了大量模板代码，使函数实现更为简单。我们甚至还可以通过链式方法调用来进一步简化这些代码
```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
  let mut s String::new();
  File::open("hello.txt")?.read_to_string(&mut s)?;
  Ok(s)
}
```
如果只是单纯地想要缩短代码
```rust
use std::io;
use std::fs;
fn read_username_from_file() -> Result<String, io::Error> {
  fs::readed_to_string("hello.txt")
}
```
- ?运算符只能被用于返回Result的函数
因为?运算符的功能类似于定义的match表达式，所以它只能被用于那些拥有Result返回类型的函数。在match表达式中，return Err(e)部分产生的返回类型是Result，所以函数的返回类型也必须是Result，才能与此处的return兼容。
```rust
use std::fs::File;

fn main() {
  // 使用了?运算符的函数必须返回 Result、Option或任何实现了std::ops::Try的类型
  let f = File::open("hello.txt")?;
}
```
在那些没有返回上述类型的函数里，一旦调用的其他函数返回了`Result<T, E>`，就需要使用match或`Result<T, E>`自身的方法来对`Result<T, E>`进行恰当的 处理。当然，你也可以选择在合适的条件下将函数的返回类型修改为`Result<T, E>`。

对于特殊的main函数而言，可用的返回类型除了()，还有更加方便的`Result<T, E>`
```rust
use std::error::Error;
use std::fs::File;
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(()) 
}
```
这里的`Box<dyn Error>`被称作trait对象. 以简单地将`Box<dyn Error>`理解为“任何可能的错误类型”。在拥有这种返回类型的main函数中使用?运算符是合法的

### 创建自定义类型来进行有效型校验
```rust
loop {
  let guess: i32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
  };
  if guess < 1 || guess > 100 {
    println!("Please input a number between 1 and 100");
    continue;
  }
  
  match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
      println!("You win!");
      break;
    }
  }
}
```
相比于到处重复验证代码，我们可以创建一个新的类型，并在创建的类型实例的函数中对值进行有效性检查
```rust
pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}.", value);
    }
    Guess {
      value
    }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}
```





