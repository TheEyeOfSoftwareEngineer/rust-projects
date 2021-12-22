#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

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

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // let rect = (30, 50);

    let rect1 = Rectangle {
      width: 30,
      height: 50,
    };
    let rect2 = Rectangle {
      width: 10,
      height: 40,
    };
    let rect3 = Rectangle {
      width: 60,
      height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));        

    println!("rect1 is {:#?}", rect1);

    // println!(
    //   "The area of the rectangle is {} square pixels.",
    //   area(&rect1)
    // );

    println!(
      "The area of the rectangle is {} square pixcels.",
      rect1.area()
    );
}

// fn area(width: u32, height: u32) -> u32 {
//   width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//   dimensions.0 * dimensions.1
// }
// 用于计算面积的area函数在被定义时只需要接收一个rectangle参数，它是结构体Rectangle实例的不可变借用
// 在函数签名和调用过程中使用&进行引用是因为 我们希望借用结构体，而不是获取它的所有权，这样main函数就可以 保留rect1的所有权并继续使用它。一定要注意借用的概念
// fn area(rectangle: &Rectangle) -> u32 {
//   rectangle.width * rectangle.height
// }