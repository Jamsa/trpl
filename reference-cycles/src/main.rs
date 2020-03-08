//Rust的内存安全保证使其难以意外地制造永远也不会被清理的内在，但并不是不可能。
//与在编译时拒绝数据竞争不同，Rust并不保证完全地避免内存泄露，这意味着内存泄露在Rust被认为是内存安全的。
//这一点可通过Rc<T>和RefCell<T>看出：创建引用循环的可能性是存在的，这会造成内在泄露，因为每一项的引用计数
//永远也到不了0，其值也永远不会被丢弃。

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::List::{Cons, Nil};
//use crate::Node;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    //用于访问List的第二项
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}


//树型结节
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing b = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    //下面的行取消注释后将出现运行时栈溢出，因为它是一个环状
    //println!("a next item = {:?}", a.tail());

    //如果有包含Rc<T>的RefCell<T>值或类似的嵌套结合了内部可变性和引用计数的类型，需要小心不要形成引用循环；
    //你无法指望Rust帮你捕获它。
    //创建引用循环是程序上的逻辑Bug，应该使用自动化测试、代码评审和其他开发实践来使其最小化。
    //另一个解决方案是重新组织数据结构，使得一部分引用拥有所有权而另一部分没有。即，循环将由一些拥有所有权的系
    //和一些无所有权的关系组成，只有所有权关系才能影响值是否可以被丢弃。

    //可以将Rc<T>变为Weak<T>来避免循环
    //上面我们使用了strong_count来获取Rc<T>，也可以调用Rc:downgrade来获取弱引用。
    //调用Rc::downgrade时会得到Weak<T>，它会将weak_count加一。weak_count无需计数为0就能使Rc实例被清理。
    //因为Weak<T>引用的值可能已经被丢弃，为了使用Weak<T>所指向的值，我们可以调用Weak<T>实例的upgrade方法，
    //这会返回Option<Rc<T>>。如果Rc<T>值还未被丢弃，则结果是Some，否则为None。

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),//父节点的弱引用
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
{
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),//children包含了leaf节点
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);//leaf的parent指定为branch，这样形成了环
    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
    );
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());//这里不会溢出
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );


}
