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

fn main() {
    let number_list = vec![34,50,25,100,65];

    let result = largest(&number_list);
    println!("最大的数是 {}",result);

    let char_list = vec!['y','m','a','q'];
    let result = largest(&char_list);
    println!("最大字符是 {}",result);
}
