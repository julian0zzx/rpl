
use crate::List::{Cons, Nil};
use crate::List2::{Cons2, Nil2};
use crate::List3::{Cons3, Nil3};

use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::RefCell;


fn main() {
    // test_box();
    // test_deref();
    // test_own_box();
    // test_drop();
    // test_rc();
    // test_refcell();
    // test_leak();
    test_weak();
}

// 15.6 
fn test_weak() {
    let leaf = Rc::new(Node{value : 3, 
        parent : RefCell::new(Weak::new()), 
        children : RefCell::new(vec![])});

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    let branch = Rc::new(Node{value : 5, 
        parent : RefCell::new(Weak::new()), 
        children : RefCell::new(vec![Rc::clone(&leaf)])});
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}

#[derive(Debug)]
struct Node {
    value: i32, 
    parent: RefCell<Weak<Node>>, 
    children: RefCell<Vec<Rc<Node>>>
}

// 15.6
fn test_leak() {
    let a = Rc::new(Cons3(5, RefCell::new(Rc::new(Nil3))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons3(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // stack overflow
    // println!("a next item = {:?}", a.tail());
}

// 15.5
fn test_refcell() {
    let mock_msger = MockMsger::new();
    println!("{}", mock_msger.msgs.borrow().len()); // 0
    mock_msger.send("nice!");
    println!("{}", mock_msger.msgs.borrow().len()); // 1
}

#[derive(Debug)]
struct MockMsger {
    msgs: RefCell<Vec<String>>
}
impl MockMsger {
    fn new() -> MockMsger {
        MockMsger{msgs: RefCell::new(vec![])}
    }
}
trait Msger {
    fn send(&self, msg: &str);
}

impl Msger for MockMsger {
    fn send(&self, msg : &str) {
        self.msgs.borrow_mut().push(String::from("hello?"));
    }
}

// 15.4
fn test_rc() {
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("count of a: {}", Rc::strong_count(&a)); // 1
    let b = Cons2(3, Rc::clone(&a));
    let c = Cons2(4, Rc::clone(&a));
    println!("count of a: {}", Rc::strong_count(&a)); // 3
    {
        let d = Cons2(8, Rc::clone(&a));
        println!("count of a, in a local scope: {}", Rc::strong_count(&a)); // 4
    }
    println!("count of a, out of local scope: {}", Rc::strong_count(&a)); // 3

}

// 15.3
fn test_drop() {
    let bd = Box4Drop::new(String::from("hello world~~~~~"));
    bd.ps();
    // drop(bd); // force to call Drop trait, std::mem::drop in prelude
    println!("before here =====");
}

#[derive(Debug)]
struct Box4Drop {
    data: String
}

impl Box4Drop {
    fn new(ss : String) -> Box4Drop { // ::
        Box4Drop{data : ss}
    }

    fn ps(&self) { // function on instance "."
        println!("{}", self.data);
    }
}

impl Drop for Box4Drop {
    fn drop(&mut self) {
        println!("{} ========", self.data);  // invoked, after ps
    }
}

// 15.1
fn test_own_box() {
    let mb = MyBox::new(32);
    assert_eq!(32, *mb); // deref *(&) === *(Box)

    let hl = MyBox::new(String::from("hello world~~~~~"));
    hello(&hl); // compiling phrase, MyBox deref to String, String deref to str, &(*hj)[..]
}

fn hello(ss : &str) {
    println!("{}", ss);
}

#[derive(Debug)]
struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(t : T) -> MyBox<T> {
        MyBox(t)
    }
}


// 15.2 
impl <T> Deref for MyBox<T>  {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn test_deref() {
    let x = 8;
    let y = &x;

    println!("{}", x);
    println!("{}", y);
    println!("{}", *y);
    // assert_eq!(8, x);
    // assert_eq!(8, y); // no implementation for `{integer} == &{integer}`

    let xx = Box::new(7);
    assert_eq!(7, *xx); // deref *(&) === *(Box)
}

// 15.1
fn test_box() {
    let b55 = Box::new(55);
    println!("b55: {:#?}", b55);
    println!("b55: {:#?} copied", *b55 + 1); // *Box

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {:#?}", list);
}
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // indirection Box/Rc/&
    Nil
}

#[derive(Debug)]
enum List2 {
    Cons2(i32, Rc<List2>), // indirection Box/Rc/&
    Nil2
}

#[derive(Debug)]
enum List3 {
    Cons3(i32, RefCell<Rc<List3>>),
    Nil3,
}

impl List3 {
    fn tail(&self) -> Option<&RefCell<Rc<List3>>> {
        match self {
            Cons3(_, item) => Some(item),
            Nil3 => None,
        }
    }
}
