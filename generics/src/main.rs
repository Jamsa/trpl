// 泛型、trait与生命周期

//没有泛型的大小比较
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//带泛型的版本
/*fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest { //这里会编译无法通过
            largest = item;
        }
    }

    largest
}*/

//结构体中的泛型
struct Point<T> {
    x: T,
    y: T,
}

//枚举定义中的泛型
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

//方法定义中的泛型
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


// ========================================================

use std::fmt::Display;

struct Pair<T>{
    x: T,
    y: T,
}

// 对所有泛型参数类型T的Pair都实现了new方法
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

// 只对f32类型定义distance_from_origin方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


// ====================

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    /*let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);*/

    // 泛型的性能
    // Rust通过在编译时进行泛型代码的单态化来保证效率。
    // 单态化是一个通过填充编译时使用的具体类型，将通用代码转为特定代码的过程
}


