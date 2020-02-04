fn main() {
    // rust核心语言中只有一种字符串类型str，字符串slice，它通常以被借用的形式出现 &str。它们是存储在别处的UTF-8编码字符串数据的引用。
    // 称为String的类型是由标准库提供的，不是核心语言的一部分。它是可增长、可变、有所有权的、UTF-8编码的字符串类型。

    // 新建字符串
    let mut _s = String::new();
    let data = "initial contents";

    let _s = data.to_string();

    // 该方法也可直接用于字符串字面值：
    let _s = "initial contents".to_string();

    // 更新字符串
    let mut s1 = String::from("foo");
    // 如果在后面加上.to_string();，s2会变成String类型，会变得有所有权。
    // 下面的push_str调用会获得所有权。之后println!打印s2将会无法编译。
    let s2 = "bar";
    s1.push_str(s2);
    s1.push('!');                // 单个字符
    println!("{}",s2);
    println!("{}",s1);
    
    let data = "hello world";
    let s = data.to_string();
    println!("{}",s);

    // 使用+运算符拼接
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;          // s1从此不再有效，因为s1使用的不是引用，所有权被转移了
    //上面的&s2并不符合函数定义的方法签名，+函数要求s2为&str，而不是&String，为什么能编译通过呢？
    //因为&String可以被强转为&str(解引用强制多态技术)
    println!("{}",s3);
    //+的实现看起来像是拼接了两个字符串，实际上它是获得了s1所有权，接接上s2的拷贝。实际上的实现比拷贝要更高效。

    //使用format拼接
    let s = format!("{}-{}",s2,s3);
    println!("{}",s);

    // 字符串不支持索引，它实际上是对Vec<u8>的封装
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
