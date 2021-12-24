// 在mod front_of_house后使用分号而不是代码块会让Rust前往与 当前模块同名的文件中加载模块内容
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
}

// mod back_of_house {
//   pub enum Appetizer {
//     Soup,
//     Salad,
//   }
// }