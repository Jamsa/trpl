#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection{
        AveragedCollection{
            list: [].to_vec(),
            average: 0 as f64,
        }
    }

    pub fn add(&mut self, value:i32){
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32>{
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self){
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


//trait是可以有默认实现的，可以代替继承

pub trait Draw {
    fn draw(&self);
}

/*pub struct Screen<T: Draw>{
    pub components: Vec<Box<T>>,
}

impl<T> Screen<T> where T: Draw{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}*/

pub struct Screen{
    pub components: Vec<Box<dyn Draw>>, //dyn泛型的动态分发，因为在编译时无法确定泛型参数的类型
}

impl Screen{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button{
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button{
    fn draw(&self){
        println!("draw Button: {:?}", self)
    }
}

#[derive(Debug)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self){
        println!("draw SelectBox: {:?}", self)
    }
}
