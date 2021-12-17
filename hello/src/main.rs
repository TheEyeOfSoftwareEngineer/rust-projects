fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // Rust 有return语句，但gcd函数中没有。如果函数体中最后一行代码是一个表达式，且表达式末尾没有分号，那这个表达式的值就是函数的返回值
    n
}

#[test]
fn test_gcd() {
         assert_eq!(gcd(14, 15), 1);
         assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                        3 * 7 * 11 * 13 * 19),
                    3 * 11);
}


fn main() {
    println!("Hello, world!");
}


