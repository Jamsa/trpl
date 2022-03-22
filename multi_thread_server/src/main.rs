use multi_thread_server::ThreadPool;
use std::thread;
use std::time::Duration;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::io::prelude::*;



fn handle_connection(mut stream: TcpStream){
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";
  let sleep = b"GET /sleep HTTP/1.1\r\n";

  let (status_line, filename) = if buffer.starts_with(get){
    ("HTTP/1.1 200 OK", "hello.html")
  } else if buffer.starts_with(sleep){
    thread::sleep(Duration::from_secs(5));
    ("HTTP/1.1 200 OK", "sleep.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND", "404.html")
  };

  let contents = fs::read_to_string(filename).unwrap();

  let response = format!(
    "{}\r\nContent-Length: {}\r\n\r\n{}",
    status_line,
    contents.len(),
    contents
  );

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let pool = ThreadPool::new(4);

  for stream in listener.incoming()// .take(2)
  {
    let stream = stream.unwrap();
    pool.execute(||{
      println!("Connection established!");
      handle_connection(stream);
    });
  }

}
