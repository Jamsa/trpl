//内部可变性是Rust中的一个设计模式，它允许你只有不可变引用时也可以改变数据，这通常是借用规则所不允许的。
//为了改变数据，该模式在数据结构中使用unsafe代码来模糊Rust通常的可变性和借用规则。
//RefCell<T>代表其数据的唯一所有权。
//类似Rc<T>，RefCell<T>只能用于单线程场景。
//Box<T>、Rc<T>和RefCell<T>的选择：
// - Rc<T>允许相同数据有多个所有者；Box<T>和RefCell<T>有单一所有者
// - Box<T>允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T>允许在运行时执行不可变或可变借用检查。
// - 因为RefCell<T>允许在运行时执行不可变或可变借用检查，所以我们可以在即便RefCell<T>自身是不可变的情况下修改其内部的值。


//结合Rc<T>和RefCell<T>来拥有多个可变数据所有者
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;


fn main() {
    let x = 5;
    //let y = &mut x;无法获取不可变值的可变引用


    //结合Rc<T>和RefCell<T>来拥有多个可变数据所有者
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)),  Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    //标准库中的Cell<T>，类似RefCell<T>，除了提供内部值的引用，它的值还会被拷贝进和拷贝出Cell<T>。
    //Mutex<T>则提供线程间安全的内部可变性。
}
