use std::fmt;
use std::ops::Add;

// 1. trait type placeholder

pub trait Iterator{
  // a placeholder type
  type Item;

  fn next(&mut self) -> Option<Self::Item>; // use the placeholder type
}

#[derive(Debug)]
struct Counter{
}

impl Iterator for Counter{
  type Item = i32;              // set type for placeholder
  fn next(&mut self) -> Option<i32>{
   Some(1)
  }
}


// 2. default generic type parameters and operator overloading
#[derive(Debug,Copy,Clone,PartialEq)]
struct Point{
  x: i32,
  y: i32,
}

// Add<T=Self> here not specified the T, It will be Point
impl Add for Point{
  type Output = Point;

  fn add(self, other: Point) -> Point{
    Point{
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl fmt::Display for Point{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f,"Point(x:{},y:{})",self.x,self.y)
  }
}

// the type of T is not same as Self
#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

// T is Meters Self is Millimeters
impl Add<Meters> for Millimeters {
  type Output = Millimeters;

  fn add(self, other: Meters) -> Millimeters {
    Millimeters(self.0 + (other.0 * 1000))
  }
}


// 3. fully qualified syntax for same method name in diffrent trait
struct Human;
trait Pilot{
  fn fly(&self);
  fn name();
}
trait Wizard{
  fn fly(&self);
  fn name();
}
impl Pilot for Human{
  fn fly(&self){
    println!("fy in Pilot");
  }
  fn name(){
    println!("pilot name");
  }
}
impl Wizard for Human{
  fn fly(&self){
    println!("fly in Wizard");
  }
  fn name(){
    println!("wizard name");
  }
}
impl Human{
  fn fly(&self){
    println!("fly in Hunam");
  }
  fn name(){
    println!("human name");
  }
}


// 4. using supertrait to call method within another trait
trait OutlinePrint: fmt::Display {
  fn outline_print(&self) {
    // call Display.to_string. compile error, if Self not implement the Display trait
    let output = self.to_string();
    let len = output.len();
    println!("{}", "*".repeat(len + 4));
    println!("*{}*", " ".repeat(len + 2));
    println!("* {} *", output);
    println!("*{}*", " ".repeat(len + 2));
    println!("{}", "*".repeat(len + 4));
  }
}

impl OutlinePrint for Point{}


// 5. using Newtype pattern to implement trait on external type
// we are allowd to implement a trait on a type as long as either the trait or
// the type are local to our crate.
// we can wrap a external type and implement trait for it.
 // the wrapper type is elided at compile time, so there is no runtime performance
// penlty.
// implement Display for Vec<String>
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f, "[{}]",self.0.join(","))
  }
}


fn main() {
   // 1. trait type placeholder
  let mut ct = Counter{};
  println!("ct={}",ct.next().unwrap_or(0));


  // 2. default generic type paramters
  let p1 = Point{x:1,y:2};
  let p2 = Point{x:2,y:4};
  let p3 = p1 + p2;
  println!("p3 = {}",p3);       // by Display trait
  println!("p3 = {:?}", p3);    // by Debug trait

  let mi = Millimeters(10);
  let me = Meters(1);
  //let m = me + mi; compile error, because Add impl by Millimeters
  let m = mi + me;
  println!("m={:?}",m);

  // 3. fully qualified method name
  let person = Human{};
  // call Human.fly by default or compile error if Human.fly not define
  person.fly();
  // explicit syntax call
  Pilot::fly(&person);
  Wizard::fly(&person);
  Human::fly(&person);

  Human::name();
  <Human as Pilot>::name();

  // 4. using supertrait to call method within another trait
  p3.outline_print();

  // 5.  Newtype pattern
  println!("{}",Wrapper(vec![String::from("a"),
                             String::from("B"),
                             String::from("c")]));

}
