fn main(){
    // 引用与借用
    let s1 = String::from("hello");

    //&语法得到指向s1的引用
    let len = calculate_length(&s1);//获取引用作为函数参数称为借用
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
    //let r2 = &mut s;            // 这行会编译不通过，因为s已经有一个可变引用
    println!("{}",r1);

    // 可通过大括号创建新作用域来使用多个可变引用
    let mut s = String::from("hello");

    {
        let r1 = &mut s;

        println!("{}",r1);
    }
    let r2 = &mut s;            // 这里仍然可以创建s的可变引用
    println!("{}",r2);

    // 可变引用的限制可以在编译期就避免数据竞争。数据竞争可由三个行为造成：
    //  a. 两个或更多指针同时访问同一数据
    //  b. 至少有一个指针被用来写入数据
    //  c. 没有同步数据访问的机制

    // 数据不能同时有可变引用和不可变引用，但是可以有多个不可变引用
    //let mut s = String::from("hello"); // 这里的mut是不必要的，下面的mut引用会报错
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    //let r3 = &mut s;            // 这里会报错
    println!("{},{}",r1,r2);

    //引用的作用域从声明的地方开始一直持续到最后一次使用为止。以下代码是可以编译的
    let mut s = String::from("hello");
    let _r1 = &s;
    let _r2 = &s;
    println!("{},{}",_r1,_r2);
    //到这里_r1,_r2已经不再使用
    let r3 = &mut s;               // 没有问题
    println!("{}",r3);


    // 悬垂指针问题：悬垂指针指向的内存可能已经被分配给其他持有者。
    // rust编译器确保引用永远也不会变成悬垂状态：当你用有一些数据的引用时，
    // 编译器确保数据不会在其引用之前离开作用域。
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
// 这种情况下，只能返回s，让所有权转移出去

// 引用的总结：
//  - 任意给定时间，要么只能有一个可变引用，要么只能有我个不可变引用。
//  - 引用必须总是有效的。
