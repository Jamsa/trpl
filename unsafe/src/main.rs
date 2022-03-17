extern "C" {
  fn abs(input: i32) -> i32;
}

unsafe fn dangerous(){
  // call external abs() function in unsafe function, 
  println!("abs(-10)={}",abs(-10));
  println!("in dangerous fn")
}

fn use_std_unsafe_split() -> (Vec<i32>, Vec<i32>){
  //let mut v = vec![1,2,3,4,5,6];
  //let r = &mut v;
  let r = &mut vec![1,2,3,4,5,6];
  //let r = &mut v[..];
  // split_at_mut can not be implemented by safe code, because it need return two mut ref from a vec. it'll not be permitted by the compiler
  let (a, b) = r.split_at_mut(3);
  assert_eq!(a, &mut[1,2,3]);
  assert_eq!(b, &mut[4,5,6]);
  //(a.to_owned(),b.to_owned())
  //let _c = a.to_owned();
  a[0] = 10;
  b[0] = 20;
  //println!("r[0]={}",r[0]);
  let mut ccc = a.to_owned();
  println!("ccc[0]={}",ccc[0]);
  ccc[0] = 10000;
  println!("a[0]={}",a[0]);
  (a.to_owned(), b.to_owned())
}

#[no_mangle]
pub extern "C" fn call_from_c() {
  println!("Just called a Rust function from C!");
}

static HELLO_WORLD: &str = "Hello, world!";
// static var can be mutable
static mut COUNTER: i32 = 0;

fn main(){
  let mut num = 5;
  // can not define raw point by & and &mut because rust does permit create ref and mut ref for a var at the same time
  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;
  unsafe{
    // deref raw point
    println!("r1 is: {}", *r1);
    *r2 += 200;
    println!("r2 is: {}", *r2);
    println!("r1 is: {}", *r1);
    // call unsafe func in unsafe block
    dangerous();

    // call external func in unsafe block
    println!("abs(-20)={}",abs(-20));
  }
  println!("num is {}", num);
  let (mut a,mut b) = use_std_unsafe_split();
  println!("a[0]={}",a[0]);
  println!("b[0]={}",b[0]);
  a[0] = 100;
  b[0] = 1000;
  println!("a[0]={}",a[0]);
  println!("b[0]={}",b[0]);


  let r = &mut vec![1,2,3,4,5,6];
  // unsafe function call here
  let (a, b) = r.split_at_mut(3);
  //change the first element, it's also the first element of r
  a[0] = 100;
  b[0] = 1000;
  // r[0] changed by set value to a[0]
  println!("r[0]={}",r[0]);

  println!("static global variable HELLO_WORLD={}",HELLO_WORLD);

  // mutable data global access is considered unsafe
  unsafe{
    COUNTER = 100;
    println!("static global mutable variable COUNTER={}",COUNTER);
  }

}

// A trait is unsafe when at least one of its methods has some invarint that the compiler can't verify
unsafe trait Foo{
}

unsafe impl Foo for i32{
}

// accessing fields of a union
