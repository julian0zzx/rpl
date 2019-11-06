
fn main() {
    test_enum();    

    test_option();
 
    // match flow
    test_match();
}

// cases of match
fn test_match() {
    println!("coin values {} cent", Coin::Quarter.value_in_coin());

    let s_5 = Some(5);
    println!("5 + 1 = {:?}", plus_one(s_5).unwrap_or(0));

    if let Some(6) = s_5 {
        println!("wooooooo");
    } else {
        println!("{}", s_5.expect("expect excepted, so bad"));
    }
}

fn plus_one(oi : Option<i32>) -> Option<i32> {
    match oi {
        Some(e) => Some(e + 1), // e represent value of Some
        None => None,
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

impl Coin {
    fn value_in_coin(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => { 
                println!("lucky!");
                25
            },
            _ => 0 // not reachable, cuz all four coin was handled 
       }
    }
}

fn test_enum() {
    let v4k = IPAddrKind::V4;
    let v6k = IPAddrKind::V6;
    println!("{:?}, {:#?}", v4k, v6k);

    let v4_str = IPAddrStr::V4(String::from("127.0.0.1"));
    println!("ip v4: {:?}", v4_str);

    let v4 = IPAddr::V4(127, 0, 0, 1);
    println!("v4: {:?}", v4);

    let m = Message::Move{y: 200, x: 100};
    m.call();
}

// options
fn test_option() {
    let some_int = Some(5);
    let some_str = Some("hello");
    assert_eq!(some_int.is_some(), true);
    assert_eq!(some_int.expect("OMG"), 5);
    println!("value of some_str: {}", some_str.expect("OMG"));
    let some_nn : Option<&str> = None;
    // println!("value of some_str: {}", some_nn.expect("OMG")); // thread 'main' panicked at 'OMG', src/libcore/option.rs:1008:5
    // println!("value of some_str: {}", some_nn.unwrap()); // thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:355:21
    println!("value of some_str: {}", some_nn.unwrap_or("none_of_none")); // none_of_none
    let k = 10;
    println!("unwrap_or_else {} with {}", Some(5).unwrap_or_else(|| 2 * k), 20);
    println!("unwrap_or_else {} with {}", None.unwrap_or_else(|| 2 * k), 20);
}

#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6
}

#[derive(Debug)]
enum IPAddrStr {
    V4(String),
    V6(String)
}

#[derive(Debug)]
enum IPAddr {
    V4(u16, u16, u16, u16),
    V6(String)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Hello: {:?}", self);
    }
}
