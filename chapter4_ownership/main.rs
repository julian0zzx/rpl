
mod bnr;
mod slc;
/// drop, copy, clone, 

fn main() {
    tt_scope0();
    tt_scope1();
    tt_scope2();

    let s = String::from("helloo");
    tt_ownership0(s); // take s
    // println!("here: {} ?", s); // s is moved

    let s = String::from("world");
    let s1 = take_and_give_back(s);
    println!("take and give back: {}", s1);

    // borrow and reference test
    bnr::brm();
    slc::slc_t();
}

fn tt_scope0() {
    let s = "ss";
    let x = s; // value "s" in stack
    println!("{}", x);
}

fn tt_scope1() {
    let mut s = String::from("hello");
    println!("{}", s);

    s.push_str(", world! ");
    println!("{}", s);

    let a = String::from("aaa");
    let b = a;
    // println!("{}", a); // value (a) used here after moved
    println!("{}", b);
}

fn tt_scope2() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // clone, 

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn tt_ownership0(s : String) {
    println!("holded here: {}", s);
}

fn take_and_give_back(s : String) -> String {
    return s;
}
