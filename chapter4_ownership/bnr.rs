
pub fn brm() {
    println!("borrow and reference...");

    let s = String::from("hello world");
    let sz = count_with_ref(&s);
    println!("reference instead of borrow: size of [{}] is {}", s, sz);

    // multi_ref();
}

fn count_with_ref(s : &String) -> usize {
    return s.len();
}

// one mutable reference
// or, multi-immutable reference
fn multi_ref() {
    let mut s = String::from("hello");
    let s1 = &mut s;
    // let s2 = &mut s; // E0499, second mutable borrow

    let ss = String::from("world");
    let ss1 = &ss;
    let ss2 = &ss;
}
