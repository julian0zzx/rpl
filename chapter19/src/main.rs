
use std::ops::Add;
use std::fmt;

use hellomacro::HelloMacro;
use hellomacro_derive::HelloMacro;

static mut COUNTER: i32 = 0;

fn main() {
    println!("Hello, Advance features!");

    // 19.1
    add_to_count(12);
    // unsafe {
    //     println!("unsafe added: {}", COUNTER);        
    // }

    // 19.2
    test_adv_lftm();
    // 19.3
    test_adv_trait();
    // 19.4
    test_adv_type();
    // 19.5
    test_adv_fp();
    // 19.6
    test_adv_macro();
}

// 19.1
fn add_to_count(inc : i32) {
    unsafe {
        COUNTER += inc;
    }
    unsafe {
        println!("unsafe added: {}", COUNTER);
    }
}

// 19.2
fn test_adv_lftm() {
    // lib.rs
}

// 19.3
fn test_adv_trait() {
    // type Item

    assert_eq!(Point{x: 3, y: 3}, Point{x:1, y: 1} + Point{x:2, y:2});
    // assert_eq!(Millimeters{l: 1500}, Millimeters{l: 500} + Meters{l: 1});
    assert_eq!(Millimeters(1500), Millimeters(500) + Meters(1));

    let person = Human;
    person.fly(); // human
    Human::name();
    Pilot::fly(&person); // pilot
    <Human as Pilot>::name();
    Wizard::fly(&person); // wizard
    <Human as Wizard>::name();

    let pp = Point{x: 1, y: 2};
    println!("{}", pp);

    println!("{}", VecWrapper(vec![String::from("1"), String::from("two"), String::from("hiiii")]));
}

#[derive(Debug)]
struct VecWrapper(Vec<String>);

impl fmt::Display for VecWrapper {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

trait Pilot {
    fn fly(&self);
    fn name();
}

trait Wizard {
    fn fly(&self);
    fn name();
}

#[derive(Debug)]
struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("a pilot is flying");
    }
    fn name() {
        println!("PILOT");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("a wizard is flying");
    }
    fn name() {
        println!("WIZARD");
    }
}

impl Human {
    fn fly(&self) {
        println!("a human is flying");
    }
    fn name() {
        println!("HUMAN");
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x : i32,
    y : i32
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Point; // type
    fn add(self, other : Point) -> Point {
        Point{x : self.x + other.x, y : self.y + other.y}
    }
}

// #[derive(Debug, PartialEq)]
// struct Millimeters {
//     l : u32
// }

// #[derive(Debug, PartialEq)]
// struct Meters {
//     l : u32
// }

// impl Add<Meters> for Millimeters {
//     type Output = Millimeters;

//     fn add(self, other: Meters) -> Millimeters {
//         Millimeters{l : self.l + (other.l * 1000)}
//     }
// }

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// 19.4
fn test_adv_type() {
    type Km = u32;
    let k1 : Km = 10;
    let k2 : u32 = 15;
    println!("{}", k1 + k2);
}

// 19.5
fn test_adv_fp() {
    println!("{}", add2_one2(add_one, 2));

    let vv = vec![1,2,3];
    let vv2 = vv.iter().map(|_v| returns_closure); // not work
    // let vv2 : Vec<_> = vv.iter().map(|v| v + 1).collect();
    println!("{:?}", vv);
    println!("{:?}", vv2);
}

fn returns_closure() -> Box<Fn(i32) -> i32> {
    // Box::new(|x| x + 1)
    Box::new(move |x| {let x2 = x + 1; println!("~~~{}~~~", x2); x2})
}

fn add_one(i : i32) -> i32 {
    i + 1
}

fn add2_one2(f : fn(i32) -> i32, arg : i32) -> i32 {
    f(arg) + f(arg)
}

// 19.6
fn test_adv_macro() {
    pp!(123);
    Oamee::hello_macro();
    // test_macro();
}

#[macro_export]
macro_rules! pp {
    ( $($x:expr) ?) => { // regrex * + ?
        $(
            println!("pp: {}", $x)
        )?; // match ?
    };
}

#[derive(Debug, HelloMacro)]
struct Oamee;

// #[mmlog]
fn test_macro() {
    println!("___test_macro___");
}

// macro_rules! mmlog {
//     ($mmnn:ident) => ({
//         #[proc_macro_attribute]
//         pub fn $name(args: TokenStream, input: TokenStream) -> TokenStream {
//             println!("macro_name: {}", mmnn);
//             TokenStream()
//         }
//     };)
// }

// #[proc_macro_attribute]
// pub fn pplog(attr: TokenStream, item: TokenStream) -> TokenStream {
//     // let ast = syn::parse(attr).unwrap();
//     // let fname = &ast.ident;
//     println!("fn: {:?}", attr);
//     item
// }
