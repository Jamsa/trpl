// macros are a way of writing code that writes other code, which is known as metaprogramming.
// macros are expanded before the compiler interpreters the meaning of the code
// the macro definitions are more complex than function definitions


// 1. `declarative` and `procedural` macros:
// - `declarative` macros with `macro_rules`
// - three kinds of `procedural` macros
//   - #[derive] macros used on structs and enums
//   - attribute like macros that define custom attributes usable on any item
//   - function like macros that look like function calls but operate on the token specified as their argument

// 2. a `declarative` macro [`simple_vec`]
#[macro_export]
macro_rules! simple_vec {
  ( $( $x:expr ),* ) => {
    {
      let mut temp_vec = Vec::new();   
      $(
        temp_vec.push($x);
      )*
        temp_vec
    }
  };
}


// 3. write a custom `derive` macro
// define hello_macro trait in `hello_macro` crate
// define hello_macro_derive macro implement in `hello_macro_derive` crate
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// use macro
#[derive(HelloMacro)]
struct Pancakes;

// the HelloMacro generate the code like the following code
// impl HelloMacro for Pancakes {
//   fn hello_macro() {
//     println!("Hello, Macro! My name is Pancakes!");
//   }
// }

// 4. `attribute-like` macros
// Attribute-like macros similar to derive macros, but more flexible.
// `derive` only works for structs and enums, attributes can be applied on other items, such as functions. 
// #[route(GET,"/")]
// fn index

// 5. `function-like` macros
// `function-like` macros looks like function calls.
// Similarly to `macro_rules!`.
// `macro_rules` macros can be defined only using match-like syntax.
// let sql = sql!(SELECT * FROM posts WHERE id=1);




fn main() {
  // 2. the simple_vec macro
  let mut v = simple_vec!(1,2,3);
  v.push(4);
  println!("{:?}",v);

  // 3. `derive` macro
  Pancakes::hello_macro();


}

