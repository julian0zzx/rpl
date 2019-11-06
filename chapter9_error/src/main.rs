
use std::io;
use std::io::Read;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");

    // panic is some assert

    // RUST_BACKTRACE=1 cargo run
    let v = vec![1,2,3];
    println!("{:#?}", v[2]);

    // panic!("wow");

    // err_result();
    // err_result2();

    // err_unwrap();

    println!("{:?}", err_prop().unwrap());
    println!("{:?}", err_prop2().unwrap());
    println!("{:?}", err_prop3().unwrap());
    println!("{:?}", err_prop4().expect("read failed: "));
}

fn err_result() {
    // let f :u32 = File::open("ate.txt"); // trick, force casting to show the type of f
    let f = File::open("main.rs");
    // let f = File::open("Cargo.toml"); // found
    let a = match f {
        Ok(file) => {
            println!("file: {:?}", file);
            file
        },
        // {} only message, {:?} one line, {:#?} multi-line pretty format
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("ate.txt") {
                Ok(fc) => fc, 
                Err(e) => panic!("{:?}", e),
            }, 
            other_err => panic!("error: {:?}", other_err)
        } // println!("error: {:?}", err)
    };
    println!("{:#?}", a);
}

fn err_result2() {
    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}

fn err_unwrap() {
    let f = File::open("Cargo.toml").unwrap(); // Ok, unwrap to Ok's value
    // let f = File::open("non_exist.txt").unwrap(); // Error, unwrap to panic!
    let f = File::open("non_exist.txt").expect("failed to open non_exist.txt"); // Error, unwrap to panic!
}

fn err_prop() -> Result<String, io::Error> {
    let f = File::open("Cargo.toml");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    let mut s = String::new();
    // return
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

// ? used on a method return a Result, Ok to continue, Err to return Error
fn err_prop2() -> Result<String, io::Error> {
    let mut f = File::open("Cargo.toml")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    
    return Ok(s);
}

fn err_prop3() -> Result<String, io::Error> {
    let mut s = String::new();
    // ? to simply err_prop2
    File::open("Cargo.toml")?.read_to_string(&mut s)?;
    Ok(s) // last state as return, NO semicolon
}

fn err_prop4() -> Result<String, io::Error> {
    fs::read_to_string("Cargo4.toml")
}


