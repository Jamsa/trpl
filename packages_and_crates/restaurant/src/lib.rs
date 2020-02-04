#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//餐厅前台模块
mod front_of_house{
    pub mod hosting{ //不加pub将无法被外部访问
        //模块为pub，并不会导致其内容都变为pub，只会允许父模块访问自己。因此需要再添加pub
        pub fn add_to_waitlist(){} 
        fn seat_at_table(){}
    }

    mod serving{
        fn take_order(){}
        fn server_order(){}
        fn take_payment(){}
    }
}

//餐厅后台
fn server_order(){}
mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::server_order(); //通过super关键字从父模块路径开始
    }
    fn cook_order(){}

    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast{
        pub fn summer(toast:&str)->Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }

    //pub的enum的所有成员都是pub的
    pub enum Appetizer{
        Soup,
        Salad,
    }
}

//使用use将模块引入作用域，在use前加pub可以将名称重新导出为pub的
use crate::front_of_house::hosting;

//使用嵌套路径可以减少use语句的数量
use std::{cmp::Ordering, isize};
//使用self引入
use std::io::{self, Write};
//效果类似
//use std::io;
//use std::io::Write;

//glob引入
use std::collections::*;

//as重命名
use std::io::Result as IoResult;


//pub关键字标记公共api
pub fn eat_at_restaurant(){
    //绝对路径：从crate根开始。以crate名或者字面值crate开头
    crate::front_of_house::hosting::add_to_waitlist();
    //相对路径：从当前模块开始，以self,super或当前模块标识符开头
    front_of_house::hosting::add_to_waitlist();

    //使用use引入了hosting
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    //toast是pub字段，因此吣随意读写，而seasonal_fruit字段则不能
    meal.toast=String::from("Wheat"); 
    println!("I'd like {} toast please", meal.toast);

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}
