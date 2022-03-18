use std::fmt;

// 1. function pointer
fn add_one(x: i32) -> i32 {
  x + 1
}

fn do_twice(f: fn(i32) -> i32, x: i32) -> i32 {
  f(x)+f(x)
}
// `fn` is a type rather than a trait, so we specify `fn` as the parameter type directly
//
// rather than declaring a generic type parameter and one of `Fn` trait as a trait bound.
// function point implement all three of the closure traits(`Fn`,`FnMut` and `FnOnce`), so
// you can always pass a function pointer as argument for a function that expects a closure.
//
// it's best to write function using trait so the function can accept either functions or closures.



// 2. exploits detail of tuple structs and tuple struct enum variant.
#[derive(Debug)]
enum Status {
  Value(u32),
  Stop,
}

impl fmt::Display for Status {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}


// 3. returing closures

fn main() {
  let answer = do_twice(add_one, 1);
  println!("answer is {}",answer);

  // 1. function pointer or closure trait
  let list_of_numbers: Vec<i32> = vec![1, 2, 3];
  // closure
  let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
  println!("[{}]",list_of_strings.join(","));
  // function
  let list_of_strings:Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
  println!("[{}]",list_of_strings.join(","));

  // 2.
  let x = Status::Stop;
  println!("{}",x);
  // init a series of Value
  let list_of_statuses: Vec<Status> = (0u32..10).map(Status::Value).collect();
  println!("[{}]",list_of_statuses.iter().map(|i| i.to_string()).collect::<String>());

  // 3.
  

}
