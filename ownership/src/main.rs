
fn main(){
    // 所有权是rust的核心功能(之一)，Rust没有使用垃圾回收机制，也没有使用手动内存管理，而是使用所有权管理内存。
    // 所有权检查发生在编译期，不会减慢运行速度。
    // 它的基本规则
    // 1. Rust中每个值都有一个称之为其所有者(owner)的变量
    // 2. 值有且只能有一个所有者
    // 3. 当所有者(变量)离开作用域，这个值将被丢弃

    // 4.1 作用域(scope)
    {                           // 这里s不可用，因为它还未声明
        //println!("{}",s);
        let s = "hello";        // s从这里开始可用
        println!("{}",s);
    }

    // String是字面量字符串之外的另一种字符串类型，String是可变类型
    {
        let mut s = String::from("hello"); // 内存申请。s必须为mut，否则 push_str会编译报错
        s.push_str(", world!");
        println!("{}",s);           
    }//变量离开作用域时，rust在}位置自动调用drop函数
    
    // 变量与数据交互的方式
    // 一、移动：
    // 类似其他语言的浅拷贝，只不过前一变量会失效。因为前一变量会失效，因此这一操作被称为移动(move)
    let x = String::from("hello");
    let y = x;
    //println!("{}",x);            // 不是标量类型，控制权转移后，x无效了，编译出错。类似其它语言中的浅拷贝，只是这个浅拷贝操作后，原来的变量无效了
    println!("{}",y);

    // 二、复制
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1={},clone s1 to s2,s2={}",s1,s2);

    //数据因为是整型的，其大小在编译时已经确定。它是分配在栈上的，它会被拷贝，不需要调用clone方法。 
    //rust有一个名为Copy trait的注解，如果类型有Copy trait，则旧变量在赋给其它变量后将仍然可用。
    //rust不允许自身或其任何部分实现了Drop trait的类型使用Copy trait。
    //什么类型是Copy的需要查看类型的文档来确定。
    //作为通用的规则，任何简单标题值的组合可以是Copy的，不需要分配内存或某种形式的资源的类型是Copy的，如：
    // a. 所有整数类型
    // b. 布尔类型
    // c. 浮点数 
    // d. 字符类型 char
    // e. 所有元素都是Copy的元组
    let x = 5;
    let y = x;
    println!("x={},y={}",x,y);            // 标量类型值为Copy，编译能通过


    // 所有权与函数
    // 值传递给函数在语义上与给变量赋值相似。可能会移动或复制。

    let s = String::from("hello");
    takes_ownership(s);         
    //println!("{}",s);  // s的值转入函数，s将不再有效

    let x = 5;
    makes_copy(x);              
    println!("{}",x);  // i32是Copy，因此x仍然有效


    // 返回值与作用域
    let s1 = gives_ownership(); // 所有权返回

    let s2 = String::from("hello"); 

    let s3 = takes_and_gives_back(s2); // 所有权传入再返回

    // println!("{},{},{}",s1,s2,s3); s2已经不再有效
    println!("{},{}",s1,s3);

    // 当有所有权要返回，同时还要返回值时可以用元组来返回数据
    let s1 = String::from("hello");
    let (s2,len) = calculate_length(s1);
    println!("{} length {}",s2,len);
}

fn takes_ownership(some_string:String){
    println!("{}",some_string);
}

fn makes_copy(some_integer:i32){
    println!("{}",some_integer);
}

fn gives_ownership() -> String{
    let some_string = String::from("hello");
    
    some_string
}

fn takes_and_gives_back(a_string:String) -> String {
    a_string
}

fn calculate_length(s:String) -> (String, usize){
    let length = s.len();
    (s,length)
}
