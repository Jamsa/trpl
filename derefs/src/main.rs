
//实现Deref trait允许我们重载解引用运算符 * 。
//通过这种方式实现Deref trait的智能指针可以被当作常规引用来对待，可以编写操作引用的代码并用于智能指针。

use core::ops::Deref;

fn main() {
    //常规引用和解引用
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    //解引用了y就可以访问y所指向的整型值，如果不解引用，则不能直接比较数字和引用，因为它们是不同的类型
    assert_eq!(5, *y);

    let y = Box::new(x);
    //可以对box解引用，追踪它的值
    assert_eq!(5, *y);

    let y = MyBox::new(x);
    assert_eq!(5, *y); //这里的*y，相当于*(y.deref())

    //解引用强制多态。Rust通过"将实现了Deref的类型的引用"转换为"原始类型通过Deref所能够转换的类型的引用"(从一种引用转成另一种引用)。
    //当这种特定类型的引用作为实参传递给和形参类型不同的函数或方法时，解引用强制多态将自动发生。这时会
    //有一系列的deref方法被调用，把我们提供的类型转换成参数所需的类型。
    let m = MyBox::new(String::from("Rust"));
    //这是能调用就是因为强制解引用多态。MyBox<T>实现了Deref trait，Rust可通过deref调用将&MyBox<String>
    //变成&String。标准库中提供了String上的Deref，它会返回字符串slice。Rust再次调用deref将&String变成
    //&str，就符合了hello函数的定义。
    hello(&m);
    //如果没有解引用多态，就得写成
    hello(&(*m)[..]);

    //类似于Deref trait重载不可变引用的*运算符，Rust提供了DerefMut trait用于重载可变引用的*运算符。
    //Rust在发现类型和trait实现满足三种情况时会进行解引用强制多态。
    // - 当T:Deref<Target=U>时从&T转成&U
    // - 当T:DerefMut<Target=U>时从&mut T转成&mut U
    // - 当T:Deref<Target=U>时从&mutT转成&U，反之是不能的，不允许从不可变引用转为可变引用
}

//自定义智能指针，它与Box不同的是，它无法控制数据存储在堆上
struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

//定义MyBox的Deref trait
impl<T> Deref for MyBox<T>{
    type Target = T; //定义了用于此trait的关联类型（稍有不同的定义泛型参数的方式）
    fn deref(&self) -> &T {
        &self.0 //返回了希望通过*运算符访问的值的引用
    }
}

//解引用强制多态
fn hello(name: &str){
    println!("hello, {}!", name);
}
