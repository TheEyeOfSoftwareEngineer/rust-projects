## 函数式语言特性：迭代器与闭包

### 闭包：能够捕获环境的匿名函数
#### 使用闭包来创建抽象化的程序行为
```rust
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
  println!("Calculate slowly...");
  thread::sleep(Duration::from_secs(2));
  intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
  if intensity < 25 {
    println!(
      "Today, do {} pushups!",
      simulated_expensive_calculation(intensity)
    );
    println!(
      "Next, do {} situps!",
      simulated_expensive_calculation(intensity)
    );
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!")
    } else {
      println!(
        "Today, run for {} minutes!",
        simulated_expensive_calculation(intensity)
      );
    }
  }
}

fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(
    simulated_user_specified_value,
    simulated_random_number
  )
}
```

#### 使用函数来进行重构
```rust
fn generate_workout(intensity: u32, random_number: u32) {
  let expensive_result = 
      simulated_expensive_calculation(intensity);
  if intensity < 25 {
    println!(
      "Today, do {} pushups",
      expensive_result
    );
    println!(
      "Next, do {} situps",
      expensive_result
    );
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!")      
    } else {
      println!(
        "Today, run for {} minutes!",
        expensive_result
      );
    }
  }
}
```

#### 使用闭包存储代码来进行重构
```rust
let expensive_closures = |num| {
  println!("Calculating slowly...");
  thread::sleep(Duration::from_secs(2));
  num
};

fn generate_workout(intensity: u32, random_number: u32) {
  let expensive_closures = |num| {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  };

  if intensity < 25 {
    println!(
      "Today, do {} pushups",
      expensive_closures(intensity)
    );
    println!(
      "Next, do {} situps",
      expensive_closures(intensity)
    );
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!")      
    } else {
      println!(
        "Today, run for {} minutes!",
        expensive_closures(intensity)
      );
    }
  }
}
```

#### 闭包的类型推断和类型标注
```rust
fn add_one_v1(x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x| { x + 1 };
let add_one_v4 = |x| x + 1 ;

let example_closure = |x| x;
// 当我们首先使用String值调用example_closure时，编译器将闭包 的参数x的类型和返回值的类型都推导为了String类型。接着，这些类 型信息就被绑定到了example_closure闭包中，当我们尝试使用其他类 型调用这一闭包时就会触发类型不匹配的错误
let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

#### 使用泛型参数和Fn trait来存储闭包
创建一个同时存放 闭包及闭包返回值的结构体。这个结构体只会在我们需要获得结果值 时运行闭包，并将首次运行闭包时的结果缓存起来，这样余下的代码 就不必再负责存储结果，而可以直接复用该结果。这种模式一般被称 作 记忆化 (memoization)或 惰性求值 (lazy evaluation)
```rust
struct Cacher<T> where T: Fn(u32) -> u32 {
  calculate: T,
  value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      value: None,
    }
  }
  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      },
    }
  }
}

