//解决单值有多个所有者，即多所有权的问题

use std::rc::Rc;
use crate::List::{BoxCons,Nil,RcCons};

fn main() {
    //双头的con list
    let a = BoxCons(5,
                 Box::new(BoxCons(10,
                               Box::new(Nil))));
    let _b = BoxCons(3, Box::new(a));
    //b和c都想得到a的所有权，会无法编译
    //let c = BoxCons(4, Box::new(a));

    let a = Rc::new(RcCons(5,
                   Rc::new(RcCons(10,
                                  Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    //每次clone时，Rc中的数据引用计数都会增加，直到有零个引用之前其数据都不会被清理
    //这里也可以调用a.clone()。Rc::clone的实现并不像大部分类型的clone实现那样进行深拷贝。它只会增加引用计数。
    let _b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    let _c = RcCons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
    {
        let _c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    //离开作用域后引用计数会减1
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

//双头con list
enum List{
    BoxCons(i32, Box<List>),
    Nil,
    RcCons(i32, Rc<List>),
}


