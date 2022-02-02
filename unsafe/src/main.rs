unsafe fn dangerous(){
  println!("in dangerous fn")
}

fn use_std_unsafe_split(){
  let mut v = vec![1,2,3,4,5,6];
  let r = &mut v;
  //let r = &mut v[..];
  // split_at_mut can not be implemented by safe code, because it need return two mut ref from a vec. it'll not be permitted by the compiler
  let (a, b) = r.split_at_mut(3);
  assert_eq!(a, &mut[1,2,3]);
  assert_eq!(b, &mut[4,5,6]);
}



fn main(){
  let mut num = 5;
  // can not define raw point by & and &mut because rust does permit create ref and mut ref for a var at the same time
  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;
  unsafe{
    // deref raw point
    println!("r1 is: {}", *r1);
    *r2 = 100;
    println!("r2 is: {}", *r2);
    println!("r1 is: {}", *r1);
    // call unsafe func in unsafe block
    dangerous();
  }
  use_std_unsafe_split();
}
