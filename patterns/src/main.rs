fn let_demo(){
    println!("===========let demo===========");
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color{
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_demo(){
    println!("=========while let demo==========");
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop(){
        println!("pop value {}", top);
    }
}

fn for_demo(){
    println!("=========for demo==========");
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn func_args_demo(&(x,y): &(i32,i32)){
    println!("=========func args demo==========");
    println!("参数为:({},{})",x,y);
}

fn pattern_syntax_demo(){
    println!("=========pattern syntax demo==========");
    let x = 8;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    //多模式匹配
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

struct Point {
    x: i32,
    y: i32,
}
fn struct_demo() {
    println!("=========struct demo==========");
    let p = Point { x: 0, y: 7 };

    //使用不同的变量名
    let Point { x: a, y: b } = p;
    println!("Point.x = {}, Point.y = {}",a,b);

    let Point { x, y } = p;
    println!("Point.x = {}, Point.y = {}",x,y);

    //匹配结构成员
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enum_demo() {
    println!("=========enum demo==========");
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }
}


fn struct_ref_demo() {
    println!("=========struct_ref demo==========");
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points
        .iter()
        // 解构引用，低版本rust的只能使用 &Point，因为iter()的返回值的迭代器的元素类型为&Point
        //.map(|Point { x, y }| x * x + y * y)
        .map(|&Point { x, y }| x * x + y * y)
        .sum();
    println!("sum of squares:{}",sum_of_squares);
}

// 复杂组合
fn compose_demo(){
    println!("=========compose demo==========");
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet:{},inches:{},point.x:{},point.y:{}",feet,inches,x,y)
}

// 使用 _ 忽略部分值
fn ignore_demo(_: i32,y: i32) {
    println!("=========ignore demo==========");
    println!("y={}",y);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    let _x = 5;// 未使用变量
    let y = 10; // 编译时会产生未使用警告



    // 使用 ... 忽略值
    struct Point3 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3 { x: 0, y: 0, z: 0 };

    match origin {
        Point3 { x, .. } => println!("x is {}", x),
    }
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
}

fn main() {
    let_demo();
    while_demo();
    for_demo();
    func_args_demo(&(1,2));
    pattern_syntax_demo();
    struct_demo();
    enum_demo();
    struct_ref_demo();
    compose_demo();
    ignore_demo(1,2);
}
