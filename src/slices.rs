fn main(){
    // slice是另一个没有所有权的数据类型。它允许你引用集合中一段连续的元素序列
    let s = String::from("hello world");
    let hello = &s[0..5];
    print!("{}", hello);

    // s是mut的s.clear才可调用。但这时无法进行slice操作。下面两种写法都不能
    // 一、
    // let mut s = String::from("hello world");
    // let hello = &s[0..5];
    // s.clear();
    // print!("{}", hello);
    // 二、
    // let mut s = String::from("hello world");
    // let hello = &mut s[0..5];
    // s.clear();                  // 这里两次mut引用 s
    // print!("{}", hello);

    print!("{}", &s[..]);       // 取整个串

    // 字符串字面量就是slice。s的类型是 &str它是一个指向二进制程序特定位置的slice。
    let s = "Hello, world!";
    print!("{}", s);

    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("{}",word);
    
    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);
    println!("{}",word);
    
    let word = first_word(my_string_literal);
    println!("{}",word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

