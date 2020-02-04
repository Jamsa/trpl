#[derive(Debug)]
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // vector 只能存储相同的类型，在内存中相邻排列
    let v:Vec<i32> = Vec::new();
    println!("{:?}",v);
    let v = vec![1,2,3]; //vec!宏
    println!("{:?}",v);

    let second1: &i32 = &v[2];            // 使用不存在的索引时会panic!
    let second2: Option<&i32> = v.get(2); // get不存在的元素时返回 None
    println!("{},{:?}",second1,second2);

    // 无效引用，
    let mut v = vec![1,2,3,4,5];
    v[2] = 9;
    for i in &v{
        println!("{}",i);
    }
    // 获取元素不可变引用后，将无法向vec中添加元素
    // 因为vec内部实现中，在push元素时，如内存不足，有可能会移动其它元素，包括头部元素
    //let first = &v[0];          
    //v.push(6);

    let mut v = vec![1,2,3,4,5];
    for i in &mut v{
        *i += 50;
    }
    
    for i in &v{
        println!("{}",i);
    }
    
    // vec中只能存储一种类型的数据。如果要存储不同类型的数据，可以存储枚举元素
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row{
        println!("{:?}",i);
    }
        
}

