// macros are a way of writing code that writes other code, which is known as metaprogramming.
// macros are expanded before the compiler interpreters the meaning of the code

// 1. three kinds of procedural macros
// #[derive] macros used on structs and enums
// attribute like macros that define custom attributes usable on any item
// function like macros that look like function calls but operate on the token specified as their argument
// 

fn main() {
    println!("Hello, world!");
}

