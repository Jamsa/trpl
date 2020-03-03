//记录消息发送条数的库

//消息发送者trait
pub trait Messenger {
    fn send(&self, msg: &str);
}

//跟踪消息发送次数及最大发送次数
pub struct LimitTracker<'a, T:Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

//跟踪Messenger trait的消息发送
impl<'a, T> LimitTracker<'a, T>
where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: you are over you quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Urgent warning: You've used up over 75% of your quota!");
        }


    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    //Messenger的Mock
    struct MockMessenger {
        //不可变引用会导致impl Messenger 中的send方法无法修改数据
        //sent_messages: Vec<String>,
        //RefCell<T>能够在外部值被认为是不可变的情况下修改内部值
        sent_messages: RefCell<Vec<String>>,
    }

    //Mock的实现
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                //sent_messages: vec![]
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    //实现了Messenger trait
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            //因为self是不可变引用，sent_messages不是RefCell时会报错
            //self.sent_messages.push(String::from(message));
            //borrow_mut得到RefMut智能指针
            self.sent_messages.borrow_mut().push(String::from(message));

            //两次借用mut指针会在运行时panic
            //let mut one_borrow_mut = self.sent_messages.borrow_mut();
            //let mut two_borrow_mut = self.sent_messages.borrow_mut();
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);

        //assert_eq!(mock_messenger.sent_messages.len(), 1);
        //borrow得到Ref智能指针
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    //borrow和borrow_mut分别得到Ref和RefMut智能指针，两者都实现了Deref，所以可以当作常规引用对待
    //RefCell<T>记录当前有多少个活动的Ref<T>和RefMut<T>智能指针。第次调用borrow时RefCell<T>将活动的不可变借用计数加一。
    //当Ref值离开作用域时，不可变借用计数减一。就像编译时借用规则一样，RefCell<T>在任何时候只允许有多个不可变借用或一个可变借用。
}
