// 泛型、trait与生命周期
use std::cmp::PartialOrd;

// PartialOrd为trait bounds
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];  // 需要Copy trait才能复制值

    for &item in list.iter(){
        if item > largest {     // 需要 PartialOrd　trait 才能比较大小
            largest = item;
        }
    }

    largest
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}



fn main() {
    let number_list = vec![34,50,25,100,65];

    let result = largest(&number_list);
    println!("最大的数是 {}",result);

    let char_list = vec!['y','m','a','q'];
    let result = largest(&char_list);
    println!("最大字符是 {}",result);

    // 生命周期参数并不修任何传入后返回的值的生命周期，而是指出任何不遵守这个协议的传入值都将被
    // 借用检查器拒绝
    
    // 下面这段代码
    // 按正常理解，到这里string1，result应该是有效的。但由于生命周期参数告诉Rust的是
    // longest函数返回的引用的生命周期与传入参数的生命周期中较短的那个保持一致。
    // （'a所指代的是x和y生命周期相重叠的部分，而返回值生命周期应该与此重叠部分相同，
    // 即等于较短的那个）
    // 这里result的生命周期已经超过了string2因此无法通过检查
    // let string1 = String::from("long string is long");
    // let result;    
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }

    
    

}

// 返回值的生命周期必须与参数的生命周期有关联，下面这段代码因为没有这种关联，因此也无法编译
// fn longest1<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }
