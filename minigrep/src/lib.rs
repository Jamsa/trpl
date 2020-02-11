use std::fs;
use std::error::Error;
pub struct Config{
    pub query:String,
    pub filename:String,
}
impl Config{
    //返回Result类型
    pub fn new(args:&[String])->Result<Config, &'static str>{ //成功时返回Config对象，失败时返回&'static str,即字符串字面值类型
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

//从main中提取的逻辑
// trait 对象 Box<dyn Error>，函数会返回实现了Error的trait,dyn指动态类型
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    Ok(())
}
