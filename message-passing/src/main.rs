use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    //创建新的通道
    //mpsc是多生产者，单消费者的缩写
    //返回值是tx和rx，分别为发送者和接收者
    let (tx, rx) = mpsc::channel();

    thread::spawn(move ||{
        let val = String::from("hi") ;
        //发送
        tx.send(val).unwrap();

        //通过channel发送后，不能再使用变量val
        //println!("val is {}", val);
    });

    //接收
    //recv会阻塞主线程执行直到从通道中接收一个值
    //当通道发送端关闭时，recv会返回一个错误表明不会再有新的值到来
    //还有try_recv不会阻塞，它立刻返回一个Result<T, E>，OK包含了可用信息，Err代表此时没有任何消息
    let received = rx.recv().unwrap();
    println!("Got: {}", received);


    let (tx, rx) = mpsc::channel();
    //第二个发送者
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //多发送者
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
