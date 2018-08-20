// Map不常用，没有被prelude
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    // 使用vector构建map
    let teams = vec![String::from("Blue"),String::from("Yellow")];
    let initial_scores = vec![10,50];

    let mut scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name,field_value);
    // 这里field_name和field_value不再有效
    //println!("{}",field_name);
    let score = scores.get(&String::from("Blue"));
    println!("{:?}",score);     // 返回的是Option

    for(key,value) in &scores{
        println!("{}: {}",key,value);
    }

    // 没有值时才插入
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // entry方法返回的是枚举Entry，它代表可能存在也可能不存在的值
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    
    println!("{:?}",scores);
}
