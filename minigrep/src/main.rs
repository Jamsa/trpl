use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

fn main() {
    let args:Vec<String> = env::args().collect();
    //let query = &args[1];
    //let filename = &args[2];
    let config = Config::new(&args).unwrap_or_else(|err|{ //一个匿名函数
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);//出错时退出
        }
    );

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);//输出至标准错误输出
        process::exit(1);
    }
}

