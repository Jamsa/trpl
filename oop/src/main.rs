use oop::{AveragedCollection,Screen,Button,SelectBox,Draw};

fn main() {
    println!("oop example:");
    let mut ac = AveragedCollection::new();
    ac.add(20);
    ac.add(20);
    ac.add(10);
    println!("average of {:?} is {}",ac,ac.average());

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run()
}
