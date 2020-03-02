fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff")};
    // c.drop(); 并不能直接调用drop
    // 但是可以直接调用std::mem::drop
    drop(c);
    let _d = CustomSmartPointer { data: String::from("other stuff")};
    println!("CustomSmartPointer created.");
} 

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    //放置实例离开作用域时希望执行的代码
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data`{}`", self.data);
    }
}
