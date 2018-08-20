fn main(){
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    }else if number % 3 == 0 {
        println!("number is divisible by 3");
    }else if number % 2 == 0 {
        println!("number is divisible by 2");
    }else{
        println!("number is not divisible by 4, 3 or 2");
    }

    // if是表达式，可以将if 的返回值赋给变量
    let condition = true;
    let number = if condition {
        5
    }else{
        6 //改为"six"会出现编译错误，因为类型不匹配
    };

    println!("The value of number is: {}",number);

    // 循环有3种类型：loop、while和for

    let mut count = 0;
    loop{
        if count >= 10 {break;}
        println!("loop count {}",count);
        count = count + 1;
    }

    let mut number = 3;
    while number != 0 {
        println!("while loop count: {}",number);
        number = number - 1;
    }

    let a = [10,20,30,40,50];

    for element in a.iter() {
        println!("the element in for loop is: {}",element);
    }

    for number in (1..4).rev() {
        println!("range number:{}",number);
    }
}
