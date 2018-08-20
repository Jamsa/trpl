fn main(){
    // 引用与借用

    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    // 得到对象的引用，并没有取得其所有权，当离开calculate_length函数作用域时，它也不会被丢弃
    println!("The length of '{}' is {}.",s1,len);

    // 可变引用本身应该是mut
    //let s = String::from("hello");
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}",s);


    // 可变引用的限制：在特定作用域中特定的数据有且只有一个可变引用
    let mut s = String::from("hello");

    let r1 = &mut s;
    //let r2 = &mut s;            // 这行会编译不通过，因为 s已经有一个可变引用
    println!("{}",r1);

    // 可通过大括号创建新作用域来使用多个可变引用
    let mut s = String::from("hello");

    {
        let r1 = &mut s;

        println!("{}",r1);
    }
    let r2 = &mut s;            // 这里仍然可以创建s的可变引用
    println!("{}",r2);

    // 数据不能同时有可变引用和不可变引用
    //let mut s = String::from("hello"); // 这里的mut是不必要的，下面的mut引用会报错
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    //let r3 = &mut s;            // 这里会报错
    println!("{},{}",r1,r2);

    // 悬垂指针问题
    //let reference_to_nothing = dangle();

}


// 获取引用作为函数参数称为借用（borrowing）
fn calculate_length(s: &String) -> usize {
    // 借用的变量默认是不可变的
    //s.push_str("aaa");
    s.len()
}

// 可变引用
fn change(some_string: &mut String){
    some_string.push_str(", world");
}

// 这个函数试图返回悬垂引用，离开dangle函数作用域后，s会被释放，返回的 &s会成为悬垂引用
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
