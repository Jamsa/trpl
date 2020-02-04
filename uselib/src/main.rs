mod client;
mod network;

use client::connect;

//use network::connect;

fn main() {
    connect();
    network::connect();
    network::server::connect();
    // 从根模块开始引用
    crate::client::connect();
}
