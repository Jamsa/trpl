struct User {
    // 这里用的String而不是&str的slice类型，是因为要让User持有自身的完整所有权
    // 当使用结构体存储引用数据时需要用上 生命周期，生命周期 确保结构体引用的数据有效性跟结构体本身保持一致。将下面的String直接改为 &str而又不指定生命周期将会报错。这里不入 生命周期 概念，暂时用String。
    username: String,           
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email:String,username:String) -> User {
    // User{
    //     email:email,
    //     username:username,
    //     active:true,
    //     sign_in_count:1,
    // }

    //同名参数可以简写
    User{
        email,
        username,
        active:true,
        sign_in_count:1,
    }
}

//与元组类似的结构体被称为元组结构体
struct Color(i32,i32,i32);

fn main() {
    // 整个实例是mut的，rust不允许将特定字段标记为mut
    let mut user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someuserusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    user1.email = String::from("anotheremail@example.com");
    println!("{}",user1.email);
    let user1 = build_user(String::from("aaa"),String::from("bbb"));
    println!("{}",user1.email);


    // 结构体更新语法
    let user2 = User{
        email: String::from("bbb"),
        username: String::from("ddfsfsd"),
        ..user1
    };
    println!("{}",user2.username);

    // 没有命名字段的元组作为结构体
    let black = Color(0,0,0);
    println!("{}",black.0);

    // 没有任何字段的类单元结构体
    // 它类似于()，即unit类型，通常用于在某个类型上实现trait，但不需要在类型内存储数据

    // 结构体示例程序
    let rect1 = Rectangle{width:30,height:50};
    println!("The area of the rectangle is {} square pixels.",area(&rect1));

    // 使用Debug格式输出rect1
    println!("rect1 is {:?}",rect1);
    // pretty print
    println!("rect1 is {:#?}",rect1);
    
    // 添加方法
    // 关于运算符(-> 或 .)，rust的自动引用和解引用功能在方法调用时会起作用（方法调用是少数启用了这种行为的地方）
    println!("Get area by method，rect1.area()={}",rect1.area());

    let rect2 = Rectangle{ width: 10,height: 40};
    println!("Can rect1 hold rect2? {}",rect1.can_hold(&rect2));

    // 关联函数
    let sq = Rectangle::square(3);
    println!("Square: {:#?}",sq); //{:?}是另一种输出格式
}

#[derive(Debug)] //派生Debug trait，以支持打印出调试信息
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// 可以有多个impl块
impl Rectangle {
    // 不带self参数的方法被称为“关联函数”。它仍与结构体关联，但不与实例关联
    fn square(size: u32) -> Rectangle {
        Rectangle{width: size,height:size}
    }
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}

//C++中的->运算符
//rust中没有，他通过自动引用和解引用功能来实现。
//当使用object.something()调用方法时，会自动为object添加&、&mut或*以便使object与方法签名匹配。
