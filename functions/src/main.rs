fn main(){
    // 函数
    another_function(3);

    // 语句和表达式，语句不返回值，表达式产生值
    // let是语句，因此不能把let再赋值给另一个变量
    // let x = (let y = 6);

    // 函数调用、宏调用、大括号代码块都是表达式
    let y = {
        let x = 3;
        x + 1 //这行末没有分号，如果加了它就变成了语句，返回值会变成空元组，而不再是数字。rust中表达式不包含结尾的分号
    };
    println!("y的值为:{}",y);

    // 函数返回值
    let x = five();
    println!("函数five的返回值是:{}",x);
}

fn another_function(x: i32){
    println!("进入函数another_funcation，传递的参数x的值为:{}",x)
}

fn five() -> i32 {
    5 //这里末尾也没有加分号
}
