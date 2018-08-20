enum IpAddkind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddkind,
    address: String,
}

fn route(ip_type: IpAddkind){}

// 将数据放入枚举成员
enum IpAddr1 {
    V4(u8,u8,u8,u8),
    V6(String),
}

// 包含不同类型成员的枚举
enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

//上面的代码如果不使用枚举，而使用struct来实现，则要定义多种不同的struct

// 枚举也可以使用impl来定义方法

impl Message {
    fn call(&self){
    }
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x:Option<i32>) -> Option<i32>{
    match x {
        //None => None,           // 模式匹配需要穷尽，否则编译无法通过
        Some(i) => Some(i + 1),
        _ => None,              // 可以使用 _ 通配符
    }
}

fn main() {
    let four = IpAddkind::V4;
    let six = IpAddkind::V6;
    route(four);
    route(six);
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option枚举（enum Option像scala case class了）
    let some_number = Some(5);
    let som_string = Some("a String");
    let absent_number: Option<i32> = None;
    println!("Option test:{:?},{:?},{:?}",some_number,som_string,absent_number);

    // 模式匹配
    let c = Coin::Penny;
    println!("c: {}",value_in_cents(c));

    println!("plus_one:{:?}",plus_one(Some(20)));

    // 使用if let语法糖，忽略其它模式
    if let Some(5) = some_number {
        println!("five");
    }else{                      // 匹配其它模式
        println!("not five");
    }
}
