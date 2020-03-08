//多线程

use std::thread;
use std::time::Duration;

fn main() {
    //如果没有join线程并不能输出完，主线程结束时程序退出
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    //通过move将所有权移转移至线程内
    let v = vec![1,2,3];
    let handle = thread::spawn(move||{
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
