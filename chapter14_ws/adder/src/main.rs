use add_one;

fn main() {
    println!("Hello, world!");

    let num = 10;

    assert_eq!(11, add_one::add_one(num));

    println!("Hello, world! {}", add_one::add_one(num));
}
