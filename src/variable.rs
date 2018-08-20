fn main() {
    //println!("Hello, world!");
    let x = 5;
    println!("The value of x is: {}",x);
    // 变量不可变，不允许重新赋值
    //x = 6;
    //println!("The value of x is: {}",x);

    //可变变量
    let mut x1 = 5;
    println!("The value of mut x1 is: {}",x1);
    x1 = 6;
    println!("The value of mut x1 is: {}",x1);

    // 常量
    const MAX_POINT: u32 = 100_000;
    // const mut MAX_POINT1: u32 = 100_000;  mut 不能用于 const
    println!("The value of const MAX_POINT is: {}",MAX_POINT);

    // 变量隐藏
    let x2 = 5;
    let x2 = x2 + 1;
    let x2 = x2 * 2;
    println!("The value of shadowing x2 is: {}",x2);

    // mut 与 隐藏的区别
    // 1. 不使用 let 就无法对变量重新赋值
    // 2. 使用 let 时创建的是一个新的变量，可以让同名变量有不同类型的值

    let spaces = "   ";
    let spaces = spaces.len();
    // 用 spaces = spaces.len() 编译出错，因为变量的类型是不能改变的
    println!("space的值是数字：{}",spaces);

    // 变量类型
    // 两类：标量和复合
    // let guess  = "42".parse().expect("Not a number"); 不明确定义变量类型会导致类型推断错误
    let guess: u32  = "42".parse().expect("Not a number");
    println!("guess的值是：{}",guess);

    // 标量的4种基本类型：整型、浮点、布尔、字符
    let heart_eyed_cat = '😻';  // 这是一只有红星眼睛的猫
    let z = 'ℤ';
    println!("Char支持Unicode值：{},{}",heart_eyed_cat,z);

    // 复合类型
    // tuple
    let tup: (i32,f64,u8) = (500,6.4,1);
    let (x,y,z) = tup;
    println!("元组tup的各个值是：({},{},{})",x,y,z);
    //println!("元组tup是：{}",tup);无法输出
    println!("使用.符号访问元组的元素，如 tup.0={}",tup.0);

    // 数组
    let a = [1,2,3,4,5];

    // array是固定长度的，不固定的可以用vector。array在栈上分配而不是在堆上
    println!("数组a[0]={}",a[0]);

    // 越界访问数组会导致运行时的panic
    let index = 10;
    println!("测试访问数组中不存在的元素:a[{}]",index);
    //let e = a[index];           // 直接写成a[10]在编译期能检查出来
    //println!("访问不存在的数组元素:a[{}]={}",index,e);

}
