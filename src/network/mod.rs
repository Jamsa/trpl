pub fn connect(){
    println!("network::connect()");
}

// 在这里添加 mod server;，并将下面的内容移入server.rs是不行的。
// lib.rs中添加类似代码与其它文件是不同的。
// mod server {
//     fn connect(){
//         println!("network::server::connect");
//     }
// }


pub mod server;
