use std::env;
use std::fs;
use std::process;


struct Config{
    query:String,
    filename:String,
}
impl Config{
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
            process::exit(1);
        }
    );

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
