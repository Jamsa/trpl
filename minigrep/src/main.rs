use std::env;
use std::fs;
use std::process;
use std::error::Error;


struct Config{
    query:String,
    filename:String,
}
impl Config{
    //返回Result类型
    fn new(args:&[String])->Result<Config, &'static str>{ //成功时返回Config对象，失败时返回&'static str,即字符串字面值类型
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}


fn main() {
    let args:Vec<String> = env::args().collect();
    //let query = &args[1];
    //let filename = &args[2];
    let config = Config::new(&args).unwrap_or_else(|err|{ //一个匿名函数
            println!("Problem parsing arguments: {}", err);
            process::exit(1);//出错时退出
        }
    );

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

//从main中提取的逻辑
// trait 对象 Box<dyn Error>，函数会返回实现了Error的trait,dyn指动态类型
fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    Ok(())
}
