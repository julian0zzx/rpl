
use futures::future::*;
use futures::future;
use futures::executor;

use std::thread;
use std::time::Duration;

fn main() {
    let user = executor::block_on(gg());
    // let user = gg();
    println!("xxxxx");
    println!("{:?}", user);

}

async fn gg() -> User {
    let user = get_user().await;
    user
}

fn get_user() -> impl Future <Output = User> {
    async {
        thread::sleep(Duration::from_secs(2));
        let user = User::new(String::from("user_in_get_user"));
        user
    }
}

#[derive(Debug)]
struct User {
    username: String
}

impl User {
    fn new(name : String) -> User {
        User{username : name}
    }
}
