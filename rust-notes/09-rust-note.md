## 编写自动化测试

### 测试函数的构成
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {        
        assert_eq!(2 * 2, 4);
    }
    #[test]
    fn another() {
      panic!("Make this test fail");
    }
}
```

### 使用assert！宏检测结果
```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
      let larger = Rectangle { length: 8, width: 7};
      let smaller = Rectangle { length: 5, width: 1};
      assert!(larger.can_hold(&smaller))
    }
    #[test]
    fn smaller_cannot_hold_larger() {
      let larger = Rectangle { length: 8, width: 7};
      let smaller = Rectangle { length: 5, width: 1};
      assert!(!smaller.can_hold(&larger));
    }
}

#[derive(Debug)]
pub struct Rectangle {
  length: u32,
  width: u32,
}

impl Retangle {
  pub fn can_hold(&self, other: &Rectangle) -> bool {
    self.length > other.length && self.width > other.width
  }
}
```
`use super::*`: 因为tests是一个内部模块，所以我们必须将外部模块中的代码导入内部模块的作用域中。这里使用了通配符(*)让外层模块所定义的全部内容在tests模块中都可用

### 使用`assert_eq!`宏和`assert_ne!`宏判断相等性
```rust
pub fn add_two(a: i32) -> i32 {
  a + 2
}

#[test]
fn it_adds_two() {
  assert_eq!(4, add_two(2));
}
```

### 添加自定义的错误提示信息
```rust
#[test]
fn greeting_contains_name() {
  let result = greeting("Carol");
  assert!(
    result.contains("Carol"),
    "Greeting did not contain name, values was `{}`", result
  );
}
```

### 使用should_panic检查panic
```rust
pub struct Guess {
  value: u32,
}

impl Guess {
  pub fn new(value: u32) -> Guess {
    if value < 1  {
      panic!("Guess value must be greater than 1 or equal to 1, got {}.", value);
    } else if value > 100 {
      panic!("Guess value must be less than or equal to 100, got {}.", value);
    }
    Guess {
      value
    }
  }
}

#[test]
#[should_panic(expected = "Guess value must be less than or equal to 100")]
fn greater_than_100() {
  Guess::new(200);
}
```

### 使用Result<T, E>编写测试
```rust
#[test]
fn it_works() -> Result<(), String> {
  if 2 + 2 == 4 {
    Ok(())
  } else {
    Err(String::from("two plus two does not equal four"))
  }
}
```


### 二进制包的集成测试
如果我们的项目是一个只有 文件而没有src/main.rs文件的二进制包，那么我们就无法在src/lib.rs目录中创建集成测试，也无法使用use语句将src/main.rs中定义的函数导入作用域。只有代码包(library crate)才可以将函数暴露给其他包来调用，而二进制包只被用于独立执行。**这就是Rust的二进制项目经常会把逻辑编写在src/lib.rs文件 中，而只在main.rs文件中进行简单调用的原因**。这种组织结构 使得集成测试可以将我们的项目视作一个代码包，并能够使用use访问包中的核心功能。只要我们能够保证核心功能一切正常， main.rs中的少量胶水代码就能够工作，无须进行测试。