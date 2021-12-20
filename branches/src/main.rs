fn main() {
    let number = 3;
    if number < 5 {
      println!("condition was true");
    } else {
      println!("condition was false");
    }
    for number in (1..4).rev() {
      println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
