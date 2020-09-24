//共享状态并发
use std::sync::{Mutex, Arc};
use std::thread;
//use std::rc::Rc;

fn main() {


    //Mutex
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    //let counter = Mutex::new(0);
    //let counter = Rc::new(Mutex::new(0));
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        //如果直接使用循环外定义的不使用Rc包装的counter，会因为在一次循环中，counter的所有权被转移，而报编译错误
        //下面这行仍然会报错，因为Rc并不能在线程间安全地共享。因为Rc<T>并没有使用任何并发原语，来确保
        //改变计数的操作不会被其他线程打断。需要使用原子引用计数Arc<T>来代替。
        //let counter = Rc::clone(&counter);
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || { //所有权转移至线程
            println!("hi number {} from the spawned thread!", i);
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

//RefCell<T>/Rc<T>与Mutex<T>/Arc<T>的相似性
//因为counter是不可变的，但是可以通过获取其内部了值的可变引用；这意味着Mutex<T>提供了内部可变性，就像Cell系列那样。
//Rust也不能避免使用Mutex<T>的全部逻辑错误。类似Rc<T>的循环引用，Mutex<T>也有造成死锁的风险。这相当于一个操作需要锁住
//两个资源而两个线程各持一个锁，这会造成它们永远互相等待。
