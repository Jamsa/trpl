fn main() {
    // rust核心语言中只有一种字符串类型str，字符串slice，它通常以被借用的形式出现 &str。它们是存储在别处的UTF-8编码字符串数据的引用。
    // 称为String的类型是由标准库提供的，不是核心语言的一部分。它是可增长、可变、有所有权的、UTF-8编码的字符串类型。

    // 新建字符串
    let mut s = String::new();
    s.push_str("foo bar");
    s.push('!');                // 单个字符
    println!("{}",s);
    
    let data = "hello world";
    let s = data.to_string();
    println!("{}",s);

    // 使用+运算符拼接
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;          // s1从此不再有效，因为s1使用的不是引用，所有权被转移了
    println!("{}",s3);

    let s = format!("{}-{}",s2,s3);
    println!("{}",s);

    // 字符串不支持索引
    //s3[0]
    // 可以使用slice

    // 遍历字符串
    for c in "नमस्ते".chars() {   // 梵文
        println!("{}", c);
    }

    for c in "नमस्ते".bytes() {
        println!("{}", c);      // 每个字符可能不止一个byte，这里是3*6个字节
    }

}
