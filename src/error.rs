use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;

// 错误处理
// 错误分为两类：可恢复和不可恢复

fn main() {
    // 不可恢复，出现panic!时，程序默认会unwinding，即回溯栈并清理它遇到的每一个数据。
    // 如果在Cargo.toml的 [profile] 中增加 panic = 'abort，则不清理数据，由操作系统来清理。
    // panic!("crash and burn");

    // 设置 RUST_BACKTRACE环境变量可得到完整错误栈，为得到backtrace信息需要使用debug标识。
    // 不使用--release运行cargo build或cargo run时默认会开启debug。

    // 可恢复错误使用Result类型作为返回值。Result是枚举类型。
    let f = File::open("hello.txt");
    println!("{:?}",f);         // hello.txt不存在时，返回的是Err

    let f = match f {
        Ok(file) => 123,//file,
        Err(error) =>{
            456//panic!("文件无法打开:{:?}",error);
        },
    };

    println!("{}",f);

    // 匹配不同类型的错误
    let f = File::open("hello.txt");
    println!("{:?}",f);

    let f = match f {
        Ok(file) => 123,//file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            456//panic!("文件无法打开:{:?}",error);
        },
        Err(error) => {
            panic!("文件:{:?},打开出错",error)
        },
    };

    println!("{}",f);

    // 失败时panic的简写：unwrap和expect
    // File::open("hello.txt").unwrap();如果为Err则直接panic
    // File::open("hello.txt").expect("自定义错误信息");如果为Err则panic，传递自定义信息，便于定位错误

    // 错误传播，类似于异常继续抛出
    let uname = read_username_from_file();
    println!("{:?}",uname);
    // 需要注意?只能用于返回Result的函数（出错时Rust自动将它错误包装为Result类型）

    // panic! 还是不panic!
    // 在示例、原型、测试代码中使用panic
}


fn read_username_from_file() -> Result<String,io::Error>{
    let mut f = File::open("hello.txt")?; // 传播错误
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
