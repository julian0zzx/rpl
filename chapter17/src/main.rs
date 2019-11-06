
extern crate chapter17;

fn main() {
    println!("Hello, world!");

    // test_avg();
    // test_trait();
}

// 17.1
fn test_avg() {
    let vvv = vec![1, 2, 3, 4, 5];
    let ac = chapter17::AveragedCollection::new(vvv);
    println!("average is {}", ac.average());
    test_avg();
}

// 17.2
fn test_trait() {
    let screen = chapter17::Screen {
        components : vec![
            Box::new(chapter17::Button{label : String::from("A button")}),
            // Box::new(chapter17::Button{label : String::from("888")})
            Box::new(chapter17::Select{value : String::from("888")})
        ]
    };
    screen.run();
}

