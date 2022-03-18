use std::fmt;
use std::io::Error;

// 1. using Newtype pattern for type safety and abstraction


// 2. create type synonyms with type alias
type Kilometers = i32;

// reducing repetition
type Thunk = Box<dyn Fn() + Send + 'static>;
fn takes_long_type(f: Thunk) {
  f();
}
fn returns_long_type() -> Thunk {
  return Box::new(|| println!("hello"))
}

// reducing repetition
pub trait Write1 {
  fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
  fn flush(&mut self) -> Result<(), Error>;

  fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
  fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

type WResult<T> = std::result::Result<T, std::io::Error>;
pub trait Write2 {
  fn write(&mut self, buf: &[u8]) -> WResult<usize>;
  fn flush(&mut self) -> WResult<()>;

  fn write_all(&mut self, buf: &[u8]) -> WResult<()>;
  fn write_fmt(&mut self, fmt: fmt::Arguments) -> WResult<()>;
}

// 3. nerver type means empty type
fn _bar() -> ! {
  panic!("hello")
}

// 4. Dynamically Sized Types (DST)
// `str` is DST, `&str` not DST
// the size of `str` is dynamic, we can not know how long it is until runtime.
// we can know the size of `&str`, `&str` is two values: the address of `str` and its length.
// we can combile str with all point type, `Box<str>` or `RC<str>`
// same with the trait: `&dyn Trait`, `Box<dyn Trait>` or `RC<dyn Trait>`.
// `dyn` prefix is object prefix. `dyn Trait` like `str` it's a value. `Trait` like `&str` is a reference.
// Rust has Sized type, which is implicitly added to every generic function
fn _f1<T>(_t: T){

}
// same as
fn _f2<T: Sized>(_t: T){
}

fn main() {
  // 1.
  let x: i32 = 30;
  let k: Kilometers = 2;
  println!("{} kilometers",k+x);

  // 2.
  let f: Thunk = Box::new(|| println!("hi"));
  takes_long_type(f);
  let f = returns_long_type();
  f();

  // 4. str is DST, the next line can not compile
  //let _s:str = "hello";
}