fn generate_workout(intensity: u32, random_number: u32) {
  let mut expensive_result = Cacher::new(|num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!(
      "Today, do {} pushups!",
      expensive_result.value(intensity)
    );
    println!(
      "Next, do {} situps",
      expensive_result.value(intensity)
    );
  } else {
    if random == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!(
        "Today, run for {} minutes",
        expensive_result.value(intensity)
      );
    }
  }
}
```

#### Cacher实现的局限性
```rust
#[test]
fn call_with_difference_values() {
  let mut c = Cacher::new(|a| a);

  let v1 = c.value(1);
  let v2 = c.value(2);

  assert_eq!(v2, 2);
}
```

#### 使用闭包捕获上下文环境
即使x不是equal_to_x的参数，equal_to_x闭包 也可以使用定义在同一个作用域中的变量x
```rust
fn main() {
  let x = 4;
  let equal_to_x = |z| z == x;
  let y = 4;
  assert!(equal_to_x(y));
}
```
闭包可以通过3种方式从它们的环境中捕获值，这和函数接收参数 的3种方式是完全一致的:获取所有权、可变借用及不可变借用。这3种方式被分别编码在如下所示的3种Fn系列的 trait中:
- FnOnce意味着闭包可以从它的封闭作用域中，也就是闭包所处的环境中，消耗捕获的变量。为了实现这一功能，闭包必须在定义时取得这些变量的所有权并将它们移动至闭包中。这也是名称FnOnce中Once一词的含义:因为闭包不能多次获取并消耗掉同一变量的所有权，所以它只能被调用一次
- FnMut可以从环境中可变地借用值并对它们进行修改
- Fn可以从环境中不可变地借用值
假如希望强制闭包获取环境中值的所有权，那么你可以在参数列表前添加move关键字。这个特性在把闭包传入新线程时相当有用，它可以将捕获的变量一并移动到新线程中去
```rust
fn main() {
  let x = vec![1, 2, 3];
  let equal_to_x = move |z| z == x;
  println!("can't use x here: {:?}", x);
  let y = vec![1, 2, 3];
  assert!(equal_to_x(y));
}
```

### 使用迭代器处理元素序列
```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();

for val in v1_iter {
  println!("Got: {}", val);
}
```
迭代器会处理所有上述的逻辑，这减少了重复代码并消除了潜在的混乱。另外，迭代器还可以用统一的逻辑来灵活处理各种不同种类的序列，而不仅仅只是像动态数组一样可以进行索引的数据结构

#### Iterator trait和next方法
```rust
pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>
}

#[test]
fn iterator_demonstration() {
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();

  assert_eq!(v1.iter.next(), Some(&1));
  assert_eq!(v1.iter.next(), Some(&2));
  assert_eq!(v1.iter.next(), Some(&3));
  assert_eq!(v1.iter.next(), None);
}
```

#### 消耗迭代器的方法
```rust
#[test]
fn iterator_sum() {
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();
  let total: i32 = v1.iter.sum();
  assert_eq!(total, 6);
}
```

#### 生成其他迭代器的方法
```rust
let v1: Vec<i32> = vec![1, 2, 3];
v1.iter().map(|x| x + 1).collect();
assert_eq!(v2, vec![2, 3, 4]);
```

#### 使用闭包捕获环境
```rust
#[derive(ParitialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter()
    .filter(|s| s.size == shoe_size)
    .collect()
}

#[test]
fn filters_by_size() {
  let shoes = vec![
    Shoe { size: 10, style: String::from("sneaker") },
    Shoe { size: 13, style: String::from("sandal") },
    Shoe { size: 10, style: String::from("boot") },
  ];
  let in_my_size = shoes_in_my_size(shoes, 10);
  assert_eq!(
    in_my_size,
    vec![
      Shoe { size: 10, style: String::from("sneaker") },
      Shoe { size: 10, style: String::from("boot") },
    ]
  )

}
```
into_iter来创建可以获取动态数组所有权的迭代器

#### 使用Iterator trait来创建自定义迭代器
- 通过调用动态数组的iter、into_iter及iter_mut方法来创建迭代器
- 可以采用类似的方法为标准库中的 其他集合类型(例如哈希表)创建迭代器
- 还可以通过实现Iterator trait来创建拥有自定义行为的迭代器
```rust
struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item: u32;
  fn next(&mut self) -> Option<Self::Item> {
    self.count += 1;
    if self.count < 6 {
      Some(self.count)
    } else {
      None
    }
  }
}

#[test]
fn calling_next_directly() {
  let mut counter = Counter::new();
  assert_eq!(counter.next(), Some(1));
  assert_eq!(counter.next(), Some(2));
  assert_eq!(counter.next(), Some(3));
  assert_eq!(counter.next(), Some(4));
  assert_eq!(counter.next(), Some(5));
  assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
  let sum: u32 = Counter::new().zip(Counter::new().skip(1)).map(|a, b| a * b).filter(|x| x % 3 == 0).sum(); // 2 6 12 20
  assert_eq!(18, sum);
}
```


