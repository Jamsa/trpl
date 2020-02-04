// trait 的使用
use std::cmp::PartialOrd;
use std::fmt::Display;

pub trait Summarizable {
    fn summary(&self) -> String;
    // 也可以定义成默认实现
    fn summarize(&self) -> String {
         format!("(Read more... {})",self.summary())
     }
}

//pub trait Display{}

pub trait Debug{}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//为类型实现trait
impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline,self.author,self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//trait作为参数，作为参数的trait需要是pub的
pub fn notify(item: impl Summarizable) {
    println!("Breaking news! {}", item.summarize());
}

//trait Bound语法，上面的函数可写成
pub fn notify1<T:Summarizable>(item:T){
    println!("Breaking news! {}", item.summarize());
}

//多个trait bound
pub fn notify2<T:Summarizable+Display>(item:T){
    println!("Breaking news! {}", item.summarize());
}

//通过where简化trait bound
fn some_function<T:Display + Clone, U:Clone+Debug>(t:T,u:U)->i32{
    1
}

fn some_function1<T,U>(t:T,u:U)->i32 
where T:Display + Clone,
U:Clone + Debug{
    1
}

//返回trait
fn returns_summarizable() -> impl Summarizable {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
//下面的写法是无法正常编译的，这种方式只支持返回一种类型
/*fn returns_summarizable1(switch: bool) -> impl Summarizable {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}*/
// 有条件的实现方法
struct Pair<T>{
    x: T,
    y: T,
}

// 只有在泛型参数T实现了Display和PartialOrd的Pair才实现cm_display方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x)
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }
}

// PartialOrd为trait bounds，PartialOrd定义了比较大小的运算符
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];  // 需要Copy trait才能复制值

    for &item in list.iter(){
        if item > largest {     // 需要 PartialOrd　trait 才能比较大小
            largest = item;
        }
    }

    largest
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summary());
    println!("1 new tweet: {}", tweet.summarize());

    let tweet = returns_summarizable();
    println!("{}",tweet.summary());

    //let tweet = returns_summarizable1(true);
    //println!("{}",tweet.summary());


    let number_list = vec![34,50,25,100,65];

    let result = largest(&number_list);
    println!("最大的数是 {}",result);

    let char_list = vec!['y','m','a','q'];
    let result = largest(&char_list);
    println!("最大字符是 {}",result);
    

    notify(tweet);
}
