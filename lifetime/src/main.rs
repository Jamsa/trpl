//生命周期注解并不改变任何引用的生命周期的长短。
//与当函数签名中指定了泛型类型参数后就可以接受任何类型一样，
//当指定了泛型生命周期后函数也能接受任何生命周期的引用。
//生命周期注解描述了多个引用生命周期相互的关系，而不影响其生命周期。



// 生命周期参数，声明了生命周期'a，
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 返回值的生命周期必须与参数的生命周期有关联，下面这段代码因为没有这种关联(没有参数与'a有关联)，因此也无法编译
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

//结构体生命周期注解
struct ImportantExcerpt<'a> {
    part: &'a str,
}

//生命周期省略
//被编码进rust引用分析的模式被称为生命周期省略
//https://kaisery.github.io/trpl-zh-cn/ch10-03-lifetime-syntax.html

fn main() {
    println!("{}",longest("aaa", "bbbb"));
    /*{
        let r;
        //println!("r: {}",r);不允许使用未初始化的变量
        {
            let x = 5;
            r = &x;
        }//这里x被drop，但是仍然需要借用，因此编译报错
        println!("r: {}",r); //需要借用到这里
    }*/

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

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
